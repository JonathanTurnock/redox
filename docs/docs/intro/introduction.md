---
sidebar_position: 1
---

# Introduction

REDOX is a web service runtime built on top of the async runtime [Tokio](https://tokio.rs/), allowing for efficient handling of I/O-bound tasks. This enables Redox to handle a large number of concurrent connections with minimal resource usage.

### Core Features:
- **Battery-Included:** Redox provides a rich set of APIs, sourced from the Rust ecosystem, wrapped in Lua for ease of use.
- **Rust Core:** The runtime is built in Rust, featuring a collection of pre-exposed native components with user-friendly APIs for web service development.
- **Lua Scripting:** Lua is the scripting language, offering a simple yet powerful tool for business logic and data manipulation.
- **Close to Metal:** For performance-critical tasks, developers can write native code in Rust (Or any other i.e. C, C++, Zig etc) and expose it as a library for Lua.
- **Contributions Welcome:** Redox is open-source and welcomes contributions from the community to build a wide collection of standard APIs and libraries.
- **Async Runtime:** Redox is built on top of the async runtime Tokio, allowing for efficient handling of I/O-bound tasks.

### Contributing factors to proceeding with Redox:
- **Ease of Use:** Redox developers can build a web microservice with actual utility using the standard APIs alone.
- **Extensibility:** Developers can write native code in Rust and expose it as a library for Lua to optimize performance and this can be done with minimal experience in Rust.
- **Performance:** Redox is able to **at least** match the performance of other scripting runtimes with comparable benchmarks for the same overall functionality.
- **Security:** Redox minimizes the attack surface and binary size by allowing developers to build a runtime with only the features they need.
- **Scalability:** Redox supports both horizontal and vertical scalability, running efficiently on a single-core setup or scaling to multicore systems with multiple workers. Redox is able to allow services to act as an "L1" Cache with logic with performance not being degraded by the volume of data in the cache (GC Overdrive).
- **Community:** Redox is open-source and welcomes contributions from the community to build a wide collection of standard APIs and libraries.
