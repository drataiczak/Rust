fn main() {
	#[derive(Debug)]
	let five = Some(5);
	
	#[derive(Debug)]
	let six = plus_one(five);
	
	#[derive(Debug)]
	let none = plus_one(None);

	println!("Five: {:?}", five);
	println!("Size: {:?}", six);
	println!("None: {:?}", none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(n) => Some(n + 1),
	}
}
