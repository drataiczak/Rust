extern crate traits;
use traits::Summarizable;

pub struct WeatherForecast {
	pub high: f64,
	pub low: f64,
	pub precip: f64,
}

impl Summarizable for WeatherForecast {
	fn summary(&self) -> String {
		format!("High: {}, Low: {}, Precip: {}", self.high, self.low, self.precip)
	}
}
