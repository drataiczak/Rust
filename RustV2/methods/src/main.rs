#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

// This implementation block allows us to define methods for a peice of data
// The first parameter of a method is ALWAYS self
// methods can take ownership, borrow mutably, or borrow immutably
impl Rectangle {
	// Calculates area
	fn area(&self) -> u32 {
		self.width * self.height
	}

	// Calculates if a rectangle can fit inside of another rectangle
	fn can_hold(&self, rect: &Rectangle) -> bool {
		self.width > rect.width && self.height > rect.height
	}

	// Called an associated function, does not take self as a parameter
	// Called using :: instead of .
	fn square(size: u32) -> Rectangle {
		Rectangle { 
			width: size,
			height: size
		}
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50
	};

	let rect2 = Rectangle {
		width: 20,
		height: 40
	};

	let rect3 = Rectangle {
		width: 40,
		height: 50
	};
	
	let rect4 = Rectangle::square(50);

	println!("The area of rect1 is {}", rect1.area());
	println!("Rect1 contain rect2? {}", rect1.can_hold(&rect2));
	println!("Rect1 contain rect3? {}", rect1.can_hold(&rect3));
	println!("rect4: {:#?}", rect4);
}
