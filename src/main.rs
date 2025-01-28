#[cfg(feature = "json")]
mod json;
#[cfg(not(feature = "json"))]
mod json {
    pub fn inject_module(_: &mlua::Lua) -> Result<(), mlua::Error> {
        Ok(())
    }
}

#[cfg(feature = "logger")]
mod logger;
#[cfg(not(feature = "logger"))]
mod logger {
    pub fn inject_module(_: &mlua::Lua) -> Result<(), mlua::Error> {
        Ok(())
    }
}

#[cfg(feature = "lru_cache")]
mod lru_cache;
#[cfg(not(feature = "lru_cache"))]
mod lru_cache {
    pub fn inject_module(_: &mlua::Lua) -> Result<(), mlua::Error> {
        Ok(())
    }
}

#[cfg(feature = "requests")]
mod requests;
#[cfg(not(feature = "requests"))]
mod requests {
    pub fn inject_module(_: &mlua::Lua) -> Result<(), mlua::Error> {
        Ok(())
    }
}

#[cfg(feature = "uuid")]
mod uuid;
#[cfg(not(feature = "uuid"))]
mod uuid {
    pub fn inject_module(_: &mlua::Lua) -> Result<(), mlua::Error> {
        Ok(())
    }
}

#[cfg(feature = "sqlite")]
mod sqlite;

#[cfg(not(feature = "sqlite"))]
mod sqlite {
    pub fn inject_module(_: &mlua::Lua) -> Result<(), mlua::Error> {
        Ok(())
    }
}

use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::{header, Method};
use actix_web::middleware::{from_fn, Next};
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use log::{error, info};
use metrics::{counter, describe_gauge, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use mlua::{Function, Lua, LuaOptions, StdLib};
use std::path::Path;
use std::{env, process, str::FromStr, sync::{Arc, RwLock}, time::Duration};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use tokio::task_local;
use tokio::time::sleep;

struct AppData {
    lua: Lua,
    prometheus_handle: PrometheusHandle,
}

#[derive(Clone)]
struct Handler {
    method: String,
    path: String,
    function: Function,
}

async fn http_requests_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    counter!("http_requests_total").increment(1);
    let start = std::time::Instant::now();

    match next.call(req).await {
        Ok(response) => {
            let duration = start.elapsed().as_secs_f64();
            counter!("http_requests_total", "status" => response.status().clone().to_string(), "path" => response.request().path().to_string(), "method" => response.request().method().to_string()).increment(1);
            histogram!("http_request_duration_seconds", "status" => response.status().clone().to_string(), "path" => response.request().path().to_string(), "method" => response.request().method().to_string()).record(duration);
            Ok(response)
        }
        Err(err) => Err(err),
    }
}

#[get("/metrics")]
async fn metrics_endpoint(app_data: web::Data<AppData>) -> impl Responder {
    let mut sys = System::new_all();

    gauge!("lua_used_memory").set(app_data.lua.used_memory() as f64);
    gauge!("used_memory").set(sys.used_memory() as f64);
    gauge!("total_memory").set(sys.total_memory() as f64);
    gauge!("total_swap").set(sys.total_swap() as f64);
    gauge!("used_swap").set(sys.used_swap() as f64);
    gauge!("cpus").set(sys.cpus().len() as f64);
    gauge!("global_cpu_usage").set(sys.global_cpu_usage());

    // Wait a bit because CPU usage is based on diff.
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
    // Refresh CPU usage to get actual value.
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_cpu(),
    );

    let process = sys.process(sysinfo::get_current_pid().unwrap()).unwrap();

    if let Some(tasks) = process.tasks() {
        tasks.iter().for_each(|pid| {
            if let Some(process) = sys.process(*pid) {
                gauge!("process_memory", "pid" => process.pid().to_string()).set(process.memory() as f64);
                gauge!("process_virtual_memory","pid" => process.pid().to_string()).set(process.virtual_memory() as f64);
                gauge!("cpu_usage", "pid" => process.pid().to_string() ).set(process.cpu_usage());
            }
        });
    } else {
        gauge!("process_memory", "pid" => process.pid().to_string()).set(process.memory() as f64);
        gauge!("process_virtual_memory","pid" => process.pid().to_string()).set(process.virtual_memory() as f64);
        gauge!("cpu_usage", "pid" => process.pid().to_string() ).set(process.cpu_usage());
    }

    HttpResponse::Ok().insert_header(header::ContentType(mime::TEXT_PLAIN_UTF_8)).body(app_data.prometheus_handle.render())
}

