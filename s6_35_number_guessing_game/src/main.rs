use rand::Rng;
use core::num;
use std::io::stdin;

fn main() {
    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter your guess: ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                // Remove the line break.
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guest is out of range");
                        } else if guess < number {
                            println!("Your guest is too low");
                        } else if guess > number {
                            println!("Your guest is too high");
                        } else {
                            println!("Correct!!!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e);
                    }
                }
            },
            Err(_) => continue,
        }
    }
}
