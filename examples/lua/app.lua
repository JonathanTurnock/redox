logger.info("Loading app.lua")
--local app_db = sqlite.connect("sqlite://app.sqlite?mode=rwc");

local app_db

get("/", function()
    local res = app_db:query("SELECT id, name FROM users WHERE id = 1")
    res[1].uuid = uuid.v4()

    tracing.span("Hello", function()
        logger.info("Hello!")

        tracing.span({name = "World"}, function()
            logger.info("World!")
        end)
    end)

    json.encode(res[1])
    return "OK"
end)

get("/todos", function()
    local result = requests.get("https://jsonplaceholder.typicode.com/todos/1")
    local data = json.decode(result)
    data.id = uuid.v4()

    return json.encode(data)
end)

get("/hello", function()
    return json.encode({ message = "Hello, World!" })
end)

local function init_db()
    logger.info("Initializing database")
    app_db = sqlite.connect("sqlite::memory:");

    logger.info("Creating tables")
    app_db:query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)")
    app_db:query("INSERT OR REPLACE INTO users (id, name) VALUES (1, 'Alice')")
end

function main()
    logger.info("Main Called...")
    init_db()
    logger.info("Main Completed...")
end