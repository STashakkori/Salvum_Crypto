// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use rand::prelude::*;
use rand::{Rng};
use rabbit::Rabbit;
fn main() {
    let args: Vec<String> = env::args().collect();
		
	if args.len() < 3 || args.len() > 7 {
		println!("  Usage: rabbit -operation  [-arguments] ");
		println!("                 -key     keyname");
		println!("                 -encrypt keyname -in filename -out filename");
		println!("                 -decrypt keyname -in filename -out filename");
		return; 
	}
	
	match args[1].trim() {
		"-key" => secure_keygen(args),
		"-encrypt" => encrypt(args),
		"-decrypt" => decrypt(args),
		_ => return,
	};
	
}

fn secure_keygen(vec: Vec<String>) {
	if vec.len() > 3 {return;}
	let keyname = vec[2].trim().to_string();
	let mut block: [u8; 16] = Default::default();
	
	println!("Generating random 16 byte key");
  let mut rng = StdRng::from_entropy();
  let mut vec_u8: Vec<u8> = Vec::new();
	
	for _i in 0..block.len() {
		let x: u8 = rng.gen();
		vec_u8.push(x);
	}
	
	block.copy_from_slice(&vec_u8);
	write_key(keyname.clone(), block);
	println!("found at ./{}", keyname);
}

fn encrypt(vec: Vec<String>) {
	if vec.len() > 7 || vec.len() < 7 {return;}
	let keyname = vec[2].trim().to_string();
	let in_name = vec[4].trim().to_string();	
	let out_name = vec[6].trim().to_string();
	
	let key = read_key(keyname);
	
	let mut rabbit = Rabbit::setup_without_iv(key);
	let mut input = read_file(in_name.clone());
	let mut startchunk = 0;
	let mut endchunk = 48;

	for i in 0..input.len() {
		let end = input.len();
		if i % 48 == 0 && i != 0 {
			rabbit.encrypt_inplace(&mut input[startchunk..endchunk]);
			startchunk = i;
			endchunk += 48;
		}
		rabbit.encrypt_inplace(&mut input[0..end]);
	}
	
	let mut output_file = match File::create(&out_name.clone()) {
		Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return;
			}
	};
	match output_file.write_all(&input) {
		Ok(an) => an,
			Err(err) => {
				println!("Failed to write. Error : {}", err);
				return;
			}	
	};
	println!("Done encrypting {} into ./{}",in_name, out_name);
}

fn decrypt(vec: Vec<String>) {
	if vec.len() > 7 || vec.len() < 7 {return;}
	let keyname = vec[2].trim().to_string();
	let in_name = vec[4].trim().to_string();	
	let out_name = vec[6].trim().to_string();
	
	let key = read_key(keyname);
	
	let mut rabbit = Rabbit::setup_without_iv(key);
	let mut input = read_file(in_name.clone());
	let mut startchunk = 0;
	let mut endchunk = 48;
	
	for i in 0..input.len() {
		let end = input.len();
		if i % 48 == 0 && i != 0 {
			rabbit.decrypt_inplace(&mut input[startchunk..endchunk]);
			startchunk = i;
			endchunk += 48;
		}
		rabbit.decrypt_inplace(&mut input[0..end]);
	}
	
	let mut output_file = match File::create(&out_name.clone()) {
		Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return;
			}
	};
	match output_file.write_all(&input) {
		Ok(an) => an,
			Err(err) => {
				println!("Failed to write. Error : {}", err);
				return;
			}	
	};
	println!("Done decrypting {} into ./{}", in_name, out_name);
}

fn read_file(filename: String) -> Vec<u8> {
	let mut buf = Vec::new();	
	let mut open_file = match File::open(&filename) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return buf;
		}
	};	
	match open_file.read_to_end(&mut buf) {
		Ok(b) => b,
		Err(err) => {
			println!("Unable to read data in as bytes. Error: {}",err);
			return buf;
		}
	};
	return buf;
}

fn write_key(filename: String, contents: [u8; 16]) {
	let mut output_file = match File::create(filename) {
		Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return;
			}
	};
	match output_file.write_all(&contents) {
		Ok(an) => an,
			Err(err) => {
				println!("Failed to write. Error : {}", err);
				return;
			}	
	};	
}

fn read_key(filename: String) -> [u8; 16] {
	
	let mut block: [u8; 16] = Default::default();
	let mut open_file = match File::open(&filename) {
		Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return block;
			}
	};	
	let mut buf = Vec::new();
	match open_file.read_to_end(&mut buf) {
		Ok(b) => b,
		Err(err) => {
			println!("Unable to read data in as bytes. Error: {}",err);
			return block;
		}
	};

	block.copy_from_slice(&buf);
	return block;
}
