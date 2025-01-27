## Redox: A Scriptable Runtime for Web Microservices

Redox is a scriptable runtime designed for building web microservices, leveraging Lua as its primary scripting language. It also supports TypeScript through [TypeScriptToLua](https://typescripttolua.github.io/), providing flexibility in development.

### Examples:
- [TypeScript Example](examples/typescript/app.ts)
- [Lua Example](examples/lua/app.lua)

Currently in the research phase, Redox aims to deliver a high-performance, scriptable runtime tailored for web microservices. Its design focuses on using a scripting language (Lua) for frequently changing business logic, while performance-critical tasks are handled natively.

Much of the current impl is built to get a POC running and evaluate benchmarks to see if the concept is viable and well-founded or if it's not actually solving the core problem.

### Core Features:
- **Rust Core:** The runtime is built in Rust, featuring a collection of pre-exposed native components with user-friendly APIs for web service development.
- **Lua Scripting:** Lua is the primary scripting language, offering a simple yet powerful tool for business logic and data manipulation.
- **Battery-Included:** Redox provides a rich set of APIs, sourced from the Rust ecosystem, wrapped in Lua for ease of use.
- **Close to Metal:** For performance-critical tasks, developers can write native code in Rust (Or any other i.e. C, C++, Zig etc) and expose it as a library for Lua.
- **Contributions Welcome:** Redox is open-source and welcomes contributions from the community to build a wide collection of standard APIs and libraries.
- **Async Runtime:** Redox is built on top of the async runtime Tokio, allowing for efficient handling of I/O-bound tasks.

### Contributing factors to proceeding with Redox:
- **Ease of Use:** Redox developers can build a web microservice with actual utility using the standard APIs alone.
- **Extensibility:** Developers can write native code in Rust and expose it as a library for Lua to optimize performance and this can be done with minimal experience in Rust.
- **Adaptability:** Redox allows developers to build a runtime tailored to their needs, selecting only the features they require.
- **Performance:** Redox is able to **at least** match the performance of other scripting runtimes with comparable benchmarks for the same overall functionality.
- **Security:** Redox minimizes the attack surface and binary size by allowing developers to build a runtime with only the features they need.
- **Scalability:** Redox supports both horizontal and vertical scalability, running efficiently on a single-core setup or scaling to multicore systems with multiple workers. Redox is able to allow services to act as an "L1" Cache with logic with performance not being degraded by the volume of data in the cache (GC Overdrive).
- **Community:** Redox is open-source and welcomes contributions from the community to build a wide collection of standard APIs and libraries.

### Encourages good practices:
- Use Lua for tasks like data manipulation, business logic, and processing.
- For complex or performance-critical functionality, write native code (e.g., in Rust) and expose it as a library that can be imported by Lua.
- In other languages, this is often referred to as a "native module" and they can be prohibitively difficult to author, but being built on this principle, Redox encourages it, while it does provide an abstraction it does not hide the complexity at the cost of the users ability to extend. 
- Clear progression from scripting to native code, allowing developers to optimize performance without sacrificing iteration speed.

> The world of the web seldom uses native code, we reach for general purpose scripting languages, JVM based languages, .NET etc
> 
> Today we have more capabilities around building and deploying highly optimized binaries for our target cloud environments than ever before.
> 
> We can spin up cloud CI servers, build amazingly performant web applications for example with Rust and ship to easily deployable and updatable targets, but the barrier to entry is very high.
> 
> When experimentation and time to market are still very much the key driving factor for choosing a technology, and optimising for performance is often an afterthought **Redox** aims to provide a runtime that is easy to get going but ready with open arms for when/if you need to optimise.

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

In the example above, the `requests.get` function is an [async operation](src/requests.rs), but the Lua function is written in a synchronous style.

## The Features System

Redox takes advantage of the Rust feature system to allow you to build a runtime that is tailored to your needs.

**Build with All Features Enabled**  
This includes every available feature, resulting in a fully equipped runtime.
```bash
cargo build --release --all-features  # ~11MB binary
```

**Build with Specific Features**  
Select only the features you need, reducing the binary size.
```bash
cargo build --release --features "sqlite,uuid"  # ~8MB binary
```

**Build Using a Distro Group**  
Use predefined feature sets, such as a web-focused distribution.
```bash
cargo build --release --features distro-web  # ~9MB binary
```

>*Note: The sizes mentioned above are current sizes and only serve as an illustration of the concept.*

## Context

Please see [CONTEXT.md](CONTEXT.md) for more information on the motivation behind Redox and its vision.

## Contributing

Redox is in the early stages of development, and we welcome contributions from the community. If you’re interested in contributing please reach out, raise an issue or submit a PR.

## License

Redox is licensed under the MIT license. See [LICENSE](LICENSE.MD) for more information.