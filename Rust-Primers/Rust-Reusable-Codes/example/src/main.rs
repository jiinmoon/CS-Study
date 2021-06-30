extern crate rand;

use rand::random;
use std::io;

fn check_user_input(user_input: u8, secret: u8) -> bool
{
    if user_input < secret {
        println!("Your guess is smaller");
    } else if user_input > secret {
        println!("Your guess is larger");
    } else {
        println!("You got it!");
        return true;
    }
    false
}

fn get_user_guess() -> u8
{
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Please, enter a valid positive integer");

        match user_input.trim_end().parse::<u8>() {
            Ok(result) => return result,
            Err(error) => println!("{:?}", error)
        };
    }
}

fn main() {
    let secret = random::<u8>();
    loop {
        let user_guess: u8 = get_user_guess();
        println!("Your input : {}", user_guess);
        
        if check_user_input(user_guess, secret) {
            break
        }
    }
}
