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

	//Collect input from a file
	//Should limit file type to something specific like yaml, using txt for now
	let args: Vec<String> = env::args().collect();
	let mut commands: Vec<String> = Vec::new();

	let file = &args[1];
	let contents = fs::read_to_string(file)
		.expect("File does not exist");

	println!("{}", contents);

	let yaml_data = serde_yaml::from_str::<InputFile>(&contents);
	yaml_data.process[0];

	println!("{:?}", yaml_data);

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