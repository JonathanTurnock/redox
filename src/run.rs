use crate::lua::get_lua_instance;
use log::{debug, error, info};
use mlua::Function;
use std::time::Instant;
use tracing::instrument;

#[instrument(name = "run")]
pub async fn run(entrypoint: &String) {
    let (lua, _) = get_lua_instance(entrypoint);

    // Start time measurement
    let start_time = Instant::now();

    info!("Running Lua script: {}", &entrypoint);
    info!("Running LUA global main function...");
    match lua.globals().get::<Function>("main") {
        Ok(main) => match main.call_async::<()>(()).await {
            Ok(_) => {
                debug!("Lua script executed successfully");

                let duration = start_time.elapsed();
                println!("Lua script execution time: {:?}", duration);

                std::process::exit(0);
            }
            Err(err) => {
                println!("{err}");
                std::process::exit(1);
            }
        },
        Err(err) => {
            error!("Error calling main function: {}", err);
            std::process::exit(1);
        }
    }
}
