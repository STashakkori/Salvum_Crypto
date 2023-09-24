// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

/*
	 This application can encrypt and decrypt hex data based on the MAGMA
	 symmetric-key block cipher.
 */

use magma::{Magma, BlockEncrypt, BlockDecrypt, NewBlockCipher};
use rand_core::{RngCore,OsRng};
use std::fs::File;
use std::env;
use std::io::Read;
use std::io::Write;

fn main() {

	let args: Vec<String> = env::args().collect();
	if args.len() == 1 || args[1] == "-h" {
		println!("Usage: ./MAGMA -e <data file>");
		println!("       ./MAGMA -d <encrypted file> <key file>");
		return;
	}
	else if args.len() > 4 {
		println!("Expected less arguments, found {}",args.len());
		return;
	}
	else if args.len() < 3 {
		println!("Expected more arguments, found {}",args.len());
		return;
	}
	let cmd = (&args[1]).trim();
	let in_file = (&args[2]).trim();
	let k_file;

	if cmd == "-d" {
		k_file = (&args[3]).trim();
		let k_vec = read_key(k_file);
		if k_vec.len() == 0 {return;}
		let mut enc_data = read_enc(in_file);
		let cipher = Magma::new((&k_vec[..32]).into());
		let mut mark = 0; 
		for i in 1..enc_data.len()+1 {
			if i % 8 == 0 {
				cipher.decrypt_block((&mut enc_data[mark..i]).into());
				mark = i;
			}
		}
		let rem_enc_data = remove_trail(&enc_data);
        if rem_enc_data.len() == 0 {return;}

		write_decrypt(rem_enc_data);
	}
	if cmd == "-e" {

		let mut key = [0u8; 32];
		OsRng.fill_bytes(&mut key);
		write_key(&key);
		let mut data_s = read_data(in_file);
		if data_s.len() == 0 {return;};
		let cipher = Magma::new(&(key.into()));

		encrypt(&mut data_s,cipher);
	}		
}

/*
	remove_trail removes the trailing zeros in a decrypted
	message.
*/
fn remove_trail(vec: &Vec<u8>) -> Vec<u8> {
	let mut res = match std::str::from_utf8(&vec) {
                    Ok(res) => res,
                      Err(err) => {
                        println!("Unable to convert slice of bytes to string slice. Error: {}",err);
                        return Vec::new();
                      }
                  };
	res = res.trim_end_matches(char::from(0));
	res.as_bytes().to_vec()
}

/*
	 write_decrypt will take the decrypted data and write it to
	 the file decrypt.txt.
 */
fn write_decrypt(hex: Vec<u8>) {
	let mut file = match File::create("decrypt.txt") {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create decrypt.txt file. Error: {}",err);
				return;
			}
	};
	match file.write_all(&hex) {
		Ok(r) => r,
			Err(err) => {
				println!("Unable to write decrypted bytes to file. Error: {}",err);
				return;
			}
	};
}

/*
	 read_data will take in the data file name and reads in the 
	 data as bytes which is then returned.
 */
fn read_data(in_file: &str) -> Vec<u8> {

	let mut file = match File::open(in_file) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to open specified file. Error: {}",err);
				return Vec::new();
			}
	};
	let mut buf = Vec::new();
	match file.read_to_end(&mut buf) {
		Ok(r) => r,
			Err(err) => {
				println!("Unable to read data to string. Error: {}",err);
				return Vec::new();
			}
	};
	buf
}

/*
	 read_enc will take in an encrypted file name and reads in the
	 data as bytes which is then returned.
 */
fn read_enc(enc_file: &str) -> Vec<u8> {

	let mut file = match File::open(enc_file) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to open encrypted.txt file. Error: {}",err);
				return Vec::new();
			}
	};
	let mut buf = Vec::new();
	match file.read_to_end(&mut buf) {
		Ok(r) => r,
			Err(err) => {
				println!("Unable to read data as bytes. Error: {}",err);
				return Vec::new();
			}
	};
	buf
}

/*
	 encrypt will take in the data vec that can then be passed to the 
	 encrypt function using the Cast5 cipher reference. This encrypted
	 data will then be written to the file encrypted.txt.
 */
fn encrypt(buf:&mut Vec<u8>, cipher: Magma) {
	let mut out_file = match File::create("encrypted.txt") {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create encrypted.txt file. Error: {}",err);
				return;
			}
	};

	let mut hex_vec = Vec::new();
	for i in buf.iter() {
		if hex_vec.len() == 8 {
			cipher.encrypt_block((&mut hex_vec[..]).into());
			match out_file.write_all(&hex_vec) {
				Ok(b) => b,
					Err(err) => {
						println!("Unable to write encrypted bytes to file. Error: {}",err);
						return;
					}
			};
			hex_vec = Vec::new();
		}
		hex_vec.push(*i);
	}
	if hex_vec.len() != 0 {
		pad_hex_vec(&mut hex_vec);
		
		cipher.encrypt_block((&mut hex_vec[..]).into());
		match out_file.write_all(&hex_vec) {
			Ok(b) => b,
				Err(err) => {
					println!("Unable to write encrypted bytes to file. Error: {}",err);
					return;
				}
		};

	}
}
	

/*
	 pad_hex_vec will take in a vector and pad it with 
	 zeros so the new length will be eight.
 */
fn pad_hex_vec(vec: &mut Vec<u8>) {
	let pad = 8 - vec.len();
	for _ in 0..pad {
		vec.push(0 as u8);
	}
}

/*
	 read_key will take in the file name and read in the 
	 data as bytes which will then by returned
 */
fn read_key(file : &str) -> Vec<u8> {
	let mut k_file = match File::open(file) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to open specified key file. Error: {}",err);
				return Vec::new();
			}
	};
	let mut key_vec = Vec::new();
	match k_file.read_to_end(&mut key_vec) {
		Ok(nb) => nb,
			Err(err) => {
				println!("Unable to read bytes in file. Error: {}",err);
				return Vec::new();
			}
	};
	key_vec
}

/*
	 write_key will write the generated 128 bit key to the file
	 key.txt
 */
fn write_key(key: &[u8; 32]) {
	let mut out_file = match File::create("key.txt") {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create key.txt file. Error: {}",err);
				return;
			}
	};

	match out_file.write_all(key) {
		Ok(b) => b,
			Err(err) => {
				println!("Unable to write key to file key.txt. Error: {}",err);
				return;
			}
	};
}
