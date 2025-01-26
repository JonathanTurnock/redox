use mlua::{Lua, Result, Table, UserData, UserDataMethods, Value};
use sqlx::{Column, Row, SqlitePool};

struct _SqliteConnection {
    pool: SqlitePool,
}

impl _SqliteConnection {
    async fn new(database_url: &str) -> Result<_SqliteConnection> {
        let pool = SqlitePool::connect(database_url).await.unwrap();
        Ok(Self { pool })
    }
}

impl UserData for _SqliteConnection {
    fn add_methods<'lua, M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("query", |lua: Lua, this, query: String| async move {
            let rows = sqlx::query(&query)
                .fetch_all(&this.pool)
                .await
                .map_err(mlua::Error::external)?;

            let mut lua_rows: Vec<Table> = Vec::new();

            for row in rows {
                let lua_row = lua.create_table().unwrap();

                for (idx, column) in row.columns().iter().enumerate() {
                    let column_name = column.name();
                    let value: Value = match row.try_get_unchecked::<Option<String>, _>(idx) {
                        Ok(Some(val)) => Value::String(lua.create_string(&val)?),
                        Ok(None) => Value::Nil,
                        Err(_) => match row.try_get_unchecked::<Option<i64>, _>(idx) {
                            Ok(Some(val)) => Value::Integer(val),
                            Ok(None) => Value::Nil,
                            Err(_) => match row.try_get_unchecked::<Option<f64>, _>(idx) {
                                Ok(Some(val)) => Value::Number(val),
                                Ok(None) => Value::Nil,
                                Err(_) => Value::Nil, // Fallback for unsupported types
                            },
                        },
                    };
                    lua_row.set(column_name, value)?;
                }

                lua_rows.push(lua_row);
            }

            Ok(lua_rows)
        });
    }
}

pub fn inject_module(lua: &Lua) -> Result<()> {
    let m = lua.create_table()?;
    m.set(
        "connect",
        lua.create_async_function(|lua: Lua, url: String| async move {
            _SqliteConnection::new(&url)
                .await
                .map(|conn| lua.create_userdata(conn))
                .map_err(mlua::Error::external)
        })?,
    )?;

    lua.globals().set("sqlite", m)?;
    Ok(())
}
