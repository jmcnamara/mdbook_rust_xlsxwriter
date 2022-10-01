# Getting started

Rust_xlsxwriter is a library and doesn't need to be installed. All that is
required is to add it the `Cargo.toml` file for your project. To demonstrate we
will start with a small sample application.

## Create a sample application

Create a new rust command-line application as follows:

```bash
cargo new hello-xlsx
```

This will create a directory like the following:

```bash
hello-xlsx/
├── Cargo.toml
└── src
    └── main.rs
```

Edit the Cargo.toml file and add the following `rust_xlsxwriter` dependency
so the file looks like below. Note, `rust_xlsxwriter` adds new features
regularly do make sure you use the latest version.


```toml
[package]
name = "hello-xlsx"
version = "0.1.0"
edition = "2021"

[dependencies]
rust_xlsxwriter = "0.2.1"
```

Modify the main.rs file so it looks like this:

```rust
{{#include ../../rust_xlsxwriter/examples/app_hello_world.rs:7:}}
```

The run the application as follows:

```bash
cargo run
```

This will create an output file called `hello.xlsx` which should look
something like this:

![Image of hello world Excel output](images/hello.png)
