// Create a struct
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn main() {
	// Create an INSTANCE of the struct
	// The whole struct must be mutable, cannot have mutable fields
	let mut user1 = User {
		// Data can be out of order because of key:value pairs
		username: String::from("Devin"),
		active: true,
		sign_in_count: 1,
		email: String::from("abc123@gmail.com"),
	};

	println!("Username: {}", user1.username);
	
	user1.active = false;

	println!("{}:{}:{}:{}", user1.username, user1.active, user1.sign_in_count, user1.email);

	let user2 = create_user(String::from("Devin2"), 
			String::from("cde456@gmail.com"),
			true, 1);

	println!("{}:{}:{}:{}", user2.username, user2.active, user2.sign_in_count, user2.email);

	let user3 = create_user_init(String::from("Devin3"), String::from("fgh789@gmail.com"));

	println!("{}:{}:{}:{}", user3.username, user3.active, user3.sign_in_count, user3.email);

	let user4 = User {
		username: String::from("Devin4"),
		email: String::from("ijk012@gmail.com"),
		..user1 // This will take the rest of the fields from the user1 struct
	};

	println!("{}:{}:{}:{}", user4.username, user4.active, user4.sign_in_count, user4.email);


}

// Instantiates and returns a struct
fn create_user(name: String, email: String, active: bool, sic: u64) -> User {
	User {
		username: name,
		email: email, // We can have the same name for params and struct members
		active: active,
		sign_in_count: sic,
	}
}

fn create_user_init(username: String, email: String) -> User {
	// If a parameter shares a name with a struct member,
	// it can be initialized by simply declaring the name of the parameter/member`
	User {
		username,
		email,
		active: true,
		sign_in_count: 1,
	}
}
