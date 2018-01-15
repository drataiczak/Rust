fn main() {
	// Vectors can only store one type of value, so it must be type annotated
	let mut v: Vec<i32> = Vec::new();
	// Because this is initialized with a datatype, we do not need to annotate v2
	let mut v2 = vec![1, 2, 3];

	// The push() function adds an element to the vector
	v.push(1);
	v.push(2);
	v.push(3);
	v2.push(4);
	v2.push(5);
	v2.push(6);

	// Indexing vectors:
	// This will cause a panic! if it references out of bounds data
	//let third: &i32 = &v[2];
	// Returns None if it accesses out of bounds data
	//let third_v2: Option<&i32> = v.get(2);

	for e in &v {
		println!("{} ", e);
	}

	for e in &v2 {
		println!("{} ", e);
	}

	// Loops through and alters v
	for e in &mut v {
		*e += 50;
	}

	for e in &v {
		println!("{}", e);
	}
}
