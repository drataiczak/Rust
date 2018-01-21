use std::fs::File;
use std::io;
use std::io::{Read, ErrorKind};

fn main() {
	let f = File::open("Hello.txt");

	let f = match f {
		Ok(file) => file,
		Err(ref e) if e.kind() == ErrorKind::NotFound  => {
			match File::create("Hello.txt") {
				Ok(fd) => fd,
				Err(err) => {
					panic!("Unable to create file: {:?}", err)
				},
			}	 
		},
		Err(err) => {
			panic!("Unable to open file {:?}", err)
		},
	};

	let pass = err_propagation();
	let pass2 = shortcut_propagation();
	println!("Password: {}", pass.unwrap());
	println!("Password2: {}", pass2.unwrap());

	
	// This will fail with a standard unwrap() error
	// let f2 = File::open("Hello2.txt").unwrap();
}

fn err_propagation() -> Result<String, io::Error> {
	let f = File::open("Hello.txt");
	
	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

fn shortcut_propagation() -> Result<String, io::Error> {
	let mut f = File::open("Hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}
