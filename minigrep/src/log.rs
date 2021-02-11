extern crate chrono;

use serde_yaml;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::Write;
use std::fs::OpenOptions;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

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
	pub file_log: HashMap<String, FileLog>,
	pub network_log: HashMap<String, NetworkLog>,
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


pub fn build_and_output_log(yaml_log: YamlLog)  {
	let content = serde_yaml::to_string(&yaml_log).expect("to string failed");

	let timestamp: SystemTime = SystemTime::now();
	let d = UNIX_EPOCH + Duration::from_secs(timestamp.secs_since_epoch);
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();

    let log_name = format!("/home/bri/Desktop/log-{}.yaml", timestamp_str);

	let mut new_file = OpenOptions::new()
							.create(true)
							.append(true)
							.open("/home/bri/Desktop/log.yaml")
							.expect("creation failed");

	new_file.write_all(content.as_bytes()).expect("write failed");

}

pub fn create_process_entry(timestamp: SystemTime, uname: String, 
							pname: String, cmd: String, pid: usize) -> ProcessLog {
	//Creates a log entry of a process
	let process_to_add: ProcessLog = ProcessLog {
		timestamp: timestamp,
		username: uname,
		process_name: pname, 
		command_line: cmd,
		pid: pid,
	};

	return process_to_add;
}

pub fn create_file_entry(timestamp: SystemTime, path: String, 
						 activity: String, uname: String, 
						 pname: String, cmd: String, 
						 pid: usize) -> FileLog {

	let file_to_add: FileLog = FileLog {
		timestamp: timestamp,
		file_path: path,
		activity: activity,
		username: uname,
		process_name: pname,
		command_line: cmd, 
		pid: pid,
	};

	return file_to_add;

}

pub fn create_network_entry(timestamp: SystemTime, uname: String,
							daddr: String, dport: String,
							saddr: String, sport: String,
							size: usize, protocol: String,
							pname: String, cmd: String,
							pid: usize) -> NetworkLog {

	let network_to_add: NetworkLog = NetworkLog {
		timestamp: timestamp,
		username: uname,
		dest_addr: daddr,
		dest_port: dport,
		src_addr: saddr,
		src_port: sport,
		data_amt: size,
		protocol: protocol,
		process_name: pname,
		command_line: cmd,
		pid: pid,
	};

	return network_to_add;

}