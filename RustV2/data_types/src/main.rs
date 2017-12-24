use std::mem::size_of;

fn main() {
	// Will not work because parse() cannot infer a type
	// let guess = "42".parse().expect("NaN");

	// Will work because we define the type of guess
	let guess: i8 = "42".parse().expect("NaN");

	println!("Value of guess: {}", guess);

	// Integer types:
	// 8-bit, 16-bit, 32-bit, 64-bit, isize
	// They all come in signed (i<size>) and unsigned (u<size>) variants

	// isize and usize produce the same number
	// Compiling with cargo built --target i686-unknown-linux-musl
	// will compile this as a 32 bit binary and show the appropriate
	// architecture value (64 for 64-bit, 32 for 32-bit)
	println!("Size of isize: {}", size_of::<isize>() * 8);	

	let t = true;
	let f: bool = false;

	println!("Boolean value t {}, f {}", t, f);

	let snowman = '\u{2603}';
	println!("Snowman: {}", snowman);

	// Tuples can group different data types
	let tup: (i32, f64, u8) = (500, 6.4, 1);

	// Use <name>.<index> to access individual items
	println!("tup contains: {} {} {}", tup.0, tup.1, tup.2);

	// can also create a pattern for matching by making a tuple variable:
	let (x, y, z) = tup;
	println!("Value of y in tup is: {}", y);

	// Arrays cannot grow in size in Rust
	let arr = [1, 2, 3, 4, 5];

	// Indexed just like C
	println!("Element at index 3 is {}", arr[3]);

	// Cannot access out of bounds, the runtime, NOT COMPILER, stops you
	// println!("Element at index 5: {}", arr[5]);
}
