extern crate rand;

use std::io;
use std::io::stdout;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let s_num = rand::thread_rng().gen_range(1, 101);

	println!("Guess the number!");

	loop {
		print!("Please enter a number: ");
		stdout().flush().expect("Unable to flush stdout");

		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Unable to read line");

		println!("Your guess was {}", guess.trim());

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&s_num) {
			Ordering::Less => println!("Your guess was too small"),
			Ordering::Greater => println!("Your guess was too big"),
			Ordering::Equal => {
				println!("You won!");
				break;
			}
		}
	}
}
