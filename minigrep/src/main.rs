use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::fs;


fn main() {
	//println!("Hello World!");
	/*First collect input from a human user
		later we'll get the input from a file
		when the program starts, it should start with an argument to the file of commands
	*/
	let args: Vec<String> = env::args().collect();
	let mut commands: Vec<String> = Vec::new();

	let file = &args[1];

	let contents = fs::read_to_string(file)
		.expect("Some error happened");

	for line in contents.lines() {
		//need to check if valid command
		commands.push(line.to_string());
	}
	

//should iterate over commands to provide list of commands
	let command = &commands[0];
	println!("{}", command);

	let process = if cfg!(target_os = "windows") {
		Command::new("cmd")
				.args(&["/C", command])
				.output()
				.expect("failed to execute")
	} else {
		Command::new("sh")
				.arg(command)
				.output()
				.expect("failed to execute") 
	};

	let hello = std::string::String::from_utf8(process.stdout)
									.ok()
									.expect("failure");
	print!{"{}", hello};

}