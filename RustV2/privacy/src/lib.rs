pub mod outermost {
	pub fn middle() {}
	
	pub fn middle_priv() {}

	pub mod inner {
		pub fn inner_fn() {}

		pub fn inner_priv() {}
	}
}

fn test() {
	outermost::middle();
	outermost::middle_priv();
	outermost::inner::inner_fn();
	outermost::inner::inner_priv();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
