use std::fs::File;
use std::io::{BufWriter, stdin, stdout, Write};

pub struct Todo {
	pub description: String,
	pub done: bool,
}

pub struct TodoFile {
	pub username: String,
	pub todo: Vec<Todo>,
}

impl TodoFile {
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
	let username = String::from("bulut");
	let file_name = format!("{}.txt", username);
	let mut file = TodoFile::new(&file_name).unwrap();

	// File creation and writer initialization
	let create_file = File::create(&file_name).expect("Could not create file");
	let mut writer = BufWriter::new(create_file);

	loop {
		let mut s = String::new();
		print!("Do you want to add a new todo? (q to quit): ");
		let _ = stdout().flush();
		stdin().read_line(&mut s).expect("Did not enter a correct string");
		let s = s.trim();
		if s == "q" {
			break;
		}

		let mut todo = Todo {
			description: s.to_string(),
			done: false,
		};

		println!("You added a new todo: {}", todo.description);
		println!("Do you want to mark it as done? (y/n)");
		let mut s = String::new();
		let _ = stdout().flush();
		stdin().read_line(&mut s).expect("Did not enter a correct string");
		let s = s.trim();
		match s {
			"y" => {
				println!("You marked the todo as done!");
				todo.done = true;
			}
			"n" => {
				println!("You did not mark the todo as done!");
			}
			_ => {
				println!("Invalid input. Try again please!");
			}
		}

		file.todo.push(todo);
	}

	for todo in &file.todo {
		let status = if todo.done { "done" } else { "not done" };
		writeln!(writer, "Todo: {}, Status: {}", todo.description, status).expect("Could not write to file");
	}

	writer.flush().expect("Failed to flush writer");

	println!("Todos have been written to the file.");
	println!("Goodbye!");
}
