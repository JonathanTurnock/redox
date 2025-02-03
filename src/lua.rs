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

#[cfg(feature = "test")]
mod test;

#[cfg(not(feature = "test"))]
mod test {
    pub fn inject_module(_: &mlua::Lua) -> Result<(), mlua::Error> {
        Ok(())
    }
}

#[cfg(feature = "tracing")]
mod tracing;

#[cfg(not(feature = "tracing"))]
mod tracing {
    pub fn inject_module(_: &mlua::Lua) -> Result<(), mlua::Error> {
        Ok(())
    }
}


use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use log::{debug, info};
use mlua::{Function, Lua};

#[derive(Clone)]
pub struct Handler {
    pub method: String,
    pub path: String,
    pub function: Function,
}

pub fn get_lua_instance(entry: &String) -> (Lua, Arc<RwLock<Vec<Handler>>>) {
    info!("Creating Lua instance...");
    // let lua = Lua::new_with(StdLib::ALL, LuaOptions::default()).unwrap();
    let lua = Lua::new();

    let handlers = Arc::new(RwLock::new(Vec::new()));

    lua.globals().set("__FILE__", entry.clone()).unwrap();

    uuid::inject_module(&lua).unwrap();
    requests::inject_module(&lua).unwrap();
    logger::inject_module(&lua).unwrap();
    json::inject_module(&lua).unwrap();
    sqlite::inject_module(&lua).unwrap();
    lru_cache::inject_module(&lua).unwrap();
    test::inject_module(&lua).unwrap();
    tracing::inject_module(&lua).unwrap();

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

    debug!("Loading Lua script: {}", entry);
    match lua.load(Path::new(entry)).exec() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }

    (lua, handlers)
}