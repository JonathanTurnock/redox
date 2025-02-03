use log::debug;
use mlua::Table;
use crate::lua::get_lua_instance;

pub async fn run(entrypoint: &String) {
    debug!("Running Lua script: {}", &entrypoint);
    let (lua, _) = get_lua_instance(entrypoint);

    let testtable = lua.globals().get::<Table>("tests").expect("Could not get tests table");

    for pair in testtable.pairs::<String, mlua::Function>() {
        let (name, callback) = pair.expect("Could not get pair");
        debug!("Running test: {}", name);
        match callback.call_async::<()>(()).await {
            Ok(_) => {
                println!("✓ Test '{}' passed", name);
            }
            Err(err) => {
                println!("✗ Test '{}' failed: {}", name, err);
            }
        }
    }

    debug!("Running LUA test functions...");
}