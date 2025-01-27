logger.info("Loading app.lua")

logger.info("Opening database connections")
const app_db = sqlite.connect("sqlite://app.sqlite?mode=rwc");

logger.info("Creating tables")
app_db.query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)")
app_db.query("INSERT OR REPLACE INTO users (id, name) VALUES (1, 'Alice')")

const cache = lru_cache.create(5)
// const cache = {}

type User = {id: number, name: string, uuid?: string}

function index() {
    let cached_res: string = cache.get("index")
    // const cached_res = cache["index"]

    if (!cached_res) {
        const res = app_db.query<User>("SELECT id, name FROM users WHERE id = 1")

        res[1].uuid = uuid.v4()
        const res_str = json.encode(res[1])
        cache.set("index", res_str)
        //cache["index"] = res_str
        cached_res = res_str
    }

    return `Hello, World!${cached_res}`
}

function get_todos() {
    const result = requests.get<string>("https://jsonplaceholder.typicode.com/todos/1")
    const data = json.decode<User>(result)
    data.uuid = uuid.v4()

    return json.encode(data)
}

get("/", index)
get("/todos", get_todos)
