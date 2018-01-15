// Tells Rust that the code will be written elsewhere
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
	// Returns to root dir
	// use ::client;
	// Goes up one dir
	use super::client;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
	client::connect();
    }
}
