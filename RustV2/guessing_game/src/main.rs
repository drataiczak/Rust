extern crate rand;

use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 101);    

    println!("Guess the number!");

    loop {
	let mut guess = String::new();

    	print!("Please input your guess: ");
    	io::stdout().flush().expect("Flush failed"); 
    	// Must flush stdout so that we print prompt before reading
    
    	// read_line() DOES sanitize input
    	io::stdin().read_line(&mut guess).expect("Failed to read the line.");
    	let guess: i32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => continue,
	};

    	println!("You guessed: {}", guess);

    	match guess.cmp(&secret_num) {
		Ordering::Less => println!("Your guess was too low!"),
		Ordering::Equal => {
					println!("Your guess was correct!");
				   	break;
				},
		Ordering::Greater => println!("Your guess was too high!")
    	}
    }
}
