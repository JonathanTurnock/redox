use log::{debug, info};
use mlua::prelude::{LuaFunction, LuaValue};
use mlua::{Error, Lua, Result};

pub fn inject_module(lua: &Lua) -> Result<()> {
    let m = lua.create_table()?;

    m.set("assert_eq", lua.create_function(assert_eq)?)?;
    m.set(
        "test",
        lua.create_async_function(|lua: Lua, (name, callback): (String, LuaFunction)| async move {
            info!("Registering Callback: {}", name);
            
            let testtable = lua.globals().get("tests").unwrap_or(lua.create_table().unwrap());
            
            testtable.set(name, callback).unwrap();
            
            lua.globals().set("tests", testtable).unwrap();
            
            Ok(())
        })?,
    )?;
    lua.globals().set("test", m)?;
    Ok(())
}

fn assert_eq(_lua: &Lua, (a, b): (LuaValue, LuaValue)) -> Result<LuaValue> {
    let equal = a == b;

    debug!("Asserting equality: {:?} == {:?}, {:?}", a, b, equal);

    if equal {
        Ok(LuaValue::Boolean(true))
    } else {
        Err(Error::RuntimeError("assertion failed".to_string()))
    }
}
