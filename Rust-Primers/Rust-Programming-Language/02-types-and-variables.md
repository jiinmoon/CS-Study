# Types and Varaibles in Rust

## Data Representation

__How is data represented in the digital computer__?

Short answer is in binary format: 0 and 1.

All electronic systems use either analgoue or digital way to represent data.
Analogue systems use current (i.e. 4 mA current in a loop) while digital
systems use pulses to encode data - stream of 0s and 1s.

Single digit 0 or 1 is called a bit - and the collection of bits are stored in
memory such as RAM or other storage devices.

__Bits__

A single bit is either 0 or 1; while it is great in storing "binary"
information such as True or False, it is inadequate in representing data that
requires more space. For example, how would we represent a string such as
"Hello, World!"?

Hence, we collect bits to encode more - binary numbers. For example, with two
bits we can store 4 states (00, 01, 10, 11). The number of states that we can
map to increases exponentially or `2^N`.

A single __byte__ is made up of 8 bits; meaning that it can represents up to
`2^8 = 256` number of states. It makes possible to store all English alphabet
in a single byte. ASCII is one example - and UNICODE dictates on the mapping of
the characters to a single state. For example, we have decided that number 65
(`0100 0001` in binary) would be represented as "A". Hence, whenever we type
character "A" in the computer, it is represented as `0100 0001` in the computer
memory.

When it comes to numbers, we have to consider negative numbers as well. This is
where "signed" notation comes in. If we specify that the number is "signed",
then we reserve a single bit to denote sign (+ or -) of current number. For
example, a signed byte can represent numbers from -2^7 to 2^7 - 1; but if it
is an unsigned byte, then we can represent from 0 to 2^8. Similar idea for
"floating" numbers as well - as we cannot infinitely store all the digits in
the floating numbers, best we can hope for is approximation. And we use
mantissa to represent floating point numbers using bits.

__Bits and Platforms__

When we describe CPU architectures, we hear term 32-bits or 64 bits CPU. This
comes from the size of the word that the CPU is capable of operating in. If the
word size increaess, this means that CPU's range of instructions and range of
memory locations that it can explore is expanded as well. This is the reason
behind why 32-bits CPU can only utilize up to 4 GB RAM memory.

This means that programming languages have their size of number types vary
depending on the platform - most can only gurantee the minimum size of the
types (i.e. `sizeof(int)` in c).

## Core Data Types

```rust
#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let x = 0;
}
```

First two lines are to supress the warning when we compile our codes. If we
comment out the second line and try to compile, we will get following output:

```bash
/Users/francisco.moon/.cargo/bin/cargo build --color=always --message-format=json-diagnostic-rendered-ansi --package example_04 --bin example_04
   Compiling example_04 v0.1.0 (/Users/francisco.moon/Codes/CS-Study/Rust-Primers/Rust-Programming-Language/example_04)
warning: unused variable: `x`
 --> src/main.rs:5:9
  |
5 |     let x = 0;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted
```

Below are some number types:

```rust
fn numeric_types() {
    // let binds variable name to a value
    // note that overflow will be caught by the compiler
    // i.e. let num1: u8 = 256 will cause overflow error
    let num1: u8 = 255;             // unsigned 8 bits (0 to 255)
    // "!" denotes a macro -- more on macros later
    println!("num1 is {}", num1);

    // note that reassignment of variable num1 will throw an error
    // all rust variable are immutable by default;
    // we use "mut" keyword to make mutable variable
    let mut num2: i8 = 0;           // signed 8 bits (-128 to 127)
    println!("Before num2 was {}", num2);
    num2 = 1;
    println!("Now num2 has changed to {}", num2);

    // rust automatically assigns i32 for number if not specified
    let num3 = 123456781;      // i32 ==> 32 bits = 4 byes
    // note that "&" denotes address of; it is a pointer
    // here, "{:p}" is needed since rust focuses on object value rather than object identity
    // hence, &num3 will dereference automatically if we just try to print &num3
    println!("num3 is {}. It is located at {:p} and it takes {} bytes",
             num3, &num3, mem::size_of_val(&num3));

    // all number formats are
    // u8, u16, u32, u64, i8, i16, ...

    // usize, isize
    // automatically deduces natural bits for currently running architecture (32 or 64?)
    let num4: isize = 123;
    let size_of_num4 = mem::size_of_val(&num4);
    println!("num is {}. It takes {} bytes and we are running {} bits OS",
             num4, size_of_num4, size_of_num4 * 8);

    // char
    let ch1: char = 'x';              // 32 bit unicode
    println!("c1 is {} . Its size is {} bytes", ch1, mem::size_of_val(&ch1));

    // floats: f32, f64 are IEEE754 signed
    let fnum1: f32 = 3.14;
    println!("fnum1 is {}. Its size is {} bytes", fnum1, mem::size_of_val(&fnum1));

    // boolean
    let flag: bool = false;               // requires entire byte
    println!("flag is {}. Its size is {} bytes", flag, mem::size_of_val(&flag));
}
```

