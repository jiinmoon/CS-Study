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
