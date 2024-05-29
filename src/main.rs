pub struct Todo {
	pub description: String,
	pub done: bool,
}

pub struct File {
	pub username: String,
	pub todo: Vec<Todo>,
}

impl File {
	pub fn new(username: &str) -> Result<Self, String> {
		match username {
			"" => Err("Username cannot be empty".to_string()),
			_ => Ok(Self {
				username: username.to_string(),
				todo: Vec::new(),
			}),
		}
	}
}

fn main() {
	let username: &str = "Bulut";
	let file = File::new(username).unwrap();
}



