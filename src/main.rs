mod json;
mod logger;
mod lru_cache;
mod requests;
mod sqlite;
mod uuid;

use std::{
    env, fs,
    str::FromStr,
    sync::{Arc, RwLock},
    time::Duration,
};

use actix_web::{
    guard::Method,
    http::Method,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use mlua::{Function, Lua};

struct Handler {
    method: String,
    path: String,
    function: Function,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("logger.yml", Default::default()).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <lua_file>", args[0]);
        std::process::exit(1);
    }
    let lua_file = args[1].clone();

    let lua_code = fs::read_to_string(&lua_file).unwrap();

    let lua = Lua::new();

    uuid::inject_module(&lua).unwrap();
    requests::inject_module(&lua).unwrap();
    logger::inject_module(&lua).unwrap();
    json::inject_module(&lua).unwrap();
    sqlite::inject_module(&lua).unwrap();
    lru_cache::inject_module(&lua).unwrap();

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

    lua.load(lua_code.to_string()).exec_async().await.unwrap();

    HttpServer::new(move || {
        let mut app = App::new();
        // .wrap(actix_web::middleware::Logger::default());

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
