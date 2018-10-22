extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Please input your guess: ");

    let secret_number = rand::thread_rng().gen_range::<u8>(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Could not read line / guess");

        let guess_int = match guess.trim_right().parse::<u8>() {
            Ok(n) => n,
            Err(_) => {
                println!("{} is not an integer", guess.trim_right());
                std::process::exit(1);
            }
        };

        match guess_int.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too low."),
            Ordering::Greater => println!("Your guess is too high."),
            Ordering::Equal => {
                println!("Your guess is correct.");
                break;
            }
        };

    }
}
