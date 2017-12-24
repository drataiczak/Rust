use std::io::{stdin, stdout, Write};

fn main() {
	let fahr = get_input();
	let cels = convert(fahr);
	
	println!("{} Fahrenheit converts to {} Celsius", fahr, cels);
}

fn get_input() -> i32 {
	let f: i32;	

	loop {
		print!("Please enter a value of Fahrenheit to convert: ");
		stdout().flush().expect("Unable to flush stdout");

		let mut inter = String::new();
		stdin().read_line(&mut inter).expect("Unable to read line");

		f = match inter.trim().parse() {
			Ok(n) => n,
			Err(_) => continue,
		};

		break;
	}
	
	return f;
}

fn convert(f: i32) -> i32 {
	((f - 32) * 5) / 9
}
