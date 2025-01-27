local function index()
    return json.encode({id = 1, name = "Alice", uuid = uuid.v4()})
end

get("/", index)