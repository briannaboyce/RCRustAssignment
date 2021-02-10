use std::env;
use std::fs;
use std::process::Command;
use serde_yaml;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::Write;
use std::fs::OpenOptions;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Process {
	path: String,
	name: String,
	arguments: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct File {
	path: Option<String>,
	file_type: String,
	name: String,
	content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct InputFile {
	process: HashMap<String, Process>,
	create: HashMap<String, File>,
	update: HashMap<String, File>,
	delete: HashMap<String, File>,
}

fn main() {

	//Collect input from a yaml file
	let args: Vec<String> = env::args().collect();

	let file = &args[1];
	let contents = fs::read_to_string(file)
		.expect("File does not exist");

	let yaml_data: InputFile = serde_yaml::from_str::<InputFile>(&contents).unwrap();

	//setup and run any processes as defined in YAML
	//TODO make this its own function
	let processes = yaml_data.process;

	//Iterate through all the processes in the YAML
	for (_name, process) in processes {
		let path = process.path;
		let name = process.name;
		let arguments = process.arguments;

		//If the command is on Windows it starts with a different command
		let mut command = if cfg!(target_os = "windows") {
			format!("cd {0} && start {1}", &path, &name).to_owned()
		} else {
			format!("cd {0} && chmod +x {0}{1} && ./{1}", &path, &name).to_owned()
		};

		//Arguments are optional, only add them to the command if they exist
		match arguments {
			Some(value) => {
				for val in value {
					command.push_str(&format!(" {:?}", val));
				}
			},
			None => {},
		}

		//Run the command based on OS
		if cfg!(target_os = "windows") {

			Command::new("cmd")
					.args(&["/C", &command])
					.spawn()
					.expect("failed to execute")
		} else {
			Command::new("sh")
					.arg("-c")
					.arg(&command)
					.spawn()
					.expect("failed to execute")
		};

	}

	//Create any new files as described by the YAML file
	let files_to_create = yaml_data.create;

	//Iterate through any files that need creation
	//TODO also make this its own function
	for (_name, file) in files_to_create {

		//Get file data
		let mut path = file.path;
		let file_type = file.file_type;
		let name = file.name;
		let content = file.content;

		//Check if path was provided
		//If so, use that
		//If not, use current directory
		let mut name_for_file =  format!("{}.{}", name, file_type);

		if let Some(specific_path) = &path {
			name_for_file = format!("{}{}.{}", *specific_path, name, file_type);
		}


		let mut new_file = OpenOptions::new()
							.create(true)
							.append(true)
							.open(name_for_file)
							.expect("creation failed");
		new_file.write_all(content.as_bytes()).expect("write failed");
	}

	println!("All done");

}