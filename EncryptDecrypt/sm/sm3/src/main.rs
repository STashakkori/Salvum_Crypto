// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

use libsm::sm3::hash::Sm3Hash;
use std::convert::TryInto;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;
fn main() {
	/*
		User passes in file with string data or a file with hash data to retrieve
		the hash.
	*/
	let args: Vec<String> = env::args().collect();
	if &args[1] == "-usage" {
		println!("cargo run [--] [arg] [file]");
		println!("args:\n-hash\n-gethash\n");
		return;
	}
	let h_gh_flag = &args[1];
	let file_name = &args[2];
	let mut file = match File::open(file_name) {
		Ok(file) => file,
			Err(err) => {
				println!("Unable to open specified file {} . Error: {}",file_name,err);
				return;
			}
	};

	let mut file_data = String::new();
	match file.read_to_string(&mut file_data) {
		Ok(num_byte) => num_byte,
			Err(err) => {
				println!("Unable to read file data to string. Error: {}",err);
				return;
			}
	};

	if h_gh_flag == "-hash" {
		let mut out_file = match File::create("hash.txt") {
			Ok(file) => file,
				Err(err) => {
					println!("Unable to create file \"hash.txt\". Error: {}",err);
					return;
				}
		};

		let hash = Sm3Hash::new(file_data.as_bytes());
		match write!(out_file,"{:?}\n{:?}\n{:?}\n",hash.digest,hash.length,hash.unhandle_msg) {
			Ok(output) => output,
				Err(err) => {
					println!("Unable to write hash data to file. Error: {}",err);
					return;
				}
		};
	}
	else if h_gh_flag == "-gethash" {
		file_data = (file_data.replace(" ","")).replace("[","");
		file_data = file_data.replace("]","");
		let lines:Vec<&str> = file_data.split("\n").collect();
		let l1: Vec<&str> = lines[0].split(",").collect();
		let l2 = match lines[1].parse::<u64>() {
			Ok(parse) => parse,
				Err(err) => {
					println!("Unable to parse structure length. Error: {}",err);
					return;
				}
		};
		let l3: Vec<&str> = lines[2].split(",").collect();
        let mut u32_l1 = Vec::new(); 
        for ele in l1.iter() {
          match ele.parse::<u32>() {
            Ok(val) => u32_l1.push(val),
              Err(err) => {
                println!("Unable to parse value to u32. Error: {}",err);
                return;
              }
          };
        }
        let mut u32_l3 = Vec::new();
        for ele in l3.iter() {
          match ele.parse::<u8>() {
            Ok(val) => u32_l3.push(val),
              Err(err) => {
                println!("Unable to parse value to u32. Error: {}",err);
                return;
              }
          };
        }
		let mut hash = Sm3Hash {digest: match u32_l1.try_into() {
                                          Ok(val) => val,
                                            Err(err) => {
                                              println!("Unable to get [u32; 8] slice. Error: {:?}",err);
                                              return;
                                            }
                                        }, length: l2, unhandle_msg: u32_l3};
		let digest: [u8;32] = hash.get_hash();					
		let mut out_file = match File::create("get_hash.txt") {
			Ok(file) => file,
				Err(err) => {
					println!("Unable to create file \"get_hash.txt\". Error: {}",err);
					return;
				}
		};
		match out_file.write(&digest) {
			Ok(output) => output,
				Err(err) => {
					println!("Unable to write to file \"get_hash.txt\". Error: {}",err);
					return;
				}
		};
	}
}

