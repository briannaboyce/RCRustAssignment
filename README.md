# RC Rust Assignment

A simple program built in Rust.

This program takes input from a YAML file and performs the following operations based on YAML input:

*	Starts an executable process
*	Creates a file
*	Edits a file
*	Deletes a file 
*	Transmit data over the network using either TCP or UDP

The program can be run from the `minigrep/src/` directory using `cargo run <input.yaml>`

A sample YAML file structure is included at `minigrep/src/test.yaml`

- The YAML file is divided into 5 objects:
	- process
	- create
	- update
	- delete
	- network

- Process object:
	- objectName
		- path: String
		- executableName: String
		- arguments: String (optional)

- Create object:
	- objectName
		- path: String
		- fileType: String
		- fileName: String
		- fileContent: String

- Update object:
	- objectName
		- path: String
		- fileType: String
		- fileName: String
		- fileContent: String

- Delete object:
	- objectName
		- path: String
		- fileType: String
		- fileName: String
		- fileContent: String (empty)
		
- Network object:
	- objectName
		- destAddr: String
		- destPort: String
		- protocol: String (TCP or UDP)
		- data: String

A log file is output in the current directory of operations taken. The file is output in YAML. 

Program runs on Linux, macOS, and Windows.
