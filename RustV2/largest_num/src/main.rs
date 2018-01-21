fn main() {
	let num = vec![34, 50, 25, 100, 65];
	

	let largest = get_largest(&num);

	println!("The largest number is: {}", largest);
}

fn get_largest(list: &[i32]) -> i32 {
	let mut largest = list[0];

	for &n in list.iter() {
		if n > largest {
			largest = n;
		}
	}

	largest
}
