mod network {
	fn connect() {

	}
	
	mod client {
		// This is also poses no problems because it is within a different
		// module
		fn connect() {

		}
	}
}

//mod client {
	// Because this is not in the network namespace, it poses
	// no issues with naming
//	fn connect() {
//
//	}
//}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
