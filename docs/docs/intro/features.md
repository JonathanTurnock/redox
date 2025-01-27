---
sidebar_position: 5
---

# Features

In the context of this document a faeture is a rust module that has lua bindings and is available to be used in your application.

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

## Distros

A distro is a predefined set of features that are commonly used together. This allows you to select a specific distro and get a runtime that is tailored to that use case.

**Build Using a Distro Group**  
Use predefined feature sets, such as a web-focused distribution.
```bash
cargo build --release --features distro-web  # ~9MB binary
```

>*Note: The sizes mentioned above are current sizes and only serve as an illustration of the concept.*