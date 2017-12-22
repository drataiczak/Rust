const CONST_NUM: i8 = 10;
const CONST_I32: i32 = 100_000;

fn main() {
	let x = 5;

	// This will not work because x is immutable
	// x = 6;

	println!("The value of x is {}", x);

	let mut y = 5;
	y = 6;
	println!("The value of y is {}", y);

	println!("Printing global const CONST_NUM: {}", CONST_NUM);
	println!("Printing global const CONST_I32: {}", CONST_I32);

	let x = 6;
	println!("The shadowed value of x is {}", x);

	let x = x + 1;
	println!("After incrementing and shadowing x: {}", x);

	let spaces = "    ";
	let spaces = spaces.len();
	println!("Spaces shadowed from a string to an integer: {}", spaces);

	// This would not work because you can't mutate data types
	// let mut spaces = "    ";
	// spaces = spaces.len();
}
