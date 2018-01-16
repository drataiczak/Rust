use std::collections::HashMap;

fn main() {
	let mut int_vec = vec![10, 1, 3, 6, 7, 1, 9, 7, 10, 7, 2, 9, 4];
	let mean: f32;
	let mut mode;
	let mut map = HashMap::new();
	let mut tmp = 0.00;

	// 1, 1, 2, 3, 4, 6, 7, 7, 7, 9, 9, 10, 10
	int_vec.sort();

	for e in int_vec.iter() {
		tmp += *e as f32;
	}

	// 83 / 13
	mean = tmp / int_vec.len() as f32;
	let median = &int_vec[int_vec.len() / 2];

	for e in int_vec.iter() {
		let count = map.entry(*e).or_insert(0);
		*count += 1;
	}

	let mut best = 0;
	mode = 0;
	for (k, v) in map.iter() {
		if *v > best {
			best = *v;
			mode = *k;
		}
	}

	println!("Mean: {}", mean);
	println!("Median: {}", median);
	println!("Mode: {}", mode);
}
