fn main() {
	let mut str1 = String::from("Hello,");
	str1.push_str(" world!");
	
	let word = find_first_word(&str1);

	
	println!("First word: {}", word);
}

fn find_first_word(strn: &str) -> &str {
	let byte_arr = strn.as_bytes();

	for (index, &element) in byte_arr.iter().enumerate() {
		if element == b' ' {
			return &strn[..index];
		}
	}

	&strn[..]
	
}
