#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use rand::Rng;
use std::io::stdin;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter your guess: ");

        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let input = buffer.trim_end().parse::<i32>();
                match input {
                    Ok(user_guess) => {
                        if user_guess < 1 || user_guess > 100 {
                            println!("Your guess is out of range.");
                        } else if user_guess < secret {
                            println!("Your guess is lower than secret.");
                        } else if user_guess > secret {
                            println!("Your guess is greater than secret.");
                        } else {
                            println!("Your guess is correct! It is {}.", secret);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Wrong input! Try again.");
                    }
                }
            },
            Err(_) => continue,
        }

    }
}
