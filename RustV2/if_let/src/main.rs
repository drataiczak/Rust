fn main() {
	let u8_1 = Some(5);
	let u8_2 = Some(3);

	match_num(u8_1);
	match_num(u8_2);
	match_if_let(u8_1);
	match_if_let(u8_2);
}

fn match_num(n: Option<u8>) {
	match n {
		Some(3) => println!("Three"),
		_ => ()
	}
}

// Replaces match, but only cares about one single value
// Could also add an else statement, but this would be the same as 
// matching to _
fn match_if_let(n: Option<u8>) {
	if let Some(3) = n {
		println!("Three");
	}
}
