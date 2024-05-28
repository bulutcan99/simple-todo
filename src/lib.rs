pub struct Todo {
	pub description: String,
	pub done: bool,
}

pub struct File {
	pub username: String,
	pub todo: Vec<Todo>,
	pub format: String,
}

impl File {
	pub fn new(username: String, format: String) -> Result<Self, Err()> {}
}