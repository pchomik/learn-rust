use colored::*;
use rand::random_range;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number from 0 to 100");

    let secret_number: u32 = random_range(0..100);

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please provide the number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Provided number {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Number is too small".red()),
            Ordering::Equal => {
                println!("{}", "Bingo".green());
                break;
            }
            Ordering::Greater => println!("{}", "Number is too big".red()),
        };
    }
}
