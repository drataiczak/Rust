enum TrafficLight {
	Red,
	Yellow,
	Green,
}

pub mod a {
	pub mod series {
		pub mod of {
			pub fn nested_modules() {}
		}
	}
}

use a::series::of;
// Brings multiple variants into scope
// use TrafficLight::{ Red, Yellow, Green };
// Brings ALL variants into scope
 use TrafficLight::*;

fn main() {
	of::nested_modules();
	let red = Red;
	let yellow = Yellow;
	let green = Green;
}
