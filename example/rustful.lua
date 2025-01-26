---Global table for JSON operations.
json = {}

---Encode data into a JSON string.
---@generic T
---@param data T The data to encode.
---@return string The JSON string.
function json.encode(data) end

---Decode a JSON string into data.
---@generic T
---@param data string The JSON string to decode.
---@return T The decoded data.
function json.decode(data) end
---Global table for logging.
logger = {}

---Log a debug message.
---@param message string The debug message.
function logger.debug(message) end

---Log an info message.
---@param message string The info message.
function logger.info(message) end

---Log a warning message.
---@param message string The warning message.
function logger.warn(message) end

---Log an error message.
---@param message string The error message.
function logger.error(message) end
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
---Global table for HTTP requests.
requests = {}

---Perform a GET request to a URL.
---@param url string The URL to request.
---@return string The response from the server.
function requests.get(url) end

---Perform a POST request to a URL.
---@param url string The URL to request.
---@param body string The body of the request.
---@return string The response from the server.
function requests.post(url, body) end

---Perform a PUT request to a URL.
---@param url string The URL to request.
---@param body string The body of the request.
---@return string The response from the server.
function requests.put(url, body) end

---Perform a DELETE request to a URL.
---@param url string The URL to request.
---@return string The response from the server.
function requests.delete(url) end

---Perform a PATCH request to a URL.
---@param url string The URL to request.
---@param body string The body of the request.
---@return string The response from the server.
function requests.patch(url, body) end

---Perform a HEAD request to a URL.
---@param url string The URL to request.
---@return string The response from the server.
function requests.head(url) end
---Global table for LRU Cache operations.
sqlite = {}

---LRU Cache object.
---@class SQLiteConnection
local SQLiteConnection = {}
SQLiteConnection.__index = SQLiteConnection

---Connect to a SQLite database.
---@param uri string The URI for the database.
---@return SQLiteConnection The SQLite connection.
function sqlite.connect(uri)
    -- This function will be implemented in Rust and will return a SQLiteConnection object.
end

---Execute a query on the SQLite database.
---@param query string The SQL query to execute.
---@return table The result set as a list of rows, where each row is a table with column names as keys.
function SQLiteConnection:query(query)
    -- This function will be implemented in Rust and will execute the query on the database.
end
---Global table for UUID operations.
uuid = {}

---Generate a version 4 UUID.
---@return string
function uuid.v4() end
