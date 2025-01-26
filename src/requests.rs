use log::{debug, info};
use mlua::{Lua, Result};

async fn get(_lua: &Lua, url: String) -> Result<String> {
    info!("GET {}", url);
    let resp = reqwest::get(&url).await.unwrap();

    info!("Status: {}", resp.status());
    let body = resp.text().await.unwrap();

    debug!("<< {:?}", body);
    Ok(body)
}

async fn post(_lua: &Lua, url: String, body: String) -> Result<String> {
    info!("POST {}", url);
    let client = reqwest::Client::new();
    let resp = client.post(&url).body(body).send().await.unwrap();

    info!("Status: {}", resp.status());
    let body = resp.text().await.unwrap();

    debug!("<< {:?}", body);
    Ok(body)
}

async fn put(_lua: &Lua, url: String, body: String) -> Result<String> {
    info!("PUT {}", url);
    let client = reqwest::Client::new();
    let resp = client.put(&url).body(body).send().await.unwrap();

    info!("Status: {}", resp.status());
    let body = resp.text().await.unwrap();

    debug!("<< {:?}", body);
    Ok(body)
}

async fn delete(_lua: &Lua, url: String) -> Result<String> {
    info!("DELETE {}", url);
    let client = reqwest::Client::new();
    let resp = client.delete(&url).send().await.unwrap();

    info!("Status: {}", resp.status());
    let body = resp.text().await.unwrap();

    debug!("<< {:?}", body);
    Ok(body)
}

async fn patch(_lua: &Lua, url: String, body: String) -> Result<String> {
    info!("PATCH {}", url);
    let client = reqwest::Client::new();
    let resp = client.patch(&url).body(body).send().await.unwrap();

    info!("Status: {}", resp.status());
    let body = resp.text().await.unwrap();

    debug!("<< {:?}", body);
    Ok(body)
}

async fn head(_lua: &Lua, url: String) -> Result<String> {
    info!("HEAD {}", url);
    let client = reqwest::Client::new();
    let resp = client.head(&url).send().await.unwrap();

    info!("Status: {}", resp.status());
    let body = resp.text().await.unwrap();

    debug!("<< {:?}", body);
    Ok(body)
}

pub fn inject_module(lua: &Lua) -> Result<()> {
    let m = lua.create_table()?;
    m.set(
        "get",
        lua.create_async_function(|_lua: Lua, url: String| async move { get(&_lua, url).await })?,
    )?;

    m.set(
        "post",
        lua.create_async_function(|_lua: Lua, (url, body): (String, String)| async move {
            post(&_lua, url, body).await
        })?,
    )?;

    m.set(
        "put",
        lua.create_async_function(|_lua: Lua, (url, body): (String, String)| async move {
            put(&_lua, url, body).await
        })?,
    )?;

    m.set(
        "delete",
        lua.create_async_function(
            |_lua: Lua, url: String| async move { delete(&_lua, url).await },
        )?,
    )?;

    m.set(
        "patch",
        lua.create_async_function(|_lua: Lua, (url, body): (String, String)| async move {
            patch(&_lua, url, body).await
        })?,
    )?;

    m.set(
        "head",
        lua.create_async_function(|_lua: Lua, url: String| async move { head(&_lua, url).await })?,
    )?;

    lua.globals().set("requests", m)?;
    Ok(())
}
