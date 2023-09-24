// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

use libsm::sm4::{Mode, Cipher};
use rand::rngs::OsRng;
use rand::RngCore;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("Error, must include source file path or message to be encrypted.");
		return;
	}
	
	let key = rand_block();
	let mut encdecflag = false;
	let plain_text = String::from(args[1].trim());
	let mut plain_text_filepath = String::new();
	let mut plain_text_ext = String::new();	
	let check_str = String::from("sm4");

	if plain_text.contains(".") { // if input contains . then were reading from a file
		let outdump: Vec::<&str> = plain_text.split(".").collect();
		plain_text_ext = outdump[1].trim().to_string();
		plain_text_filepath = outdump[0].trim().to_string();
		encdecflag = true;
	}
	
	if encdecflag == true { // file has an extension
		
		if check_str == plain_text_ext {  //matches .sm4 so we decrypt
			
			let mut file_for_decrypt = match File::open(plain_text.clone()) {
				Ok(input) => input,
				Err(err) => {
					println!("Failed to parse input filepath, Error : {}", err);
					return;
				}
			};
			
			let mut decrypt_buffer = Vec::<u8>::new();
			match file_for_decrypt.read_to_end(&mut decrypt_buffer) {
				Ok(data) => data,
				Err(err) => {
					println!("Failed to read into buffer, Error : {}", err);
					return;
				}
			};
			
      let mut file_for_key = match File::open("keyvector") {
				Ok(input) => input,
				Err(err) => {
					println!("Failed to parse input filepath, Error : {}", err);
					return;
				}
			};
			
			let mut key_buffer = Vec::<u8>::new();
			match file_for_key.read_to_end(&mut key_buffer) {
				Ok(i) => i,
				Err(err) => {
					println!("Failed to read into buffer, Error : {}", err);
					return;
				}
			};

			let cipher = Cipher::new(&key_buffer, Mode::Cfb);
			let plain_text_return: Vec<u8> = cipher.decrypt(&decrypt_buffer[..], &key_buffer);
			plain_text_filepath.push_str(".txt");

			//let s = String::from_utf8_lossy(&plain_text_return[..]);

			let mut out_fname = match File::create(plain_text_filepath.clone()) {
				Ok(file) => file,
				Err(err) => {
					println!("unable to create file Err: {}", err);
					return;
				}
			};	
		
			match out_fname.write(&plain_text_return) {
				Ok(file) => file,
				Err(err) => {
					println!("unable to create file Err: {}", err);
					return;
				}
			};
			
			println!("Success, your .sm4 has been decrypted into .txt");
			
		}  //we need to encrypt, we were passed in a file that has an extension so we open read encrypt and write
		else { 
			
			let cipher = Cipher::new(&key, Mode::Cfb);
			
			let mut file_for_encrypt = match File::open(plain_text.clone()) {
			Ok(input) => input,
			Err(err) => {
				println!("Failed to parse input filepath, Error : {}", err);
				return;
				}
			};			

			let mut encrypt_buffer = Vec::new();
			match file_for_encrypt.read_to_end(&mut encrypt_buffer) {
				Ok(input) => input,
				Err(err) => {
					println!("Failed to read into buffer, Error : {}", err);
					return;
				}
			};

			let cipher_text: Vec<u8> = cipher.encrypt(&encrypt_buffer, &key);
			plain_text_filepath.push_str("-out.sm4");
		    	
			let mut out_fname = match File::create(plain_text_filepath) {
				Ok(file) => file,
				Err(err) => {
					println!("unable to create file Err: {}", err);
					return;
				}
			};
			
			match out_fname.write(&cipher_text) {
				Ok(file) => file,
				Err(err) => {
					println!("unable to create file Err: {}", err);
					return;
				}
			};
			
			let mut out_keyname = match File::create("keyvector") {
				Ok(file) => file,
				Err(err) => {
					println!("unable to create file Err: {}", err);
					return;
				}
			};

			match out_keyname.write(&key) {
				Ok(file) => file,
				Err(err) => {
					println!("unable to create file Err: {}", err);
					return;
				}
			};			
			
			println!("Success, your -out.sm4 and keyvector have been successfully created");
		}
	}
	else {	// we encrypt the input message and create a new file and write to there
 		
		let mut cipher_out_fname = match File::create("cyphertext.sm4") {
			Ok(file) => file,
			Err(err) => {
				println!("unable to create file Err: {}", err);
				return;
			}
		};
		
		let cipher = Cipher::new(&key, Mode::Cfb);	
		let cipher_text: Vec<u8> = cipher.encrypt(&plain_text.as_bytes(), &key);
		    	
		match cipher_out_fname.write(&cipher_text) {
			Ok(file) => file,
			Err(err) => {
				println!("unable to create file Err: {}", err);
				return;
			}
		};
			
		let mut out_keyname = match File::create("keyvector") {
			Ok(file) => file,
			Err(err) => {
				println!("unable to create file Err: {}", err);
				return;
			}
		};

		match out_keyname.write(&key) {
			Ok(file) => file,
			Err(err) => {
				println!("unable to create file Err: {}", err);
				return;
			}
		};	
		
		println!("Successfully encrypted your input into cyphertext.sm4, and keyvector.");
	}
}

fn rand_block() -> [u8; 16] {
    let mut block: [u8; 16] = [0; 16];
    OsRng.fill_bytes(&mut block[..]);
    block
}

