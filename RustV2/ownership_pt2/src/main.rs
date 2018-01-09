fn main() {
	let st = String::from("Hello");

	take_ownership(st); // st is invalid after this

	let st2 = give_ownership(); // st2 is given ownership from the function

	let st3 = take_and_give_ownership(st2); // st3 has ownership taken and returned
	// st3 now has ownership of the data in st2
	// st2 is now invalid

	let (st4, len) = strlen(st3); // st3 is now invalid

	let x = 5;

	println!("st4: {}, len: {}", st4, len);

	make_copy(x); // x will still be valid
}

fn strlen(st: String) -> (String, usize) {
	let len = st.len();
	(st, len) // Returns two values
}

fn take_ownership(st: String) {
	println!("st: {}", st);
} // st will be dropped

fn give_ownership() -> String {
	let st = String::from("Hello");
	st
}

fn take_and_give_ownership(st: String) -> String {
	st
}

fn make_copy(x: i32) {
	println!("x: {}", x); // x is not dropped because it is copied, not moved
}
