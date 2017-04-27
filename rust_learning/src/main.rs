fn main() {
	print_mism_types(5, 6);
	
	// This works because x is bound to 6. println! does not alter its value
	let x: u8 = add_one(5);
	println!("x: u8 immutable - add one: {}", add_one(x));
}

/// We CANNOT type mismatch. Rustc will throw an error
fn print_mism_types(x: u8, y: u8) {
	println!("x + y = {}", x + y);
}

fn add_one(x: u8) -> u8 {
	x + 1
}
