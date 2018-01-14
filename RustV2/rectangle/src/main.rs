// This allows println!() to print Rectangle structs
#[derive(Debug)]
struct Rectangle {
	height: u32,
	width: u32,
}

fn main() {
	let width1 = 30;
	let height1 = 50;
	let rect2 = (30, 50);
	let rect3 = Rectangle {
		height: 30,
		width: 50
	};

	println!("The area of rectangle1 is {}", area(width1, height1));
	println!("The area of rectangle2 is {}", area_tuple(rect2));
	println!("The area of rectangle3 is {}", area_struct(&rect3));
	// Will display a struct with #[derive(Debug)] before it
	println!("rect3: {:?}", rect3);
	// Will print a prettier struct
	println!("rect3: {:#?}", rect3);

}

fn area_struct(rect: &Rectangle) -> u32 {
	rect.height * rect.width
}

fn area_tuple(dim: (u32, u32)) -> u32 {
	dim.0 * dim.1
}

fn area(width: u32, height: u32) -> u32 {
	width * height
}
