use mlua::{Lua, Result};
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "uuid.v4")]
fn v4(_: &Lua, (): ()) -> Result<String> {
    let uuid = Uuid::new_v4();
    Ok(uuid.to_string())
}

pub fn inject_module(lua: &Lua) -> Result<()> {
    let m = lua.create_table()?;
    m.set("v4", lua.create_function(v4)?)?;

    lua.globals().set("uuid", m)?;
    Ok(())
}
