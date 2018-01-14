#[derive(Debug)]
enum State {
	Alabama,
	Alaska,
	// ...
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(State),
}

fn main() {
	let p = Coin::Penny;
	let n = Coin::Nickel;
	let d = Coin::Dime;
	let q = Coin::Quarter(State::Alabama);
	let mut count: u32 = 0;

	println!("Penny: {}", get_value(&p));
	println!("Nickel: {}", get_value(&n));
	println!("Dime: {}", get_value(&d));
	println!("Quarter: {}", get_value(&q));

	count = count_coin(&p, count);
	count = count_coin(&n, count);
	count = count_coin(&d, count);
	count = count_coin(&q, count);

	println!("The total of coins minus quarters is: {}", count);
}

fn get_value(c: &Coin) -> u8 {
	match c {
		&Coin::Penny => 1,
		&Coin::Nickel => 5,
		&Coin::Dime => 10,
		&Coin::Quarter(ref state) => {
			println!("State: {:?}!", state);
			25
		},
	}
}
	
fn count_coin(c: &Coin, mut cnt: u32) -> u32 {
	// Matches only quarters. Must use ref state
	// to appease the compiler. We must borrow the enum
	// (hence, ref) so that we can re-use it here
	if let &Coin::Quarter(ref state) = c {
		println!("State: {:?}", state);
	}
	else {
		cnt += get_value(c) as u32;
	}

	cnt
}
