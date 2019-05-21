use std::io;
use std::cmp::Ordering;

extern crate rand;
use rand::Rng;

fn main() {
    let min = 1;
    let max = 101;
    let secret_number = rand::thread_rng().gen_range(min, max);

    // max - 1 because max is exclusive in gen_range
    println!("Guess the number between {} and {}!", min, max - 1);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
