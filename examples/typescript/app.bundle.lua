--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]

local ____modules = {}
local ____moduleCache = {}
local ____originalRequire = require
local function require(file, ...)
    if ____moduleCache[file] then
        return ____moduleCache[file].value
    end
    if ____modules[file] then
        local module = ____modules[file]
        local value = nil
        if (select("#", ...) > 0) then value = module(...) else value = module(file) end
        ____moduleCache[file] = { value = value }
        return value
    else
        if ____originalRequire then
            return ____originalRequire(file)
        else
            error("module '" .. file .. "' not found")
        end
    end
end
____modules = {
["app"] = function(...) 
--[[ Generated with https://github.com/TypeScriptToLua/TypeScriptToLua ]]
logger.info("Loading app.lua")
logger.info("Opening database connections")
app_db = sqlite.connect("sqlite://app.sqlite?mode=rwc")
logger.info("Creating tables")
app_db:query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)")
app_db:query("INSERT OR REPLACE INTO users (id, name) VALUES (1, 'Alice')")
cache = lru_cache.create(5)
function index(self)
    local cached_res = cache:get("index")
    if not cached_res then
        local res = app_db:query("SELECT id, name FROM users WHERE id = 1")
        res[2].uuid = uuid.v4()
        local res_str = json.encode(res[2])
        cache:set("index", res_str)
        cached_res = res_str
    end
    return "Hello, World!" .. cached_res
end
function get_todos(self)
    local result = requests.get("https://jsonplaceholder.typicode.com/todos/1")
    local data = json.decode(result)
    data.uuid = uuid.v4()
    return json.encode(data)
end
get(nil, "/", index)
get(nil, "/todos", get_todos)
 end,
}
return require("app", ...)
