---
sidebar_position: 4
---

# Async

Async programming can be really hard to get right, but with Redox, you don't have to worry about this. 

Redox is built on top of the async runtime [Tokio](https://tokio.rs/), allowing for efficient handling of I/O-bound tasks. This enables Redox to handle a large number of concurrent connections with minimal resource usage.

When Redox invokes a Lua function that uses an async operation, Redox handles the async transparently, allowing the Lua function to be written in a synchronous style.

```lua
local function get_todos()
    local result = requests.get("https://jsonplaceholder.typicode.com/todos/1")
    local data = json.decode(result)
    data.id = uuid.v4()

    return json.encode(data)
end
```

In the example above, the `requests.get` function is an async operation, but the Lua function is written in a synchronous style.