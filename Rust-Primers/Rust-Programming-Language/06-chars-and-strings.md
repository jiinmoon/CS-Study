# Characters and Strings in Rust

## Strings

```rust
fn strings() {
    // there are two types of string in rust

    // one is &'static str
    let s1 = "Hello, Rust!";
    let s2:&'static str = "Hello, Rust!!!";

    // &str is a "string slice" of utf-8
    // this is glued; it is immutable
    for c in s1.chars().rev() {
        println!("{}", c);
    }

    // accessing a character from &str is not a guaranteed operations
    // we need to check its returned value which is Option<char>
    if let Some(first_char) = s1.chars().nth(0) {
        println!("First char is {}", first_char);
    }


    // there is another String type which is a construct
    // this is flexible and mutable
    // in fact, it is allocated in heap and guarantees UTF-8
    let mut alphabet = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        alphabet.push(a as char);
        a += 1;
    }

    println!("{}", alphabet);

    let mut alphabet2 = String::new();
    for c in 'a'..='z' {
        alphabet2.push(c);
    }

    println!("{}", alphabet2);

    // to convert between &str and String
    let x:&str = &alphabet;

    // concatenation
    // String + str
    //let y = alphabet + "!";
    let z = alphabet + &alphabet2;

    let mut abc = String::from("abc");
    let mut abc2 = "abc".to_string();
    abc.remove(0);
    abc.push_str("!");
    println!("{}", abc.replace("bc", "def"));
}
```

## String Formatting

Like `println!`, `format!` is also an macro which compiler will checks and
substitute for.

```rust
fn string_formatting() {
    let name = "John";
    let greeting = format!("Hi, {}! Nice to meet you", name);
    println!("{}", greeting);

    let hello = "Hello";
    let world = "World!";
    let hello_world = format!("{}, {}", hello, world);
    println!("{}", hello_world);

    let one = "One";
    let two = "two";
    let one_two_one = format!("{0}, {1}, {0}, {1}", one, two);
    println!("{}", one_two_one);

    let arabia = format!(
        "The trick, {first_name} {last_name}, is not minding that it hurts.",
        first_name = "William",
        last_name = "Porter");
    println!("{}", arabia);
}
```
