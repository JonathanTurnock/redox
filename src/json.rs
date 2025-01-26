use log::debug;
use mlua::prelude::LuaValue;
use mlua::{Lua, LuaSerdeExt, Result};
use serde_json::from_str;
use serde_json::Value;

pub fn inject_module(lua: &Lua) -> Result<()> {
    let m = lua.create_table()?;
    m.set("encode", lua.create_function(encode)?)?;
    m.set("decode", lua.create_function(decode)?)?;

    lua.globals().set("json", m)?;
    Ok(())
}

fn decode(lua: &Lua, string: String) -> Result<LuaValue> {
    debug!("Decoding JSON: {:?}", string);

    let serde_value = from_str::<Value>(&string).unwrap();
    let lua_value = lua.to_value(&serde_value).unwrap();

    debug!("Decoded JSON: {:?}", lua_value);
    Ok(lua_value)
}

fn encode(lua: &Lua, lua_value: LuaValue) -> Result<String> {
    debug!("Encoding Lua value: {:?}", lua_value);

    let json_string = serde_json::to_string(&lua_value).unwrap();
    let lua_string = lua.create_string(&json_string).unwrap();

    debug!("Encoded JSON: {:?}", lua_string);
    Ok(String::from(json_string))
}
