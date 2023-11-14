use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess a number between 1 and 10000 >:)");

    let solution = rand::thread_rng().gen_range(1..=10000);

    loop {
        print!("Your guess: ");
        io::stdout().flush().expect("Failed to flush to stdout"); // unwrap ignores errors

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line :(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue
            }
        };

        match guess.cmp(&solution) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Congragulations! {guess} was the right number.");
                break;
            }
        } 
    }
}
