use std::env;
use std::fs;
use std::process::Command;
use serde_yaml;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::Write;
use std::fs::OpenOptions;
use std::net::TcpStream;
use std::path::Path;
use std::io::Read;
use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};

mod log;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Process {
	path: String,
	name: String,
	arguments: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct FileInfo {
	path: Option<String>,
	file_type: String,
	name: String,
	content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct NetworkConnection {
	dest_addr: String,
	dest_port: String,
	protocol: String,
	data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct InputFile {
	process: HashMap<String, Process>,
	create: HashMap<String, FileInfo>,
	update: HashMap<String, FileInfo>,
	delete: HashMap<String, FileInfo>,
	network: HashMap<String, NetworkConnection>,
}

fn main() {

	//Create log
	let mut log_file = log::YamlLog::default();

	//Collect input from a yaml file
	let args: Vec<String> = env::args().collect();

	let file = &args[1];
	let contents = fs::read_to_string(file)
		.expect("File does not exist");

	let yaml_data: InputFile = serde_yaml::from_str::<InputFile>(&contents).unwrap();

	//Setup and run any processes as defined in YAML
	let processes = yaml_data.process;
	let processes_to_log: HashMap<String, log::ProcessLog> = execute_process(processes);
	log_file.process_log = processes_to_log;

	let mut files_to_log: HashMap<String, log::FileLog> = HashMap::new();

	//Create any new files as described by the YAML file
	let files_to_create = yaml_data.create;
	files_to_log = create_file(files_to_create, files_to_log);

	//Update file
	let files_to_update = yaml_data.update;
	files_to_log = update_file(files_to_update, files_to_log);

	//Delete file
	let files_to_delete = yaml_data.delete;
	files_to_log = delete_file(files_to_delete, files_to_log);

	//Add all files to log
	log_file.file_log = files_to_log;
	

	//Network data transmission
	let network_operations = yaml_data.network;
	let network_to_log: HashMap<String, log::NetworkLog> = transmit_data(network_operations);
	log_file.network_log = network_to_log;

	println!("log so far {:?}", log_file);

	println!("All done");

}

fn execute_process(processes: HashMap<String, Process>) -> HashMap<String, log::ProcessLog> {
	let mut log_of_processes: HashMap<String, log::ProcessLog> = HashMap::new();
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

		//Log the processes
		let log_process_name = format!("{}", _name);
		//TODO populate with the real data
		log_of_processes.insert(log_process_name, log::create_process_entry(SystemTime::now(), 
																	"bri".to_string(), 
																	"process".to_string(), 
																	"command".to_string(), 
																	12));
	}

	return log_of_processes;
}

fn create_file(files_to_create: HashMap<String, FileInfo>, file_log: HashMap<String, log::FileLog>) -> HashMap<String, log::FileLog> {
	let mut file_log = file_log;
	//Iterate through any files that need creation
	for (_name, file) in files_to_create {

		//Get file data
		let path = file.path;
		let file_type = file.file_type;
		let name = file.name;
		let content = file.content;

		let name_for_file = get_file_path(name, file_type, path);

		let mut new_file = OpenOptions::new()
							.create(true)
							.append(true)
							.open(name_for_file)
							.expect("creation failed");
		new_file.write_all(content.as_bytes()).expect("write failed");
		//Log the processes
		let log_file_name = format!("{}", _name);
		//TODO populate with the real data
		file_log.insert(log_file_name, log::create_file_entry(SystemTime::now(),
																		"path/to/file/".to_string(),
																		"create".to_string(), 
																		"bri".to_string(), 
																		"process".to_string(), 
																		"command".to_string(), 
																		12));
	}

	return file_log;

}

fn update_file(files_to_update: HashMap<String, FileInfo>, file_log: HashMap<String, log::FileLog>) -> HashMap<String, log::FileLog> {
	let mut file_log = file_log;
	//Iterate through any files that need updating
	//TODO should check if we want to overwrite or append to file
	//TODO should check if extension for file should change
	//TODO should check if old file extension should be kept as well as new file
	//See here: https://www.linuxjournal.com/content/getting-started-rust-working-files-and-doing-file-io
	for(_name, file) in files_to_update {
		//File data
		let path = file.path;
		let file_type = file.file_type;
		let name = file.name;
		let content = file.content;

		let name_for_file = get_file_path(name, file_type, path);

		let mut update_file = OpenOptions::new()
							.append(true)
							.open(name_for_file)
							.expect("update failed");
		update_file.write_all(content.as_bytes()).expect("write failed");

		//Log the processes
		let log_file_name = format!("{}-update", _name);
		//TODO populate with the real data
		file_log.insert(log_file_name, log::create_file_entry(SystemTime::now(),
																		"path/to/file/".to_string(),
																		"update".to_string(), 
																		"bri".to_string(), 
																		"process".to_string(), 
																		"command".to_string(), 
																		12));
	}
	return file_log;
}

fn delete_file(files_to_delete: HashMap<String, FileInfo>, file_log: HashMap<String, log::FileLog>) -> HashMap<String, log::FileLog> {
	let mut file_log = file_log;
	//Iterate through any files that need deleting
	for(_name, file) in files_to_delete {
		let path = file.path;
		let file_type = file.file_type;
		let name = file.name;

		let name_for_file = get_file_path(name, file_type, path);

		fs::remove_file(name_for_file).expect("deletion failed");

		//Log the processes
		let log_file_name = format!("{}-delete", _name);
		//TODO populate with the real data
		file_log.insert(log_file_name, log::create_file_entry(SystemTime::now(),
																		"path/to/file/".to_string(),
																		"delete".to_string(), 
																		"bri".to_string(), 
																		"process".to_string(), 
																		"command".to_string(), 
																		12));
	}
	return file_log;
}

fn get_file_path(name: String, file_type: String, path: Option<String>) -> String {

		//Check if path was provided
		//If so, use that
		//If not, use current directory
	let mut name_for_file =  format!("{}.{}", name, file_type);

		if let Some(specific_path) = &path {
			name_for_file = format!("{}{}.{}", *specific_path, name, file_type);
		}
		return name_for_file;
}

fn transmit_data(network_operations: HashMap<String, NetworkConnection>) -> HashMap<String, log::NetworkLog> {
	let mut log_of_networks: HashMap<String, log::NetworkLog> = HashMap::new();
	//Iterate through any network connections and transmission
	for(_name, network_op) in network_operations {
		let protocol = network_op.protocol;
		let dest_addr = network_op.dest_addr;
		let dest_port = network_op.dest_port;
		let data = network_op.data;

		match protocol.trim() {
			"tcp" => {
				let destination = format!("{}:{}", dest_addr, dest_port);
				println!("{}", destination);
				let mut stream = TcpStream::connect(destination).expect("Connection failed");

				//TODO need to check if there is a path to a file or just regular data
				//handle errors more gracefully
				let path = Path::new(&data);
			    let file_name = path.file_name().unwrap();
			    println!("File name: {:?}", file_name);

			    let mut file = std::fs::File::open(path).expect("failure");
			    let file_size = file.metadata().unwrap().len();
			    println!("File size: {}", file_size);

			    let mut buffer = vec![0; file_size as usize];
			    let read_amt = file.read(&mut buffer).expect("read fail");
			    println!("Bytes read from file: {}", read_amt);

				stream.write(&buffer).expect("write failed");
			},
			"udp" => {
				let destination = format!("{}:{}", dest_addr, dest_port);
				println!("{}", destination);

				//Automatically bind UDP socket
				let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");

				let path = Path::new(&data);
			    let file_name = path.file_name().unwrap();
			    println!("File name: {:?}", file_name);

			    let mut file = std::fs::File::open(path).expect("failure");
			    let file_size = file.metadata().unwrap().len();
			    println!("File size: {}", file_size);

			    let mut buffer = vec![0; file_size as usize];
			    let read_amt = file.read(&mut buffer).expect("read fail");
			    println!("Bytes read from file: {}", read_amt);

			    socket.connect(destination).expect("connect failed");
				socket.send(&buffer).expect("send failed");
			}
			_ => {},
		} 

		//Log the processes
		let log_network_name = format!("{}", _name);
		//TODO populate with the real data
		log_of_networks.insert(log_network_name, log::create_network_entry(SystemTime::now(), 
																	"bri".to_string(), 
																	dest_addr,
																	dest_port,
																	"127.0.0.1".to_string(),
																	"0000".to_string(),
																	31,
																	protocol,
																	"process".to_string(), 
																	"command".to_string(), 
																	12));
	}

	return log_of_networks;
}