use mlua::{Lua, Result};
use mlua::prelude::{LuaFunction, LuaValue};
use tracing::{info_span, Instrument};

async fn span(_: Lua, (data, lua_function): (LuaValue, LuaFunction)) -> Result<()> {
    lua_function.call_async::<()>(()).instrument(info_span!("lua_span", data = serde_json::to_string(&data).unwrap())).await?;
    Ok(())
}

pub fn inject_module(lua: &Lua) -> Result<()> {
    let m = lua.create_table()?;
    m.set("span", lua.create_async_function(span)?)?;
    lua.globals().set("tracing", m)?;
    Ok(())
}
