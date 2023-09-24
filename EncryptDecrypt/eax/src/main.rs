// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::io::Read;
use aes::Aes256;
use eax::Eax;
use eax::aead::{Aead, NewAead, generic_array::GenericArray};
use rand::prelude::*;
use rand;
use rand::rngs::OsRng;

fn main() {
  let args: Vec<String> = env::args().collect();

	if args.len() < 3 || args.len() > 7 {
		println!("  Usage: eax -operation  [-arguments] ");
		println!("             -key     keyname");
		println!("             -encrypt keyname -in filename -out filename");
		println!("             -decrypt keyname -in filename -out filename");
		return; 
	}
	
	match args[1].trim() {
		"-key" => secure_keygen(args),
		"-encrypt" => encrypt(args),
		"-decrypt" => decrypt(args),
		_ => return,
	};
	
}

fn encrypt(vec: Vec<String>) {
	if vec.len() > 7 || vec.len() < 7 {return;}
	let keyname = vec[2].trim().to_string();
	let in_name = vec[4].trim().to_string();	
	let out_name = vec[6].trim().to_string();
	
	let keyparse = read_key(keyname);
	let key = GenericArray::from_slice(&keyparse);

	let cipher = Eax::<Aes256>::new(key);
	let nonce = makenonce();
	let nonce = GenericArray::from_slice(&nonce); // 128-bits; unique per message
	
	let input = read_in_as_bytes(in_name.clone());

	let ciphertext = match cipher.encrypt(nonce, input.as_ref()) {
		Ok(f) => f,
		Err(err) => {
			println!("Encryption Error: {}",err);
			return;
		}			
	};
	write_out_as_bytes(out_name.clone(), ciphertext);
	println!("Done encypting {} into ./{}", in_name, out_name);
}

fn decrypt(vec: Vec<String>) {
	if vec.len() > 7 || vec.len() < 7 {return;}
	let keyname = vec[2].trim().to_string();
	let in_name = vec[4].trim().to_string();	
	let out_name = vec[6].trim().to_string();
	
	let keyparse = read_key(keyname);
	let key = GenericArray::from_slice(&keyparse);

	let cipher = Eax::<Aes256>::new(key);
	let nonche = read_nonce();
	let nonce = GenericArray::from_slice(&nonche);

	
	let input = read_in_as_bytes(in_name.clone());

	let plaintext = match cipher.decrypt(nonce, input.as_ref()) {
		Ok(f) => f,
		Err(err) => {
			println!("Decryption Error: {}",err);
			return;
		}			
	};
 
	write_out_as_bytes(out_name.clone(), plaintext);
	remove_nonce();
	println!("Done decypting {} into ./{}", in_name, out_name);
}
 
fn makenonce() -> [u8; 16] {
  let mut nonce = [0u8; 16];
	OsRng.fill_bytes(&mut nonce);
	write_nonce("nonce".to_string(), nonce);
	return nonce;
}

fn secure_keygen(vec: Vec<String>) {
	if vec.len() > 3 {return;}
	let keyname = vec[2].trim().to_string();


  let mut key = [0u8; 32];
	OsRng.fill_bytes(&mut key);
	println!("key: {:?}", key);
	write_key(keyname.clone(), key);
	println!("found at ./{}", keyname);
} 

fn write_out_as_bytes(filename: String, vec: Vec<u8>) {
	let mut output_file = match File::create(filename) {
		Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return;
			}
	};
	match output_file.write_all(&vec) {
		Ok(an) => an,
			Err(err) => {
				println!("Failed to write. Error : {}", err);
				return;
			}	
	};
}

fn read_in_as_bytes(filename: String) -> Vec<u8> {
	let mut buf = Vec::new();
	let mut file = match File::open(&filename) {
			Ok(f) => f,
			Err(err) => {
				println!("Unable to open specified file for reading. Error: {}",err);
				return buf;
			}			
		};
	match file.read_to_end(&mut buf) {
		Ok(b) => b,
		Err(err) => {
			println!("Unable to read data in as bytes. Error: {}",err);
			return buf;
		}
	};
	return buf;
}

fn write_key(filename: String, contents: [u8; 32]) {
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

fn read_key(filename: String) -> [u8; 32] {
	let mut key = [0u8; 32];	
	let mut open_file = match File::open(&filename) {
		Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return key;
			}
	};	
	let mut buf = Vec::new();
	match open_file.read_to_end(&mut buf) {
		Ok(b) => b,
		Err(err) => {
			println!("Unable to read data in as bytes. Error: {}",err);
			return key;
		}
	};
	key.copy_from_slice(&buf[..]);
	return key;
}

fn write_nonce(filename: String, contents: [u8; 16]) {
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

fn read_nonce() -> [u8; 16] {
	let mut key = [0u8; 16];	
	let mut open_file = match File::open("nonce".to_string()) {
		Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return key;
			}
	};	
	let mut buf = Vec::new();
	match open_file.read_to_end(&mut buf) {
		Ok(b) => b,
		Err(err) => {
			println!("Unable to read data in as bytes. Error: {}",err);
			return key;
		}
	};
	key.copy_from_slice(&buf[..]);
	return key;
}

fn remove_nonce() {
	match fs::remove_file("nonce".to_string()) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return;
		}
	};
}