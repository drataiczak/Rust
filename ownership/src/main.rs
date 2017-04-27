fn main() {
	println!("Stack string creation... ");
	let s = "Hello";
	println!("Stack'd string: {}", s);

	println!("Throwing it on the heap, son");
	let mut t = String::from(s);
	println!("Dat heap: {}", t);

//	println!("Attempting stack concatenation..."); This ONLY works on heap
//	println!("{}, s.push_str(" world!"));
	
	println!("Concatenating dat heap bro");
	t.push_str(", world!");
	println!("{}", t);

	println!("-------------");
	
	let a = String::from("New str");
	let b: u32 = 1;

	steal_heap(a);
	touch_stack(b);

//	println!("a: {}", a); A move has occurred, this won't work
	println!("b: {}", b);

	let c = String::from("String Two");
	let mut d = String::from("String ");
	ref_heap(&c, &mut d);
	println!("Omg I can still use c and d: {} {}", c, d);
}

fn steal_heap(c: String) {
	println!("Lol, your string is invalid now, loser. Here it is {}", c);
}

fn touch_stack(d: u32) {
	println!("You can still use your u32. Here's the value {}", d);
}

fn ref_heap(str: &String, str2: &mut String) {
	println!("Hey, string is working! {}", str);
	str2.push_str("three");
	println!("Here's an edit of str2: {}", str2);
}
