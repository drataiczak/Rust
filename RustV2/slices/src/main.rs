use std::io;
use std::io::Write;

fn main() {
	let st = String::from("Hello good sir!");
	
	let idx = first_word_index(&st);
	
	let arr = [1, 2, 3, 4, 5];

	// We have to use a slice here
	// The syntax is essentially a reference to a String with indices in brackets
	println!("The first word of your string is: {}", &st[..idx]);

	println!("Now using a slice method: {}", first_word_by_slice(&st));

	// Slices can be used for other data types too
	for e in &arr[..3] {
		print!("{} ", e);
		io::stdout().flush().expect("Unable to flush stdout");
	}
	println!("");
}

fn first_word_index(st: &String) -> usize {
	let bytes = st.as_bytes();

	for(idx, &byte) in bytes.iter().enumerate() {
		if byte == 0x20 {
			return idx;
		}
	}

	st.len()
}

// The parameter as a &str allows it to accept Strings and &strs
// This is done by passing a String as a slice of itself
// e.g.:
// let st = String::from("Hello, world");
// let first = first_word_by_slice(&st[..]);
fn first_word_by_slice(st: &str) -> &str {
	let bytes = st.as_bytes();

	for(idx, &byte) in bytes.iter().enumerate() {
		if byte == b' ' {
			return &st[..idx];
		}
	}
	
	// Will return the whole string
	&st[..]
}
