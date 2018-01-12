struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

	print_black_fields(black);
	print_point_fields(origin);
}

fn print_black_fields(col: Color) {
	println!("{}:{}:{}", col.0, col.1, col.2);
}

fn print_point_fields(pt: Point) {
	println!("{}:{}:{}", pt.0, pt.1, pt.2);
}
