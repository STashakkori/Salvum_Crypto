// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

extern crate speck_rs;
use crate::speck_rs::Speck;
use crate::speck_rs::cipher_modes::ECB;
use std::fs::File;
use std::io::Write;
use std::env;
use std::io::Read;
use rand::{Rng};
use substring::Substring;
use rand::prelude::*;

fn main() {

	let args: Vec<String> = env::args().collect();
	
	if args.len() < 3 || args.len() > 7 {
			println!("  Usage: speck -operation  [-arguments] ");
			println!("               -key     keyname");
			println!("               -encrypt keyname -in filename -out filename");
			println!("               -decrypt keyname -in filename -out filename");
			return; 
	}
	match args[1].trim() {
		"-key" => secure_keygen(args),
		"-encrypt" => encrypt_block(args),
		"-decrypt" => decrypt_block(args),
		_ => return,
	};

}

fn secure_keygen(vec: Vec<String>) {
	if vec.len() > 3 {return;}
	let keyname = vec[2].trim().to_string();
	println!("Generating random 128-bit integer from kernel entropy or cpu jitter");
  let mut rng = StdRng::from_entropy();
  let x: u128 = rng.gen();
  println!("key: {}", x);
	write_file(keyname.clone(), x);
	println!("found at ./{}", keyname);
}

fn encrypt_block(vec: Vec<String>) {
	//println!("{}", vec.len());
	if vec.len() > 7 || vec.len() < 7 {return;}
	let keyname = vec[2].trim().to_string();
	let in_name = vec[4].trim().to_string();	
	let out_name = vec[6].trim().to_string();

	let keyb = read_file(keyname);
	let key: u128 = match keyb.parse::<u128>() {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to parse key. Error : {}", err);
			return;
		}	
	};
	println!("key: {}", key);

	let speck = Speck::new(&key);
	
	//Reading file in as u8
	let mut file = match File::open(&in_name) {
		Ok(f) => f,
		Err(err) => {
			println!("Unable to open specified file for reading. Error: {}",err);
			return;
		}			
	};

	let mut buf = Vec::new();
	match file.read_to_end(&mut buf) {
		Ok(b) => b,
		Err(err) => {
			println!("Unable to read data in as bytes. Error: {}",err);
			return;
		}
	};

	let mut output_file = match File::create(&out_name) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return;
		}
	};
	/////////////////////////////////////////////////////////
		let mut mark = 0;
    let mut iter = 0;
    
		for i in 0..buf.len() {
			
			let mut block: [u8; 16] = Default::default();
      
			if i % 16 == 0 && i != 0 {

				block.copy_from_slice(&buf[mark..i]);
				let hopegod = u128::from_be_bytes(block);
				let enc = speck.encrypt(&hopegod);
				let mut zerofix = enc.to_string().len();
				
				while zerofix < 39 {
					match write!(output_file, "0",) {
						Ok(x) => x,
						Err(e) => { 
							println!("Error: {}", e);
							return;
						}
					};
					zerofix+=1;	
				}
			
				match write!(output_file, "{}", enc) {
					Ok(x) => x,
					Err(e) => { 
						println!("Error: {}", e);
						return;
					}
				};
				mark+= 16;
				
				
      }
      iter += 1;
    }
		//////////////////////////////////////////////////////////	

	for mark in mark..iter {
		
		let enc = speck.encrypt(&(buf[mark] as u128));
		let mut zerofix = enc.to_string().len();
		
		while zerofix < 39 {
			match write!(output_file, "0",) {
				Ok(x) => x,
				Err(e) => { 
					println!("Error: {}", e);
					return;
				}
			};
			zerofix+=1;
		}
			
		match write!(output_file, "{}", enc) {
			Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return;
			}
		};
	}
	println!("All done encrypting {} into ./{}", &in_name, &out_name);	
}

fn decrypt_block(vec: Vec<String>) {
	if vec.len() > 7 {return;}
	let keyname = vec[2].trim().to_string();
	let in_name = vec[4].trim().to_string();	
	let out_name = vec[6].trim().to_string();

	let keyb = read_file(keyname);
	let key: u128 = match keyb.parse::<u128>() {
		Ok(an) => an,
		Err(err) => {
			println!("Failed to parse key. Error : {}", err);
			return;
		}	
	};
	
	println!("key: {}", key);	
	let speck = Speck::new(&key);
	
	//Reading file in as string
	let mut file = match File::open(&in_name) {
		Ok(f) => f,
		Err(err) => {
			println!("Unable to open specified file for reading. Error: {}",err);
			return;
		}			
	};

	let mut buf = String::new();
	match file.read_to_string(&mut buf) {
		Ok(b) => b,
		Err(err) => {
			println!("Unable to read data in as bytes. Error: {}",err);
			return;
		}
	};
	
	let mut output_file = match File::create(&out_name) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return;
		}
	};

	let mut counter = 0;
	let mut ilast = 0;

	for i in 0..buf.len() {
		if i > 0 && i % 39 == 0 {

			let loopy = buf.substring(counter,i);
			let loopstr: u128 = match loopy.parse::<u128>() {
				Ok(x) => x,
				Err(e) => { 
					println!("loop Error: {}", e);
					return;
				}
			};

			let blockenc = speck.decrypt(&loopstr);
			let bytes = blockenc.to_be_bytes();

			if bytes[0] != 0 {
				match output_file.write_all(&bytes) {
					Ok(x) => x,
					Err(e) => { 
						println!("Error: {}", e);
						return;
					}
				};
			}
			else {
				match output_file.write_all(&[bytes[15]]) {
					Ok(x) => x,
					Err(e) => { 
						println!("Error: {}", e);
						return;
					}
				};
			}
			counter += 39;
		}
		ilast += 1;
	}
	
	let last = buf.substring(counter, ilast);

	let laststr: u128 = match last.parse::<u128>() {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return;
		}
	};

	let blockenc = speck.decrypt(&laststr);
	let bytes = blockenc.to_be_bytes();
	
	match output_file.write_all(&[bytes[15]]) {
		Ok(x) => x,
		Err(e) => { 
			println!("Error: {}", e);
			return;
		}
	};
	println!("Done decrypting {} into {}", &in_name,&out_name); 
}

fn write_file(filename: String, contents: u128) {
	let mut output_file = match File::create(filename) {
		Ok(x) => x,
			Err(e) => { 
				println!("Error: {}", e);
				return;
			}
	};
	match write!(output_file, "{}", contents) {
		Ok(an) => an,
			Err(err) => {
				println!("Failed to write. Error : {}", err);
				return;
			}	
	};	
}

fn read_file(filename: String) -> String {
	let mut open_file  = match File::open(&filename) {
		Ok(input) => input,
			Err(err) => {
				println!("Failed to open file Error : {}", err);
				return err.to_string();
			}
	};
	let mut temp = String::new();
	match open_file.read_to_string(&mut temp) {
		Ok(input) => input,
			Err(err) => {
				println!("Failed to read file, Error : {}", err);
				return err.to_string();
			}
	};
	return temp;
}
