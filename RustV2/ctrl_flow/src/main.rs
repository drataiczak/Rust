use std::io::{self, stdin, stdout, Write};

fn main() {
	let mut num: i8;

	loop {
		print!("Please enter a value between 1 and 10 inclusive: ");
		stdout().flush().expect("Unable to flush stdout");
	
		let mut entry = String::new();
		stdin().read_line(&mut entry).expect("Unable to read stdin");

		// Parse to i8
		num = match entry.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		if num <= 0 {
			continue;
		}
		if num > 10 {
			continue;
		}
		
		break;
	}

	if num > 5 {
		println!("Number greater than 5");
	}
	if num <= 5 {
		println!("Number less than or equal to 5");
	}

	
	// Could be replaced by a series of else if statements:
	// if num % 10 == 0 {
	// 	println!("Number is divisible by 10");
	// }
	// else if num % 9 == 0 {
		println!("Number is divisible by 9");
	// }
	// ...
	let mut divis = 10;

	loop {
		if divis == 1 {
			break;
		}
		
		if num % divis == 0 {
			println!("{} is divisible by {}", num, divis);
		}	

		divis = divis - 1;
	}

	if_let_fn(num);

	while_fn(num);
	
	for_loop_fn(num);
}

fn if_let_fn(num: i8) {
	let cond = true;

	// Because an if statement is an expression, it can be assigned to a variable
	let num = if cond == true {
			5
		}
		else {
			// We could not do:
			// "six"
			// because we would have a type mismatch with the first
			// half of the if/else statement
			6
		};

	println!("Nev value of num: {}", num);
}

fn while_fn(num: i8) {
	let mut num = num;

	while num != 0 {
		num = num - 1;
		println!("Decrementing... Value: {}", num);
	}

	println!("While loop complete");
}

fn for_loop_fn(num: i8) {
	// An easy way to create a for loop
	for n in (0..num).rev() {
		println!("Decrementing for loop... Value: {}", n);
	}

	println!("For loop complete");

	// For loop using an array iterator
	let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	for n in arr.iter().rev() {
		println!("Decrementing for loop... Value: {}", n);
	}

	println!("For loop complete again");
}
