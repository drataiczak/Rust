use std::u8;

fn main() {
	println!("Function 1");

	function_two();
}

fn function_two() {
	println!("Function 2");

	function_three(255);
}

fn function_three(num: u8) {
	println!("Function 3 received value {}", num);

	let mut inc_num = function_four(num);

	println!("Function four returned value: {}", inc_num);
}

// Will cause the runtime to panic if the u8 overflows to a u32
fn function_four(num: u8) -> u8 {
	println!("Function 4 received value {}", num);
	
	// Expressions return their value automatically if they are the last line of a fn
	num + 1
}
