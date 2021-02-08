use std::env;
use std::process::Command;
use std::fs;


fn main() {

	//Collect input from a file
	//Should limit file type to something specific like yaml, using txt for now
	let args: Vec<String> = env::args().collect();
	let mut commands: Vec<String> = Vec::new();

	let file = &args[1];

	let contents = fs::read_to_string(file)
		.expect("File does not exist");

	for line in contents.lines() {
		//need to check if valid command
		commands.push(line.to_string());
	}
	

	//Iterate over list of commands
	for command in commands {
		let process = if cfg!(target_os = "windows") {

			Command::new("cmd")
					.args(&["/C", &command])
					.output()
					.expect("failed to execute")
		} else {
			Command::new("sh")
					.arg("-c")
					.arg(&command)
					.output()
					.expect("failed to execute") 
		};

		let output = std::string::String::from_utf8(process.stdout)
										.ok()
										.expect("failure");

		//println!("process ID is {}", process::id());

		print!("{}", output);
		println!(".");
	}
		println!("All done");

}