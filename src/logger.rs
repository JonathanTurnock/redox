use mlua::{Lua, Result};

fn debug(_: &Lua, (string): (String)) -> Result<()> {
    log::debug!("{}", string);

    Ok(())
}

fn info(_: &Lua, (string): (String)) -> Result<()> {
    log::info!("{}", string);

    Ok(())
}

fn warn(_: &Lua, (string): (String)) -> Result<()> {
    log::warn!("{}", string);

    Ok(())
}

fn error(_: &Lua, (string): (String)) -> Result<()> {
    log::error!("{}", string);

    Ok(())
}

pub fn inject_module(lua: &Lua) -> Result<()> {
    let m = lua.create_table()?;
    m.set("debug", lua.create_function(debug)?)?;
    m.set("info", lua.create_function(info)?)?;
    m.set("warn", lua.create_function(warn)?)?;
    m.set("error", lua.create_function(error)?)?;

    lua.globals().set("logger", m)?;
    Ok(())
}