fn get_lua_instance(entry: &String, handlers: &mut Vec<Handler>) -> Lua {
    let lua = Lua::new_with(StdLib::ALL, LuaOptions::default()).unwrap();

    uuid::inject_module(&lua).unwrap();
    requests::inject_module(&lua).unwrap();
    logger::inject_module(&lua).unwrap();
    json::inject_module(&lua).unwrap();
    sqlite::inject_module(&lua).unwrap();
    lru_cache::inject_module(&lua).unwrap();


    lua.globals()
        .set("get", {
            lua.create_function_mut(move |_, (path, function): (String, Function)| {
                handlers.push(Handler {
                    method: "GET".to_string(),
                    path,
                    function,
                });
                Ok(())
            })
                .unwrap();
        })
        .unwrap();

    lua.globals()
        .set(
            "sleep",
            lua.create_async_function(move |_lua, n: u64| async move {
                tokio::time::sleep(Duration::from_millis(n)).await;
                Ok(())
            })
                .unwrap(),
        )
        .unwrap();


    let compiler = mlua::Compiler::new().set_optimization_level(1).set_debug_level(1).set_coverage_level(0);
    lua.set_compiler(compiler);

    match lua.load(Path::new(entry)).exec() {
        Ok(_) => (),
        Err(err) => {
            println!("{err}");
        }
    }

    lua
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("logger.yml", Default::default()).unwrap();


    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <command> <lua_file>", args[0]);
        std::process::exit(1);
    }
    let command = args[1].clone();
    let lua_file = args[2].clone();

    match command {
        command if command == "run" => {
            let lua = get_lua_instance(&lua_file, &mut Vec::new());
            info!("Running Lua script: {}", lua_file);
            info!("Running LUA global main function...");
            match lua.globals().get::<Function>("main") {
                Ok(main) => {
                    match main.call_async::<()>(()).await {
                        Ok(_) => Ok(()),
                        Err(err) => {
                            println!("{err}");
                            std::process::exit(1);
                        }
                    }
                }
                Err(err) => {
                    error!("Error calling main function: {}", err);
                    std::process::exit(1);
                }
            }
        }
        command if command == "serve" => {
            let prometheus_handle = PrometheusBuilder::new().install_recorder().unwrap();

            HttpServer::new(move || {
                let mut handlers = Vec::new();
                let lua = get_lua_instance(&lua_file, &mut handlers);
                let app_data = web::Data::new(AppData {
                    lua,
                    prometheus_handle: prometheus_handle.clone(),
                });
                let mut app = App::new().wrap(middleware::Logger::default()).wrap(from_fn(http_requests_middleware)).app_data(app_data.clone()).service(metrics_endpoint).service(metrics_endpoint);

                for handler in handlers.iter() {
                    let lua_fn = Arc::new(handler.function.clone());
                    app = app.route(
                        &handler.path,
                        web::method(Method::from_str(&handler.method).unwrap()).to(move || {
                            let lua_fn = Arc::clone(&lua_fn);
                            async move {
                                match lua_fn.call_async::<String>(()).await {
                                    Ok(response) => HttpResponse::Ok().body(response),
                                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                                }
                            }
                        }),
                    );
                }

                app
            })
                .shutdown_timeout(5)
                .bind(("0.0.0.0", 8080))?
                .run()
                .await
        }
        _ => {
            println!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}
