# Rust Programming Language

Notes on lectures found on `https://www.udemy.com/course/rust-lang/`.

---

## What is Rust?

- It is a relatively new language which was developed by Mozilla Corporation.

- It aims to be lower level language that rivals with C++ and C.

- Introduces interesting concepts such as strict memory safety.

## Installing Rust

```
https://www.rust-lang.org/tools/install
```

Automatically sets-up the env changes.

## "Hello, Rust!"

Rust source files have `.rs` suffix. Let's create a simple `hello.rs` file:

```rust
fn main()
{
    println!("Hello, Rust!");
}
```

The compiler for Rust is called `rustc` and invoked as follows:

`$ rustc hello.rs`

This generates compiled binary named `hello` which upon execution, prints the
"Hello, Rust!" message to the terminal.

## Managing Packages with `cargo`

Just like `pip` for Python, Rust has a package manager called `cargo`.

As projects grow larger, we need more packages and dependencies which becomes
harder and harder to manage by hand.

By default, `cargo` is shipped with Rust and is available.

To use `cargo`, it requires us to set up our directory structure.

The source codes, `.rs` files, are placed in the `/src` directory within the
project directory.

Also, the entry point of the program is named `main.rs` (i.e. how `__main__.py`
is like for Python I suppose?).

In the root of the project directory, we now create `Cargo.toml` which is
a simple format that configures the project.

```toml
[package]
name = "hello_world"
version = "0.0.1"
authors = ["Francisco Moon <jiinmoon@tutanota.com>"]    
```

Now, instead of using `rustc` to compile our source code manually, we use
`cargo` to build our project for us and packages it nicely. To do so, we run
following:

`cargo build`

This command builds the project and creates `Cargo.lock` file and `/target`
directory which contains all the files at various stages (i.e. `.d` file, and
so on).

To execute:

`cargo run`

Notice that `cargo run` can be used without having to run `cargo build` - it
will simply build and run the program.

---

This gives us an overview of how we should structure our Rust project. In fact,
`cargo` allows us to quickly set up our project and lay down the foundation for
us:

`cargo new <project_name> --bin`

This will create a new directory with given `<project_name>` and populates with
`Cargo.toml` as well as `/src` directory which contains `main.rs` within.

---

## Rust with Intellij Idea

Just a side note: if wish to follow this along with an IDE, IntelliJ Idea has
a Rust plug-in which is currently being updated.

---


