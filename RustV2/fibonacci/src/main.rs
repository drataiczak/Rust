use std::io::{stdin, stdout, Write};

fn main() {
	let (mut first, mut second, count) = get_input();
	let mut result: u32 = 0;

	println!("Calculating the {} number of the Fibonacci sequence starting with {} and {}", count, first, second);

	// Do actual fib calculations here
	for n in 1..count {
		result = calculate_fib(first, second);
		first = second;
		second = result;
	}
	println!("The {} number n the sequence is {}", count, result);
}

fn get_input() -> (u32, u32, u32) {
	let mut first: u32;
	let mut second: u32;
	let count: u32;

	loop {
		let mut f = String::new();
		let mut s = String::new();
		let mut c = String::new();

		print!("Please enter the first number of the sequence: ");
		stdout().flush().expect("Unable to flush stdout");
		stdin().read_line(&mut f).expect("Failed to read line");
	
		print!("Please enter the second number of the sequence: ");
		stdout().flush().expect("Unable to flush stdout");
		stdin().read_line(&mut s).expect("Failed to read line");

		print!("Please enter the number of the sequence to find: ");
		stdout().flush().expect("Unable to flush stdout");
		stdin().read_line(&mut c).expect("Failed to read line");

		first = match f.trim().parse() {
			Ok(n) => n,
			Err(_) => continue,
		};		

		second = match s.trim().parse() {
			Ok(n) => n,
			Err(_) => continue,
		};			
		
		count = match c.trim().parse() {
			Ok(n) => n,
			Err(_) => continue,
		};

		break;
	}
	
	return (first, second, count);
}

// Can only handle up to 46-ish. It will overflow u32 at that point
fn calculate_fib(first: u32, second: u32) -> u32 {
	first + second
}
