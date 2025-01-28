logger.info("Loading app.lua")

logger.info("Opening database connections")
--local app_db = sqlite.connect("sqlite://app.sqlite?mode=rwc");

local app_db

local function index()

    if not app_db then
        app_db = sqlite.connect("sqlite::memory:");

        logger.info("Creating tables")
        app_db:query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)")
        app_db:query("INSERT OR REPLACE INTO users (id, name) VALUES (1, 'Alice')")
    end

    local res = app_db:query("SELECT id, name FROM users WHERE id = 1")
    res[1].uuid = uuid.v4()
    local res = json.encode(res[1])
    return "Hello, World!" .. res
end

local function get_todos()
    local result = requests.get("https://jsonplaceholder.typicode.com/todos/1")
    local data = json.decode(result)
    data.id = uuid.v4()

    return json.encode(data)
end

get("/", index)
get("/todos", get_todos)
