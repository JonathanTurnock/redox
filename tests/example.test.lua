test.test("Should fetch data from json placeholder", function()
    local response = requests.get("https://jsonplaceholder.typicode.com/posts/1")
    test.assert_eq(response:get_status(), 200)

    local res = response:get_json()
    test.assert_eq(res.userId, 1)
    test.assert_eq(res.id, 1)
    test.assert_eq(res.title, "sunt aut facere repellat provident occaecati excepturi optio reprehenderit")
end)

test.test("UUID should be of type string", function()
    local uuid = uuid.v4()
    test.assert_eq(type(uuid), "string")
end)