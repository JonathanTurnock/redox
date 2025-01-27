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

use actix_web::http::Method;
use actix_web::{web, App, HttpResponse, HttpServer};
use log::{error, info};
use mlua::{Function, Lua, LuaOptions, StdLib};
use std::path::Path;
use std::{
    env,
    str::FromStr,
    sync::{Arc, RwLock},
    time::Duration,
};

struct Handler {
    method: String,
    path: String,
    function: Function,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let lua = Lua::new_with(StdLib::ALL, LuaOptions::default()).unwrap();

    log4rs::init_file("logger.yml", Default::default()).unwrap();

    uuid::inject_module(&lua).unwrap();
    requests::inject_module(&lua).unwrap();
    logger::inject_module(&lua).unwrap();
    json::inject_module(&lua).unwrap();
    sqlite::inject_module(&lua).unwrap();
    lru_cache::inject_module(&lua).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <command> <lua_file>", args[0]);
        std::process::exit(1);
    }
    let command = args[1].clone();
    let lua_file = args[2].clone();

    let handlers = Arc::new(RwLock::new(Vec::new()));

    lua.globals()
        .set("get", {
            let handlers = Arc::clone(&handlers);
            lua.create_function_mut(move |_, (path, function): (String, Function)| {
                let mut handlers = handlers.write().unwrap();
                handlers.push(Handler {
                    method: "GET".to_string(),
                    path,
                    function,
                });
                Ok(())
            })
                .unwrap()
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

    match lua.load(Path::new(&lua_file)).exec_async().await {
        Ok(_) => (),
        Err(err) => {
            println!("{err}");
        }
    }


    match command {
        command if command == "run" => {
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
            HttpServer::new(move || {
                let mut app = App::new().wrap(actix_web::middleware::Logger::default());

                for handler in handlers.write().unwrap().iter() {
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
