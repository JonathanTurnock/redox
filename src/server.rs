use crate::lua::get_lua_instance;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::{header, Method};
use actix_web::middleware::{from_fn, Next};
use actix_web::web::Data;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use log::{error, info};
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use mlua::{Function, Lua};
use std::process::exit;
use std::str::FromStr;
use std::sync::{Arc};
use actix_web::rt::spawn;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};
use tokio::time::sleep;
use tracing::{info_span, Instrument};

pub struct AppData {
    pub lua: Arc<Lua>,
    pub prometheus_handle: Arc<PrometheusHandle>,
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
async fn metrics_endpoint(app_data: Data<AppData>) -> impl Responder {
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
                gauge!("process_memory", "pid" => process.pid().to_string())
                    .set(process.memory() as f64);
                gauge!("process_virtual_memory","pid" => process.pid().to_string())
                    .set(process.virtual_memory() as f64);
                gauge!("cpu_usage", "pid" => process.pid().to_string() ).set(process.cpu_usage());
            }
        });
    } else {
        gauge!("process_memory", "pid" => process.pid().to_string()).set(process.memory() as f64);
        gauge!("process_virtual_memory","pid" => process.pid().to_string())
            .set(process.virtual_memory() as f64);
        gauge!("cpu_usage", "pid" => process.pid().to_string() ).set(process.cpu_usage());
    }

    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::TEXT_PLAIN_UTF_8))
        .body(app_data.prometheus_handle.render())
}

pub async fn serve(
    entrypoint: String,
    shutdown_timeout: u64,
    bind_address: String,
    bind_port: u16,
) {
    let prometheus_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to create PrometheusBuilder");

    let prometheus_handle = Arc::new(prometheus_handle);

    info!("Creating Server...");

    HttpServer::new(move || {
        let (lua, handlers) = get_lua_instance(&entrypoint);
        let lua = Arc::new(lua);
        let app_data = Data::new(AppData {
            lua: Arc::clone(&lua),
            prometheus_handle: Arc::clone(&prometheus_handle),
        });

        spawn(async move {
            let start_time = std::time::Instant::now();
            match lua.globals().get::<Function>("main") {
                Ok(main) => match main.call_async::<()>(()).await {
                    Ok(_) => {
                        let duration = start_time.elapsed();
                        println!("Lua environment initialized in {:?}", duration);
                    }
                    Err(err) => {
                        println!("{err}");
                        exit(1);
                    }
                },
                Err(err) => {
                    error!("Error getting main function: {}", err);
                    exit(1);
                }
            }
        }.instrument(info_span!("lua_init")));

        let mut app = App::new()
            .wrap(middleware::Logger::default())
            .wrap(from_fn(http_requests_middleware))
            .app_data(Data::clone(&app_data))
            .service(metrics_endpoint);

        for handler in handlers.read().unwrap().iter() {
            let lua_fn = Arc::new(handler.function.clone());
            let path = handler.path.clone();
            let method = handler.method.clone();
            app = app.route(
                &handler.path,
                web::method(Method::from_str(&handler.method).unwrap()).to(move || {
                    let path = path.clone();
                    let method = method.clone();
                    let lua_fn = Arc::clone(&lua_fn);

                    let span = info_span!(
                        "http_request",
                        path = path.clone(),
                        method = method.clone(),
                        lua_fn = format!("{:?}", lua_fn.info())
                    );

                    async move {
                        match lua_fn.call_async::<String>(()).instrument(span).await {
                            Ok(response) => HttpResponse::Ok().body(response),
                            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                        }
                    }
                }),
            );
        }

        app
    })
    .shutdown_timeout(shutdown_timeout)
    .bind((bind_address, bind_port))
    .expect("Failed to bind server")
    .run()
    .await
    .expect("Failed to run server");
}
