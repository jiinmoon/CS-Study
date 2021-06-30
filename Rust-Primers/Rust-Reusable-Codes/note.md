# Hey Rust

First Rust program:

```rust
fn main() {
	hey_rust("Francisco")
}

fn hey_rust(name: &str) {
	println!("Hey, {}!", name)
}
```

# Basics of Rust

## Bindings and Mutability

Rust is similar to C or JS in syntax; and like many langauges, it has variables but calls it **bindings**.

```rust
let var1 = "value1";
let var2: &str = "value2";
```

**Note that bindings are immutable by default**.

This is to enforce strict memory/type safety. For instance, following will not compile:

```rust
let var1 = "value1";
// compile error
// cannot assign twice to immutable variable
var1 = "new value";
```

Immutable objects imply an aspect of functional language; but Rust also supports **mutable binding** as well.

```rust
let mut i = 0;
i += 1;
i += 100;
```

Rust also prevents you from using *uninitialized variables*.

```rust
let mut var;
do_something(var);
```

## Built-In Types

Rust has a **strong, static type** system.

Strong means that once variable type is set, it is not to be changed (unless with defined casting methods or `<type x> as <type z>` keywords).

Static means the all variable types are known to the compiler at compilation time.

Let's talk about some built-in primitive types. There are:

- booleans
- integers
- floats
- characters
- arrays
- slices
- typles
- functions (functions are first-class citizen in Rust)

```rust
// compiler is able to infer the type that is resolved from rhs expr
let x = true;
let y: bool = false;
```

Rust char is UTF-8; it supports emojis 😝!

Rust has unsigned/signed integer types from size 8 to 64.

As for floats, there are single-precision and double-precision floating point numbers.

```rust
let signed_int1: i16 = 132;
let unsigned_int1: u8 = 9;

// mininum & maximum ranges are provided
let max_i32_val = i32::max_value();
let min_i32_val = i32::min_value();

// you can use system defined sizes
// note that usize is expected for array indexing
let x: isize = 1;
let y: usize = 2;

let single_float: f32 = 3.14;
let double_float: f64 = 0.009e-22;
```

Rust supports homogeneous array (single type) and heterogeneous tuples (many types). They can be accessed via indexing or dot notations.

```rust
let arr: [i32; 3] = [ 1, 2, 3 ];
let tuple: (i32, &str, u8) = (42, "is meaning of life", 9);

println!("arr[0] = {} \t arr.0 = {}", arr[0], arr.0);
```

## Imports and Namespaces

Rust code is organized into **crates** which consist of **modules**.

Suppose that we have our own program; it can borrow external crates such as `rand` crate (lib). And within our program, we can have several modules such as `file_handler` and so on.

```rust
use std::collections::LinkedList;

fn main()
{
	let mut nums = LinkedList::new();
	for v in [1, 2, 3] {
		nums.push_back(v);
	}

	println!("{:?}", nums);
	
	for v in nums.iter_mut() {
	    *v *= 2;
	}
	
	println!("{:?}", nums);
}
```

To create a new module, use `mod` keyword:

```rust
// module does not have to be in the same file
// under the same directory, we can have greeter.rs
mod greeter {
	// note the pub keyword;
	// without pub, function is private by default
	pub fn say_hello(name: &str) {
		println!("Hello! {} 🤠", name);
	}
}

fn main() {
	// notice how this will give an error;
	// this is a scoping error; compiler cannot find the mod
	// say_hello();

	// so instead, we need to import it into the scope
	use greeter::say_hello;
	say_hello("John");
	// or use it directly
	greeter::say_hello("Mike");
}
```

## Standard Library

Rust standard library offers...

- Types and operations on types
- Memory management
- Filesystem and I/O functions
- Basic networking
- Environment manipulation
- Multiprocessing support
- Collections

### Memory Management

Rust provides both *owned* and *reference-counted* memory management; and more advanced methods.

Most of the time, Rust is capable of managing the memory by itself.

```rust
// Read/Write on heap; used in one place
// think of malloc(...) in C
// automatically dropped once no ref pointer is pointing at it
let boxed = Box::new("some data");

// Read-Only for any number of users
let ref_counted = Rc::new("some data");
```

### Networking and I/O

Rust standard library provides synchronous I/O only.

