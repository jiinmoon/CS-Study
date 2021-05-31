#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;

const PI: f32 = 3.14;
static mut PI2: f32 = 3.14;

fn if_statement() {
    let num = 21;

    if num % 3 == 0 && num % 5 == 0 {
        println!("{} is Fizz Buzz", num);
    } else if num % 3 == 0 {
        println!("{} is Fizz", num);
    } else {
        println!("{} if Buzz", num);
    }

    // if-statement also works in assignments like Python
    let result = if num % 3 == 0 { "Fizz" } else { "Buzz" };
    println!("{}", result);
}

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

fn data_types() {
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

fn main() {
    //data_types();
    //operators();
    //scope_and_shadows();
    /*
    println!("{}", PI);
    unsafe {
        PI2 *= 2.0;
        println!("{}", PI2);
    }
    */
    //if_statement();
    //loops();
    //match_statement();
}
