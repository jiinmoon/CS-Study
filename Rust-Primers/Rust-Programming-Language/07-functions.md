# Functions

## `fn`

```rust
fn print_value(x: i32) {
    println!("Passed in value is {}", x);
}

// Rust is pass by value
// to modify the actual value, we must pass the reference
// then, dereference to modify the value stored within
fn increment_by_one(x: &mut i32) { *x += 1; }

fn product(x: i32, y: i32) -> i32 {
    return x * y;
}

fn functions() {
    print_value(22);

    let mut x = 0;
    println!("Before x was {}", x);
    increment_by_one(&mut x);
    println!("After x is {}", x);

    let mut y = 10;
    let mut z = 11;
    let result = product(y, z);
    println!("{} * {} = {}", y, z, result);
}
```

## Methods

Methods are way to invoke/implement functions for `structs`.

```rust
use std::num;

struct Point {
    x: i32,
    y: i32
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        return (
            ((self.start.x - self.end.x) as f64).powf(2.0) +
            ((self.start.y - self.end.y) as f64).powf(2.0)
        ).sqrt();
    }
}

// how to invoke function within a struct?
fn methods() {
    let p1 = Point { x: -10, y: 0 };
    let p2 = Point { x: 1, y: 10 };
    let line1 = Line { start: p1, end: p2 };

    println!("Length of line is {}.", line1.len());
}
```

## Closures

```rust
// functions are first class citizens
fn closures() {
    let add_one = increment_by_one;
    let mut x = 41;
    add_one(&mut x);
    println!("x = {}", x);

    // closure (anonymous function)
    let plus_one = |x:i32| -> i32 { x + 1 };
    // note: original x is unchanged
    println!("{} + 1 = {}", x, plus_one(x));

    // don't always have to specify types; compiler can mostly figure out for us
    let plus_two = |x: &mut i32| { *x += 2 };
    plus_two(&mut x);
    println!("{}", x)
}
```

## Higher Order Functions

Functions that can take/return functions.

```rust
fn higher_functions() {
    // sum of all even squares < 1000
    let mut result = 0;

    let is_in_range = check_range(500);

    for i in 0.. {
        let isq = i * i;
        if !is_in_range(isq) { break };
        if is_even(isq) { result += isq };
    }
    println!("Result: {}", result);


    let result2 : i32 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < 500)
        .filter(|x:&i32| is_even(*x))
        .sum();
    println!("Result : {}", result2);
}
```
