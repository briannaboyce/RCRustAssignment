use serde_yaml;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProcessLog {
	timestamp: SystemTime,
	username: String,
	process_name: String,
	command_line: String,
	pid: usize,

}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileLog {
	timestamp: SystemTime,
	file_path: String,
	activity: String,
	username: String,
	process_name: String,
	command_line: String,
	pid: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NetworkLog {
	timestamp: SystemTime,
	username: String,
	dest_addr: String,
	dest_port: String,
	src_addr: String, 
	src_port: String,
	data_amt: usize,
	protocol: String,
	process_name: String,
	command_line: String,
	pid: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct YamlLog {
	pub process_log: HashMap<String, ProcessLog>,
	file_log: HashMap<String, FileLog>,
	network_log: HashMap<String, NetworkLog>,
}

impl Default for YamlLog {
	fn default() -> YamlLog {
		YamlLog {
			process_log: HashMap::new(),
			file_log: HashMap::new(),
			network_log: HashMap::new(),
		}
	}
}


pub fn build_and_output_log() {

}

pub fn create_process_entry(timestamp: SystemTime, uname: String, pname: String, cmd: String, pid: usize) -> ProcessLog {

	let process_to_add: ProcessLog = ProcessLog {
		timestamp: timestamp,
		username: uname,
		process_name: pname, 
		command_line: cmd,
		pid: pid,
	};

	//println!("log so far {:?}", process_to_add);

	return process_to_add;
}

pub fn create_file_entry() {

}

pub fn create_network_entry() {

}