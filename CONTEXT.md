## Why would I not just use Node.js, Deno, Bun, Python, etc?

First, let’s acknowledge that there’s absolutely nothing wrong with these runtimes—they’re excellent runtimes with specific strengths. For instance, Deno and Bun are advancing the use of more native code, as demonstrated by Bun’s [SQLite integration](https://bun.sh/docs/api/sqlite).

However, Redox is not designed to compete directly with these runtimes. Instead, its goal is to offer a high-level, batteries-included Lua runtime. Unlike general-purpose runtimes like Node.js, Deno, Bun, or Python, which have a minimal core and broad applicability.

> Redox is a collection of Lua bindings to native Rust libraries, configurable at compile time, with pre-built distributions for common use cases.

Rust boasts an incredible ecosystem. Redox aims to leverage Rust’s power without requiring developers to learn Rust upfront. By exposing a broad ecosystem directly to Lua, Redox allows you to write your business logic in Lua while taking full advantage of Rust’s performance and reliability.

## Flipping the 80/20

The aim of Redox is to flip the 80/20 rule on its head. This is a gross oversimplification, but serves as an illustration of the point.

In a Web Application using a general purpose scripting runtime, the vast majority of the runtime is a stable set of low level APIs.

The community adds a vast array of libraries on top of this runtime, and if your lucky you can catch some Native Modules in that i.e. `better-sqlite3`.

| Component      | General Purpose Scripting Runtime | Redox |
|----------------|-----------------------------------|-------|
| Business Logic | 20%                               | 20%   |
| Libraries      | 60%                               | 0%    |
| Runtime        | 20%                               | 80%   |

But the point is, the vast majority of your app is not your code, it's often lacking in performance, and you have no control over it.

The classic example for this in the world of Node.js is the `express` library. It's the defacto standard for Node.js web applications, but a basic synthetic benchmark shows that even in the Node.js world, there are other options that are magnitudes faster.

Synthetic benchmarks are useless, yes your application will suffer other bottlenecks, yes, but this all adds up, all taxing the scripting runtime, when you have nothing more than a CRUD api.

## Vision Statement

Redox is a high-performance, scriptable runtime designed for developing web microservices. It prioritizes ease of use, extensibility, and the ability to adapt to specific development needs.

Developers can choose from pre-built Redox distributions, such as web-api-pgsql or aws, or customize their own runtime by selecting only the features they require. This flexibility is particularly beneficial in performance-critical, embedded, or security-conscious environments, where tailored binaries minimize the attack surface and binary size.

With Redox, developers can write business logic in Lua/Typescript for simplicity and rapid iteration, while reserving complex or performance-intensive tasks for lower-level languages like Rust, Zig, C, or C++. These can be seamlessly integrated into Lua through generic APIs, fostering clear distinctions between disposable business logic and critical code.

Redox supports both horizontal and vertical scalability, running efficiently on a single-core setup or scaling to multi-core systems with multiple workers. The Rust core provides an extensive collection of APIs sourced from the Rust ecosystem on crates.io, wrapped in Lua for intuitive and straightforward use.

This combination of performance, scalability, and extensibility ensures that Redox empowers developers to build robust and efficient web microservices.

> Note: Clustering is the spawning of multiple runtime instances, Redox uses a single instance with multiple workers, which is a different approach to scaling and allows much more efficient memory sharing.