It has support for both file-based I/O and network I/O (including user input from the console).

### Environment Manipulation

Rust standard library provides access to the environment - a way for program to check its surroundings. This includes environment variables, arguments, paths, and so on.

```rust
fn main () 
{
	use std::env;

	let vars = env::vars();

	for (k, v) in vars 
	{
		println!("Key: {} | Value: {}", k, v);
	}
}
```

### Multi-Threading

Rust standard library provides only thread-based mutli-processing.

External libraries are available and they can provide *promise-based* async processing and *green thread* M to N concurrency.

### Collections

- Sequences (`Vec`, `LinkedList`)
- Deque (`VecDeque`)
- Maps (`HashMap`, `BtreeMap`)
- Sets (`HashSet`, `BtreeSet`)
- Heaps (`BinaryHeap`)

### Rust Standard Library Philosophy

Only quality APIs make it into the standard library; hence, some of useful functionalities that you may have expected from the standard library is found elsewhere. Some useful ones are...

- `extern crate rand`
- `extern crate tokio`
- `extern crate chrono`
- `extern crate serde`
- `extern crate rocket`

---

## Simple Impl of Recursive Fibonacci

```rust
fn fib_recur(n: u64) -> u64
{
	match n {
		0 | 1 => 1,
		_ => { fib_recur(n - 1) + fib_recur(n - 2) }
	}
}

fn main()
{
	let x = 40;
	println!("Fib of {} is {}", x, fib_recur(x));
}
```

## Better Impl of Fibonacci

Recursive implementation has a serious flaw in that it duplicates work too much - creating exponential number of recursive calls. If we can store/cache the results, we can solve this much faster.

(Alternatively, we could just implemented iteratively)

```rust
use std::collections::HashMap;

fn fib_better(n: u64, map: &mut HashMap<u64, u64>) -> u64
{
	match map.contains_key(&n) {
	    false => {
	        match n {
	            0 | 1 => 1,
	            _ => { 
	                let result = fib_better(n - 1, map) + fib_better(n - 2, map);
	                map.insert(n, result);
	                result
	            }
	        }
	    },
	    
	    true => *map.get(&n).unwrap()
	}
}

fn main()
{
	let x = 40;
	println!("Fib of {} is {}", x, fib_better(x, &mut HashMap::new()));
}
```

---

## `rustup` and `cargo`

Rust has different release channels: Stable, Beta, Nightly.

### `rustup`

`rustup` is used to manage our toolchain.

```bash
$ rustup install <toolchain>
$ rustup default <toolchain>
```

It is used to manage which version of the toolchain to run (i.e. stable-aarch64-apple-darwin). If you need to run on specific version you can do so by `$ rustup run stable-x.xx <test commands>`.

### `cargo`

`cargo` is a project/dependency management tool.

We can easily start a new Rust project using `cargo` as follows:

`$ cargo new <new-project>`

This creates directory structure necessary for the project at hand. All the source `.rs` should go under `/src`.

`Cargo.toml` specifies the extern dependencies and cargos needed - as well as project description/version.

To compile and execute:

`$ cargo run`

### Crate Ecosystem

Rust divides code into crates; there is a central repository of publicly available crates, `crates.io`.

Crate version number follows a format:

```
a.b.cd
major.minor.patch

major: potentially breaking changes
minor: new APIs
patch: fixes
```

i.e. crate version `1.2.89` refers to major version 1; minor version 2; patch 89.

To use a pub crate, we edit the `Cargo.toml`; suppose that we want to use `rand` crate:

```toml
...

[dependencies]
rand = "0.5"
```

And we import to our program as follows:

```rust
extern crate rand;

use rand::random();

...
```

### Crate Docs

`https://docs.rs/<crate-name>`

You can generate official doc-string (much like Java docs) via `///` notation.

```rust
/// this is a docstring
/// everything written here
/// will be made available as docstring
/// supports .md format
///
/// # Example usage
///
/// ```rust
/// fn main() {
///		some_function();
///	}
/// ```
fn some_function() {
	println!("Hello!");
}
```

The docstrings are generated with `cargo` command:

`cargo doc --open`

which opens the docstring in `html` format.

And generated docstring is made available under project directory `/<project-name>/target/doc`.
