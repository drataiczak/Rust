extern crate traits;
use traits::Summarizable;
use traits::Tweet;

fn main() {
	let tweet = Tweet {
		username: String::from("Devin"),
		content: String::from("Here's a tweet!"),
		reply: false,
		retweet: false,
	};

	traits::notify(tweet);
//	println!("One new tweet: {}", tweet.summary());
}
