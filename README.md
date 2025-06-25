# Rust FFI Example: Safe C Interoperability

For readers: Googles Gemini Code Assist was utilized in some parts of this code. I have done my best to vet the code, but I am still learning about the subject myself so it might contain something unexpected, dangerous, stupid or all of the above. Use your own judgement.

This repository demonstrates a robust and idiomatic way to interact with C code from Rust using the Foreign Function Interface (FFI). It showcases best practices for creating a safe, ergonomic Rust API on top of an unsafe C library.

## Project Overview

The core of this project is a simple C function that generates a "Hello, [Name]" greeting. The primary goal is to expose this C functionality to Rust in a safe and user-friendly manner.

The project is structured as a Cargo workspace containing two main crates:

1.  **`hello-sys`**: This is the low-level "sys" crate. Its responsibilities include:
    *   Compiling the C source code (`src/hello.c`).
    *   Generating raw Rust bindings to the C functions using `bindgen`.
    *   Linking the compiled C library into the Rust build.
    *   It handles the `unsafe` aspects of FFI directly.
2.  **`hello-rs`**: This is the high-level, idiomatic Rust crate. It provides:
    *   A safe, user-friendly Rust API (`hello-rs::hello`) that wraps the raw, `unsafe` bindings from `hello-sys`.
    *   Proper error handling and type conversions (e.g., `Option<&str>` to C-style strings, `Result<String>` for return values).
    *   Comprehensive unit tests to ensure the wrapper behaves as expected.

Additionally, the repository includes:

*   **`src/hello.c` / `src/hello.h`**: The actual C source and header files for the greeting function.
*   **`src/main.c`**: A simple C executable to demonstrate the C function's usage directly.

## Key Design Choices

*   **Caller-Provides-Buffer Pattern**: The C `hello` function is designed to receive an output buffer from the caller. This is a crucial pattern for C APIs, as it makes the function re-entrant and thread-safe, avoiding static buffers and memory management issues within the C layer.
*   **Safe Rust Wrapper**: The `hello-rs` crate takes the raw, `unsafe` C function and wraps it in a safe Rust API. This means users of `hello-rs` do not need to write any `unsafe` code themselves to interact with the C library.
*   **Fuzzing**: The `hello-rs` crate includes a fuzzing target to stress-test the Rust-C boundary with malformed and unexpected inputs, helping to uncover potential vulnerabilities or crashes.

## Prerequisites

Before you begin, ensure you have the following installed:

*   **Rust and Cargo**: Install via `rustup` (recommended): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
*   **Nightly Rust Toolchain**: Required for fuzzing: `rustup toolchain install nightly`
*   **`cargo-fuzz`**: The Rust fuzzing tool: `cargo install cargo-fuzz`
*   **C Compiler**: A C compiler (like GCC or Clang) is required to compile the C source code.

## Building and Running

In addition to following shorthands a set of Makefile targets are provided to automate them. See the Makefiles for more information.

### 1. Build the Entire Workspace

From the `./hello-rs` folder run:

```bash
cd hello-rs
cargo build
```
This will compile both `hello-sys` (as a depedency) and `hello-rs`.

### 2. Run the C Executable

The C `main.c` file is compiled separately. You can build and run it from the `src` directory:

```bash
cd src
gcc main.c hello.c -o main_c_app
./main_c_app
```

### 3. Run Rust Tests

To run the unit tests for the `hello-rs` crate:

```bash
cd hello-rs
cargo test
```

## Using the `hello-rs` Crate

You can integrate `hello-rs` into your own Rust project by adding it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
hello-rs = { path = "path/to/this/repo/hello-rs" } # Adjust path as needed
```

Then, you can use the `hello` function:

```rust
use hello_rs::hello;

fn main() -> anyhow::Result<()> {
    let greeting = hello(Some("Rust"))?;
    println!("{}", greeting); // Output: Hello, Rust

    let default_greeting = hello(None)?;
    println!("{}", default_greeting); // Output: Hello, World

    let empty_greeting = hello(Some(""))?;
    println!("{}", empty_greeting); // Output: Hello, World

    Ok(())
}
```

## Fuzzing

The `hello-rs` crate includes a fuzzing target to test the robustness of the `hello` wrapper function.

To run the fuzzer:

```bash
cd hello-rs
cargo +nightly fuzz run fuzz_hello
```

The fuzzer will run indefinitely, generating random inputs and feeding them to the `hello` function. If it finds an input that causes a panic, crash, or other undefined behavior, it will save that input to a file in `fuzz/artifacts/fuzz_hello/` for later analysis and debugging.

Press `Ctrl+C` to stop the fuzzer.
