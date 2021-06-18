# Rustlang Quick Crash Course

## What is this?

Introduction of Rust focused on fundamentals and syntax.

## What is Rust?

It is a system level language that competes with C, C++ and GO which focuses
more on lower level instead of applications (user facing softwares); so, Rust
is used mostly for compilers, drivers and tools.

One notable feature of Rust is that it does not have GC (Garbage Collection)
and manual memory management. For example, Java Script will every so often
looks for variables to clean and to free up the memory - those that do not have
reference anymore. Depending on the program, this can take some time and
intrude on the performance of your program. In the case of C and C++, this has
to be managed by the programmer and can be error-prone and responsible for many
memory leaks.

In Rust, you do not have to manage the memory yourself nor does an automatic
check every so often to perform garbage collection. Rust performs the memory
management on demand - once the threshold is reached, then it would start to
look for variables to free up the memory.

Rust has its own package manager called, `cargo` - just like `pip` for Python or
`npm` for NodeJS. It manages and tracks dependencies.

Other notable tools are `rustup` to manage versions and to perform language
updates and `rustc` is the compiler.

## Getting Started

There are several IDEs available integrated with Rust such as IntelliJ IDEA
(with Rust plugin) or VS Code.

Simplest way to compile any rust code is using the `rustc` tool to manually
compile each `.rs` code files into executable. Suppose we have a `hello.rs`
with following lines of code:

```rust
fn main()
{
    println!("Hello");
}
```

Every program requires a point of entry and Rust is not any different - that is
the purpose of the `main` function.

`println!` is a macro, identifiable by its `!`. Compiler will automatically
expand on set macros wherever it sees one.

We can compile and execute as follows:

```
$ rustc hello.rs
$ ./hello
```

---

However, typical workflow in Rust on getting started with a new project would
be to use `cargo`. 

To start a new project, `cargo new <new-project-name>` or within the new-project directory, `cargo init`.

This creates a new `<new-proejct-name>` directory with `Cargo.toml` file and
`\src` directory. `Cargo.toml` is used to manage the dependencies and house
keeping details for `cargo` to properly manage the project. Under the `\src`
directory, 

The project can be compiled and run using `cargo run` command, which builds the `\target` directory to house the debugging information alongside the binary executables. To simply complie, `cargo build` command is sufficient.

---

To link the multiple `.rs` files and use the code from each other, `mod` is used. For example, suppose that under `\src` directory we have `print.rs` file with following code:

```rust
// print.rs
fn say_hello() {
    println!("This is Hello! from print.rs");
}
```

Then, we can invoke it from another file, say `main.rs` as follows:

```rust
// main.rs

// two ways to import the print.rs
// 1. using `mod` keyword
// think of it as Python's `import`
mod print

// 2. using `use` keyword
// think of it as Python's `from _ import _ as _`
use print::say_hello as hello;

fn main() {
    // invoke using first method
    print::say_hello();
    // invoke using second method
    hello();
}
```

## Print Formatting

We cannot for example do following:

```rust
fn say_hello() {
    println!(42);
}
```

`println!` macro argument is expected to be supplied in formatted string such as follows:

```rust
fn say_hello() 
{
    // Basic Printing Format
    println!("{} is the meaning of {}.", 42, "life");

    // Positional Arguements
    println!("{0} {1} {0} {1}", "first", "second");

    // Named Arguments
    println!("{first_name} {last_name}", "John", "Smith");
}
```

## Variables (Assignments)

```rust
/* Variables
 * 
 * Variables hold primitive data or "refereces" to data.
 * In Rust, variables are "immutable" by default.
 * 
 * Note that Rust is block scoped and block is defined by "{...}".
 */
pub fn run() 
{
    let name = "John";
    let age = 29;
    // following will be an error: cannot assign to immutable var
    //age = 30;
    println!("{} is {} years old.", name, age);

    // "mut" keyword makes the variable as mutable
    let mut mutable_age = 30;
    mutable_age = 99;
    println!("{} is {} years old.", name, mutable_age);

    // we may also define constant variables
    // compiler will simply replace all const vars with that value on compile time
    // note that you have to specify type for the const
    const PI: f32 = 3.14;
    println!("PI = {}", PI);

    // multiple assignment in single line
    let (dog_name, dog_age) = ("Luke", 12);
    println!("{} is a good dog and is {} years old.", dog_name, dog_age);
}
```

## Data Types

__refer to `types.rs`__.

## String

__refer to `string.rs`__.

## Typical Collections: Tuples, Arrays, Vectors

__refer to `array.rs`__.

## Conditionals

__refer to `conditionals.rs`__.

Also, it is possible to evaluate statements such as `let` or `match` as well -
which will return bool depending on the outcome.

## Looping

__refer to `loops.rs`__.

## Functions

__refer to `functions.rs`__.

## Pointers & References

__refer to `pointer_ref.rs`__.

## Structs

__refer to `structs.rs`__.

## Enums

__refer to `enums.rs`__.

## Cli

__refer to `cli.rs`__.