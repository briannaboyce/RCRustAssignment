use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;


fn main() {
	//println!("Hello World!");
	/*First collect input from a human user
		later we'll get the input from a file
		when the program starts, it should start with an argument to the file of commands
	*/
	let args: Vec<String> = env::args().collect();

	let command = &args[1];
	let parameter = &args[2];

	let process = if cfg!(target_os = "windows") {
		Command::new("cmd")
				.args(&["/C", command, parameter])
				.output()
				.expect("failed to execute")
	} else {
		Command::new(command)
				.arg(parameter)
				.output()
				.expect("failed to execute") 
	};

	let hello = std::string::String::from_utf8(process.stdout)
									.ok()
									.expect("failure");
	print!{"{}", hello};

}