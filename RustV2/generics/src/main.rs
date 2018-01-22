// Structs can also use generics:
// This declaration indicates that x and y are of the same type
struct Point<T> {
	x: T,
	y: T,
}

// can operate on Point structs of any type
impl <T> Point<T> {
	fn get_x(&self) -> &T {
		&self.x
	}
}

// Can only operate on point structs that are f32s
impl Point<f32> {
	fn dist_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

// Allows us to define a function that works on Point2 structs but allows us
// to have a second Point2 that we operate on
impl<T, U> Point2<T, U> {
	fn mixup<V, W>(self, other:Point2<V, W>) -> Point2<T,W> {
		Point2 {
			x: self.x,
			y: other.y,
		}
	}
}

// This declaration indicates that x and y are of diffrent types
struct Point2<T, U> {
	x: T,
	y: U,
}

fn main() {
	let v = vec![10, 20, 100, 40, 65, 22];
	let max = get_largest(&v);
	println!("The largest integer in the list is: {}", max);

	let v = vec!['y', 'm', 'a', 'q'];
	let max = get_largest(&v);
	println!("The largest char is: {}", max);

	let int_point = Point { 
		x: 5,
		y: 10
	};

	let float_point = Point {
		x: 1.0,
		y: 4.0
	};
	
	let two_type_point = Point2 {
		x: 1,
		y: 4.0
	};

	let two_type_point2 = Point2 {
		x: "Hello",
		y: 'c'
	};

	let x = int_point.get_x();
	println!("The value of x is: {}", x);

	println!("Dist from origin of float_point: {}", float_point.dist_from_origin());

	let ttp = two_type_point.mixup(two_type_point2);
	println!("ttp.x {}; ttp.y {};", ttp.x, ttp.y);
}

fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut max = list[0];

	for &e in list {
		if e > max {
			max = e;
		}
	}

	max
}
