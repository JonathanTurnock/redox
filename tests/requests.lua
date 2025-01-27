local m = {}

logger.info("Running tests/requests.lua")

m.tests = {}

table.insert(m.tests, function()
    logger.info("Running test 1...")
    local r = requests.get("https://jsonplaceholder.typicode.com/posts/1")
    assert(r:get_status() == 200)
    assert(r:get_header_value("content-type") == "application/json; charset=utf-8")
    assert(r:get_text() == [[{
  "userId": 1,
  "id": 1,
  "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
  "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
}]])
end)

return m

