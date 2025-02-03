---Global table for SQLite operations.
lru_cache = {}

---SQLite connection object.
---@class SQLiteConnection
local LruCache = {}
LruCache.__index = LruCache

---Connect to a SQLite database.
---@param size integer The URI for the database.
---@return SQLiteConnection The SQLite connection.
function lru_cache.create(size)
    -- This function will be implemented in Rust and will return a SQLiteConnection object.
end

---Get a value from the cache.
-- @generic V
-- @param key string The key to get.
-- @return V The value associated with the key.
function LruCache:get(key)
    -- This function will be implemented in Rust and will execute the query on the database.
end

--- Set a key-value pair in the cache.
-- @generic V
---@param key string The key to set.
---@param value string The value to set.
---@return nil
function LruCache:set(key, value)
    -- This function will be implemented in Rust and will execute the query on the database.
end
