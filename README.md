# LLM APIs with Rust

This repository contains LLM APIs with Rust language.

# Method

This repo is heavily inspired by [kalosm](https://crates.io/crates/kalosm).

1. Create a new project:
```bash
cargo llm_apis
cd ./llm_apis
```

2. Add `Kalosm` as a dependencies
```bash
cargo add kalosm
cargo add tokio --features full
cargo add tiktoken-rs = "0.5.9"
```

3. Run `main.rs` as below.
  - For Windows

```bash
LIBCLANG_PATH=/path/to/clang+llvm-XX.X.XX-x86_64-pc-windows-msvc/lib cargo run --release
```