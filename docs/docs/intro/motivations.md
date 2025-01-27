---
sidebar_position: 2
---

# Motiviations

## Why Lua?

Lua is a very simple language that is easy to learn and has a very low barrier to entry. It is pervasive in the games industry and is used in many high performance applications and embedded environments.

It is also very easy to embed in other languages, and just as easy to create native modules for, which makes it a great choice for the **scripting** component of your application.

## Why Rust?

Rust is a great language for building high performance applications. It has a great type system and is very safe, pulling in your dependencies with Rust means the Lua GC is only working on your "**Scripted**" parts that you have the power to optimise.

Rust has amazing support for building binaries across platforms, and excellent build tools and package management solutions.

## Async Runtime

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
