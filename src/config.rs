use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Config {
	username: String,
	format: String,

}

impl Settings {
	pub fn new() -> Result<Self, ConfigError> {
		let s = Config::builder()
			.add_source(File::with_name("config"))
			.add_source(Environment::with_prefix("app"))
			.build()?;

		println!("Config: {:?}", s.get::<String>("config.username"));
		println!("Config: {:?}", s.get::<String>("config.format"));
		s.try_deserialize()
	}
}