## Operators

```rust
fn operators() {
    // arithmetic operators
    // usual numeric operations work including +, -, *, /
    // it also supports % modulo operator but does not have power operator (like python **)
    let mut num1 = 2 + 3 - 1 * 10 / 22;
    println!("num1 is {}. Remainder of num1 by 3 is {}", num1, num1 % 3);

    // c like syntactic sugar also works as well such as
    // num1 += 1 or num1 *= 2;

    num1 = i32::pow(num1, 3);
    println!("num1 cubed is {}", num1);

    // note on the computing powers on floats
    // this operation can get expensive as it requires integrals
    let num2 = 1.23;
    let num2_cubed = f64::powi(num2, 3);
    let num2_to_pi = f64::powf(num2, std::f64::consts::PI);
    println!("{} cubed is {} and {} to pi is {}", num2, num2_cubed, num2, num2_to_pi);

    // bit-wise operations are also available
    // supported operations are | or, & and, ^ xor, ! nor
    // as well as left and right shifts (<< and >>)
    let mut num3 = 1 | 2;
    num3 <<= 10;
    println!("{}", num3);

    // common logical operators are >, >=, <, <=, ==
    let is_PI_less_than_4 = std::f64::consts::PI < 4.0;
    let is_PI_equal_to_3 = std::f64::consts::PI == 3.0;
    println!("Is PI less than 4? ===> {}", is_PI_less_than_4);
    println!("Is PI equal to 3? ===> {}", is_PI_equal_to_3);
}
```

## Scope and Shadows

Rust shares much of the same scoping idea with other languages; outside of
curly braces, it would be considered out of scope.

```rust
fn scope_and_shadows() {
    let num1 = 1;
    {
        let num2 = 2;
        // notice how num1 can be accessed without a problem
        println!("This is inside a new scope. num1 is {} and num2 is {}", num1, num2);
        // but how would rust behave when we declare another num1 INSIDE this scope?
        let num1 = 3;
        println!("num1 has changed to {} by re-declaring", num1);                   // this prints 3
    }
    println!("This is outside a scope. num1 is {}", num1);  // this prints 1

    // below line will cause compiler to throw an error
    // it will not be able to find value num2 in current scope
    //println!("This is outside a scope {}", num2);
}
```

## Constants

```rust
// global variable declaration
const PI: f32 = 3.14;
static PI2: f32 = 3.14;
static mut PI3: f32 = 3.14;

fn main() {
    println!(PI);
    unsafe {
      PI3 *= 2.0;
      println!("{}", PI3);
    }
    // cannot access/change PI3 outside of unsafe scope
    // println!("{}", PI3);
}
```

Note that constants do not get fixed address (not saved on any memory) - simply
at compilation time, all constants are simply replaced with the value that they
were declared with.

To give a constant this address, we use the keyword `static` - this would make
the variable visible everywhere. However, declaring with `static mut` would be
an error-prone and memory safety is not guranteed - other threads may change
the value and deadlocks can occur. 

To access or change "static mutable" global variables, you must declare
`unsafe` scope that you are trying to access the variables to acknowledge that
you are accessing and changing the global varaible.

