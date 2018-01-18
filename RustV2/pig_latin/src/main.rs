use std::io;
use std::io::Write;

fn main() {
	let string3 = "Here's five apples in an igloo from a farm in Ontario";

	let vec: Vec<&str> = string3.split(' ').collect();

	for string in vec.iter() {
		print!("{} ", convert(string));
		io::stdout().flush().expect("Unable to fluh stdout");
	}
	println!("");
}

fn convert(string: &str) -> String {
	let mut converted: String = String::from("");	

	if !(string.to_lowercase().starts_with('a') ||
		string.to_lowercase().starts_with('e') ||
		string.to_lowercase().starts_with('i') ||
		string.to_lowercase().starts_with('o') ||
		string.to_lowercase().starts_with('u')) {
		let mut tmp = String::from(string);
		let ch = tmp.remove(0);

		converted.push_str(&tmp);
		converted.push_str("-");
		converted.push(ch.to_lowercase().nth(0).unwrap());
		converted.push_str("ay");
		
	}
	else {
		converted = String::from(string);
		converted.push_str("-hay");
	}

	return converted;
}
