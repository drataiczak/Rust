use std::collections::HashMap;

fn main() {
	let mut scores = HashMap::new();
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let init_score = vec![10, 15];

	// This zips the two vectors together into a vector of tuples of format:
	// (teams, init_score). It then collects the vector and turns each tuple
	// into a hashmap

	// For data with the Copy trait, data is copied to the hashmap. Otherwise it is
	// Moved to the hashmap
	let scores_vec: HashMap<_, _> = teams.iter().zip(init_score.iter()).collect();

	let new_team = String::from("Green");
	let new_score = 15;

	// insert() will add a new key/value pair
	scores.insert(new_team, new_score);

	// Insert adds an item to the hashmap
	// insert(key, value)
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 5);

	// Gets the associated value
	//let sc = scores.get("Blue");

	scores.entry(String::from("Green")).or_insert(60);
	scores.entry(String::from("Purple")).or_insert(60);

	for (k, v) in &scores {
		println!("{}:{}", k,v);
	}

	let text = "Here is my really big string of text!";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}

	println!("{:?}", map);
}
