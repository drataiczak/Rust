fn main() {
	let vec = vec![1, 2, 3];

	// This will cause a panic! because of buffer overread
	// To see the backtrace from this panic!() we use the command
	// RUST_BACKTRACE=1 cargo run
	vec[100];

	// panic!() will walk back the stack and clean up all data.
	// To instead immediately exit, add the following lines to the
	// Cargo.toml file
	// [profile.release]
	// panic = 'abort'
	panic!("Sugar we're going down!");
}
