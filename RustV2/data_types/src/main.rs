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
}
