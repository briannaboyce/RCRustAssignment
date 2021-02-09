use std::env;
use std::fs;
use std::process::Command;
use serde_yaml;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Process {
	path: String,
	name: String,
	arguments: std::collections::HashMap<String, String>,
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
	process: std::collections::HashMap<String, Process>,
	create: std::collections::HashMap<String, File>,
	update: std::collections::HashMap<String, File>,
	delete: std::collections::HashMap<String, File>,
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
		let arguments = process.arguments;

		println!("{}", path);
		println!("{}", name);


		let mut list_of_args: Vec<String> = Vec::new();

		for(key, value) in arguments {
			list_of_args.push(format!(" {}={}", key, value));
		}


		let mut command = format!("cd {} && ./{}", &path, &name).to_owned();
		let mut i = 0;

		while i < list_of_args.len() {
			command.push_str(&list_of_args[i]);
			i+=1;
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

	/*for line in contents.lines() {
		//need to check if valid command
		commands.push(line.to_string());
	}
	

	//Iterate over list of commands
	for command in commands {
		//let process = 
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

		println!(".");
	}*/
		println!("All done");

}