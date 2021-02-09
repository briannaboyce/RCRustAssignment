use std::env;
use std::fs;
use std::process::Command;
use serde_yaml;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Process {
	path: String,
	name: String,
	arguments: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct File {
	path: String,
	r#type: String,
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

	let processes = yaml_data.process;

	println!("the process is: {:?}", processes);


	for (name, process) in processes {
		let path = process.path;
		let name = process.name;
		let mut arguments = process.arguments;

		println!("{}", path);
		println!("{}", name);
		println!("{:?}", arguments);


		let mut list_of_args: Vec<String> = Vec::new();

		let mut command = if cfg!(target_os = "windows") {
			format!("cd {0} && start {1}", &path, &name).to_owned()
		} else {
			format!("cd {0} && chmod +x {0}{1} && ./{1}", &path, &name).to_owned()
		};

		match arguments {
			Some(value) => {
				for val in value {
					command.push_str(&format!(" {:?}", val));
				}
			},
			None => break,
		}

		println!("{}", command);


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
		println!("All done");

}