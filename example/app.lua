logger.info("Loading app.lua")

logger.info("Opening database connections")
local app_db = sqlite.connect("sqlite://app.sqlite?mode=rwc");

logger.info("Creating tables")
app_db:query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)")
app_db:query("INSERT OR REPLACE INTO users (id, name) VALUES (1, 'Alice')")

local cache = lru_cache.create(5)
-- local cache = {}

local function index()
    local cached_res = cache:get("index")
    -- local cached_res = cache["index"]

    if cached_res == nil then
        local res = app_db:query("SELECT id, name FROM users WHERE id = 1")
        res[1].uuid = uuid.v4()
        local res = json.encode(res[1])
        cache:set("index", res)
        -- cache["index"] = res
        cached_res = res
    end

    return "Hello, World!" .. cached_res
end

local function get_todos()
    local result = requests.get("https://jsonplaceholder.typicode.com/todos/1")
    local data = json.decode(result)
    data.id = uuid.v4()

    return json.encode(data)
end

get("/", index)
get("/todos", get_todos)
