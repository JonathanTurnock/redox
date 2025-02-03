use log::info;
use mlua::{IntoLua, Lua, LuaSerdeExt, Result, UserData, UserDataMethods};
use reqwest::header::HeaderMap;
use reqwest::{Client, Response};
use serde_json::{from_str, Value};

struct _Response {
    status: u16,
    headers: HeaderMap,
    body: String,
}

impl _Response {
    async fn new(response: Response) -> Self {
        let status = response.status().as_u16();
        let headers = response.headers().to_owned();
        let body = response.text().await.unwrap();

        Self {
            status,
            headers,
            body,
        }
    }
}

impl UserData for _Response {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_status", |_, this, _: ()| {
            Ok(this.status)
        });

        methods.add_method("get_headers", |lua, this, _: ()| {
            let headers = lua.create_table().unwrap();
            for (key, value) in this.headers.iter() {
                headers.set(key.as_str(), value.to_str().unwrap()).unwrap();
            }
            Ok(headers)
        });

        methods.add_method("get_header_value", |lua: &Lua, this, key: String| {
            let value = this.headers.get(key.as_str()).unwrap().to_str().unwrap();
            Ok(value.into_lua(lua).unwrap())
        });

        methods.add_method("get_text", |_, this, _: ()| {
            Ok(this.body.clone())
        });

        methods.add_method("get_json", |lua, this, _: ()| {
            let serde_value = from_str::<Value>(&this.body).unwrap();
            let lua_value = lua.to_value(&serde_value).unwrap();
            Ok(lua_value)
        });
    }
}

struct _Requests {
    client: Client,
}

impl _Requests {
    fn new() -> Result<_Requests> {
        Ok(Self { client: Client::new() })
    }
}

impl UserData for _Requests {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("get", |lua: Lua, this, url: String| async move {
            info!("GET {}", url);
            let response = this.client.get(&url).send().await.unwrap();
            _Response::new(response).await.into_lua(&lua)
        });

        methods.add_async_method("post", |lua: Lua, this, (url, body): (String, String)| async move {
            info!("POST {}", url);
            let response = this.client.post(&url).body(body).send().await.unwrap();
            _Response::new(response).await.into_lua(&lua)
        });

        methods.add_async_method("put", |lua: Lua, this, (url, body): (String, String)| async move {
            info!("PUT {}", url);
            let response = this.client.put(&url).body(body).send().await.unwrap();
            _Response::new(response).await.into_lua(&lua)
        });

        methods.add_async_method("delete", |lua: Lua, this, url: String| async move {
            info!("DELETE {}", url);
            let response = this.client.delete(&url).send().await.unwrap();
            _Response::new(response).await.into_lua(&lua)
        });
    }
}


pub fn inject_module(lua: &Lua) -> Result<()> {
    let m = lua.create_table()?;
    m.set(
        "client",
        lua.create_async_function(|_lua: Lua, _url: String| async move {
            _Requests::new().map(|requests| _lua.create_userdata(requests))
        })?,
    )?;

    m.set(
        "get",
        lua.create_async_function(|lua: Lua, url: String| async move {
            info!("GET {}", url);
            let response = reqwest::get(&url).await.unwrap();
            _Response::new(response).await.into_lua(&lua)
        })?,
    )?;

    lua.globals().set("requests", m)?;
    Ok(())
}
