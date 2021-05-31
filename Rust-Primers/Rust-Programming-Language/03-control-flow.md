# Control Flow

## `if` Statements

```rust
fn if_statement() {
    let num = 21;

    if num % 3 == 0 && num % 5 == 0 {
        println!("{} is Fizz Buzz", num);
    } else if num % 3 == 0 {
        println!("{} is Fizz", num);
    } else {
        println!("{} if Buzz", num);
    }
}
```

## `while` and `for` loops

While rust looping structures share much of the similarities with other
programming languages, a main difference can be seen in `for` loop.

```rust
fn loops() {
    let mut x = 1;
    while x < 100 {
        x += 1;
    }
    println!("{}", x);

    // same as "while true"
    let mut y = 1;
    loop {
        y += 1;
        if y == 100 { break; }
    }

    for i in 1..11 {
        if i == 3 { continue; }
        if i == 9 { break; }
        print!("{} ", i);
    }

    // like python enumerate on iterable
    for (i, x) in (9..15).enumerate() {
        println!("{} - {}", i, x);
    }
}
```

## `match` statement

Note that compiler will check for whether all the cases has been covered for
variable that we are trying to match here; for example, `code` is `i32` and
such compiler will look to see whether all values have been covered. In this
case, `_` will covers all. 

```rust
fn match_statement() {
    // match statement is similar to case-switch statement from other languages
    let code = 960;

    let country = match code {
        960 => "Palestine",
        // = sign in range indicates inclusive
        1..=1000 => "Not yet specified",
        _ => "invalid"
    };

    println!("Code {} is {}", code, country);
}
```

## combination locks example

```rust
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

//use rand::Rng;
use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

fn main() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                println!("Guess a four digit number: ");
                let mut input = String::new();
                // note that below does NOT work
                //let mut input = "";
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Failed");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("Your guess is correct!");
                break;
            }
        }
    }
}
```

Notice how we handle `read_line()` using `match` statement. Read up more on the
`read_line()` and example that rust provides from `std::io::stdin`.


