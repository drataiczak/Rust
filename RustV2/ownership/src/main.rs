// If a type has the Copy trait, an older variable is still usable after assignment
// e.g. int - Has a known compile time size, but also has the Copy trait
// A type cannot have the Copy trait if it also has the Drop trait

fn main() {
	// A string literal is baked into the binary, a String is not
	let mut st: String;
	
	// Essentially a malloc() call
	// The '::' symbol means that from() is a function of String
	st = String::from("Hello"); // Creates a string from the text "Hello"

	// String's are mutable!
	st.push_str(", world!");

	let st2 = st; // This would copy the pointer st holds, not the actual data
	// When drop() is called, this will cause a double free because
	// Rust will try to free the pointer st, and st2, which is the same as st
	// To prevent the double free, st would become invalid
	// This is similar to a shallow copy, but is actually called a 'move'


	// This will NOT work because st is no longer valid, it has been moved to st2
	// println!("st contains: \'{}\'", st);

	// This will deep copy st2
	let st3 = st2.clone();

	println!("st2 contains: \'{}\'", st2);
	println!("st3 contains: \'{}\'", st3);

} // st2 is no longer valid; Essentially a free() call
// Rust calls drop() when a variable goes out of scope
