// Traits act similarly to Java's interfaces and C's header files
// If a function implements a specific trait, it must implement all methods
// that trait contains
pub trait Summarizable {
	// Implements a default behavior for the summary function that can be overriden
	fn summary(&self) -> String {
		format!("(Read more from {}...)", self.author_summary())
	}

	fn author_summary(&self) -> String;
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summarizable for NewsArticle {
	fn summary(&self) -> String {
		format!("{}, by {}, ({})", self.headline, self.author, self.location)
	}

	fn author_summary(&self) -> String {
		format!("{}", self.author)
	}
}

impl Summarizable for Tweet {
//	fn summary(&self) -> String {
//		format!("{}: {}", self.username, self.content)
//	}

	fn author_summary(&self) -> String {
		format!("@{}", self.username)
	}
}

pub fn notify<T: Summarizable>(item: T) {
	println!("Breaking new! {}", item.summary());
}
