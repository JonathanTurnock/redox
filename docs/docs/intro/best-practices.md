---
sidebar_position: 3
---

# Best Practices

- Use Lua for tasks like data manipulation, business logic, and processing.
- For complex or performance-critical functionality, write native code (e.g., in Rust) and expose it as a library that can be imported by Lua.
- In other languages, this is often referred to as a "native module" and they can be prohibitively difficult to author, but being built on this principle, Redox encourages it, while it does provide an abstraction it does not hide the complexity at the cost of the users ability to extend.
- Clear progression from scripting to native code, allowing developers to optimize performance without sacrificing iteration speed.

:::tip
The world of the web seldom uses native code, we reach for general purpose scripting languages, JVM based languages, .NET etc

Today we have more capabilities around building and deploying highly optimized binaries for our target cloud environments than ever before.

We can spin up cloud CI servers, build amazingly performant web applications for example with Rust and ship to easily deployable and updatable targets, but the barrier to entry is very high.

When experimentation and time to market are still very much the key driving factor for choosing a technology, and optimising for performance is often an afterthought **Redox** aims to provide a runtime that is easy to get going but ready with open arms for when/if you need to optimise.
:::