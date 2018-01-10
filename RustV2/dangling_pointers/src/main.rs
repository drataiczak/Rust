fn main() {
	let st = no_dangle();
}

// Cannot work because st will be out of scope and leave the 
// caller with a dangling pointer
// fn dangle() -> &String {
//	let st = String::from("Hello");
//
//	&st
// }

// Because a String is returned, the data is allocated on the heap and
// does not go out of scope after no_dangle()
fn no_dangle() -> String {
	let st = String::from("Hello!");

	st
}
