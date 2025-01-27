---
sidebar_position: 6
---

# Types and Typescript

Lua has a variety of super sets which can be used to author Lua code,  many bring with them a type system, and a way to write more robust code.

REDOX is currently using LuaJIT, which is a Just-In-Time Compiler for Lua, and it does not have a type system.

However, the following projects are worth checking out:

- [TypescriptToLua](https://typescripttolua.github.io/) - TypescriptToLua is a tool that allows you to write Lua code in Typescript, it has a type system and is a great way to write more robust Lua code.
- [Fennel](https://fennel-lang.org/) - Fennel is a lisp that compiles to Lua, it has a type system and is a great way to write more robust Lua code.
- [Lua Language Server](https://luals.github.io/) - This project provides a language server for Lua, and can be used with many editors to provide type hints and other features.
- [Teal](https://github.com/teal-language/tl) - Teal is a typed dialect of Lua, it is a great way to write more robust Lua code.
- [Moonscript](https://moonscript.org/) - Moonscript is a language that transpiles to Lua, it has a type system and is a great way to write more robust Lua code.

:::tip
REDOX publishes a `.d.ts` for each API that is exposed to Lua, this can be used in your Typescript projects to provide type hints.

This has the added benefit that if you have a dependency that uses pure Typescript, you can use it in your REDOX project like you would in any JS runtime project.

See the example https://github.com/JonathanTurnock/redox/tree/main/examples/typescript
:::