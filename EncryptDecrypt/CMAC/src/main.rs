// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

use aes::Aes128;
use std::env;
use cmac::{Cmac, Mac, NewMac};
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
	let args :Vec<String> = env::args().collect();
	if args.len() == 1 || args[1] == "-h" {
		println!("Usage: ./CMAC <initial file> <update file>");
		println!("       ./CMAC -v <initial file> <update file> <tag bytes file>");
		return;
	}

	if args.len() > 5 {
		println!("To many arguments, expected 2 or 4, found {}",args.len());
		return;
	}
	else if args.len() < 3 {
		println!("To few arguments, expected 2 or 4, found {}",args.len());
		return;
	}

	let init_file;
	let upda_file;
	let u_buf : Vec<&str>;
	let i_buf;
	if args[1] == "-v" {
		init_file = &args[2];
		upda_file = &args[3];
		let tb_file = &args[4];
		let u_string = open_upda(upda_file);
		i_buf = open_init(init_file);
		let tb_buf = open_tb(tb_file);
		verify(&i_buf,u_string,&tb_buf);

	}

	else {
		init_file = &args[1];
		upda_file = &args[2];

		let u_string = open_upda(upda_file);
		u_buf = u_string.split("\n").collect();

		i_buf = open_init(init_file);

		let mut mac = match Cmac::<Aes128>::new_from_slice(&i_buf) {
			Ok(val) => val,
				Err(err) => {
					println!("Unable to initialize new MAC instance. Error: {:?}",err);
					return;
				}
		};


		for ele in u_buf.iter() {
			mac.update(ele.as_bytes());
		}

		let result = mac.finalize();


		let tag_bytes = result.into_bytes();
		let mut out_file = match File::create("tb.txt") {
			Ok(file) => file,
				Err(err) => {
					println!("Unable to create file \"tb.txt\". Error: {}",err);
					return;
				}
		};
		match out_file.write_all(&tag_bytes) {
			Ok(file) => file,
				Err(err) => {
					println!("Unable to write tag bytes to file \"tb.txt\". Error: {}",err);
					return;
				}
		};

	}
}


fn open_tb(tb_file: &str) -> Vec<u8> {
	let mut tb_file = match File::open(tb_file.trim()) {
		Ok(file) => file,
			Err(err) => {
				println!("Unable to open specified tag bytes file. Error: {}",err);
				return Vec::new();
			}
	};

	let mut tb_buf = Vec::new();
	match tb_file.read_to_end(&mut tb_buf) {
		Ok(data) => data,
			Err(err) => {
				println!("Unable to read tag bytes data. Error: {}",err);
				return Vec::new();
			}
	};
	tb_buf


}

fn open_upda(upda_file: &str) -> String {
	let mut u_file = match File::open(upda_file.trim()) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to open specified file update file. Error: {}", err);
				return "".to_string();
			}
	};

	let mut u_buf = String::new();
	match u_file.read_to_string(&mut u_buf) {
		Ok(num) => num,
			Err(err) => {
				println!("Unable to read update data file as bytes. Error: {}",err);
				return "".to_string();
			}
	};
	u_buf
}

fn open_init(init_file : &str) -> Vec<u8> {

	let mut i_file = match File::open(init_file.trim()) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to open specified file \"{}\". Error: {}", init_file, err);
				return Vec::new();
			}
	};

	let mut i_buf = Vec::new();
	match i_file.read_to_end(&mut i_buf) {
		Ok(num) => num,
			Err(err) => {
				println!("Unable to read file data as bytes. Error: {}",err);
				return Vec::new();
			}
	};

	i_buf.pop();
	i_buf
}

fn verify(i_data: &[u8], u_buf: String ,tb : &[u8]) {
	let mut mac = match Cmac::<Aes128>::new_from_slice(i_data) {
                    Ok(mac) => mac,
                      Err(err) => {
                        println!("Unable to extract mac from data slice. Error: {}",err);
                        return;
                      }
                  };
	let vec:Vec<&str> = u_buf.split("\n").collect();
	for ele in vec.iter() {
		mac.update(ele.as_bytes());
	}
	match mac.verify(tb) {
		Ok(_) => {
			println!("True");
		},
			Err(err) => {
				println!("Unable to verify tag bytes. Error: {:?}",err);
				return;
			}
	};
}	


