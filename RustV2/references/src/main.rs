fn main() {
	let mut st = String::from("Hello");
	// Passes a reference to st so we do not have to return a tuple
	let len = calc_len(&mut st);

	{
		let st2 = &mut st;
	}

	// We cannot have to mutable references to the same data
	// in the same scope. However, in different scopes we can
	let st3 = &mut st;

	// This will not work because Rust does not allow you to have both
	// a mutable and immutable reference to the same data
	// let st4 = &st;
	

	println!("The length of st is: {}", len);
}

fn calc_len(st: &mut String) -> usize {	
	// This would not work if st was immutable
	st.push_str(", world!");

	st.len()
} // st will be dropped here, but the data it refers to will not
