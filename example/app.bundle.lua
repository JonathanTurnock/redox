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
get(
    "/",
    function()
        return "Hello World!" .. uuid.v4()
    end
)
get(
    "/todos",
    function()
        local data = requests.get("https://jsonplaceholder.typicode.com/todos/1")
        local d = json.decode(data)
        d.id = uuid.v4()
        return json.encode(d)
    end
)
 end,
}
return require("app", ...)
