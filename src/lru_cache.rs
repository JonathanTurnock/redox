use lru::LruCache;
use mlua::{Lua, Result, UserData, UserDataMethods, Value};
use std::num::NonZero;

struct _LruCache {
    cache: LruCache<String, Value>,
}

impl _LruCache {
    fn new(size: &usize) -> Result<_LruCache> {
        let cache = LruCache::new(NonZero::new(*size).unwrap());
        Ok(Self { cache })
    }
}

impl UserData for _LruCache {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("get", |_lua, this, key: String| {
            match this.cache.get(&key) {
                Some(value) => Ok(value.clone()),
                None => Ok(Value::Nil),
            }
        });

        methods.add_method_mut("set", |_lua, this, (key, value): (String, Value)| {
            this.cache.put(key, value);
            Ok(())
        });
    }
}

pub fn inject_module(lua: &Lua) -> Result<()> {
    let m = lua.create_table()?;
    m.set(
        "create",
        lua.create_function(|_lua: &Lua, size: usize| _LruCache::new(&size))?,
    )?;

    lua.globals().set("lru_cache", m)?;
    Ok(())
}
