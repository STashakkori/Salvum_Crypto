// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

/*
	 This application can encrypt and decrypt data based on the aes_gcm 256 bit
	 symmetric-key block cipher.
 */

use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand_core::{RngCore,OsRng};
use std::fs::File;
use std::env;
use std::io::Read;
use std::io::Write;

fn main() {

	let args: Vec<String> = env::args().collect();
	if args.len() == 1 || args[1] == "-h" {
		println!("Usage: ./aes_gcm_256 -e <data file> <output key path> <output nonce path> <output payload path>");
		println!("       ./aes_gcm_256 -d <encrypted file> <key file> <nonce file> <output payload path>");
		return;
	}
	else if args.len() > 6 {
		println!("Expected less arguments, found {}",args.len());
		return;
	}
	else if args.len() < 6 {
		println!("Expected more arguments, found {}",args.len());
		return;
	}
	let cmd = (&args[1]).trim();
	let in_file = (&args[2]).trim();
	let k_file;
	let n_file;

	if cmd == "-d" {

		k_file = (&args[3]).trim();
		n_file = (&args[4]).trim();

		let k_vec = read_key(k_file);
		if k_vec.len() == 0 {return;}

		let nonce_d = read_nonce(n_file);
		if nonce_d.len() != 12 {return;}
		let nw_nonce_d = Nonce::from_slice(&nonce_d);
		
		let enc_data = read_enc(in_file);
		
		let cipher = match Aes256Gcm::new_from_slice(&k_vec) {
			Ok(c) => c,
			Err(err) => {
				println!("Unable to get cipher from key vector. Error: {}",err);
				return;
			}
		};

		let plaintext = match cipher.decrypt(nw_nonce_d, enc_data.as_ref()) {
      Ok(decrypted_text) => decrypted_text,
      Err(err) => {
        println!("Failed to decrypt. {}",err.to_string());
        return;
      }
    };
    let payload_path = (&args[5]).trim().to_string();
		write_decrypt(plaintext, payload_path);

	}

	else if cmd == "-e" {
		let mut key = [0u8; 32];
		let mut nonce = [0u8; 12];
    let nonce_path = (&args[4]).trim().to_string();
		OsRng.fill_bytes(&mut key);
		OsRng.fill_bytes(&mut nonce);

		write_nonce(&nonce, nonce_path);
		let nw_nonce = Nonce::from_slice(&nonce);
    let key_path = (&args[3]).trim().to_string();
		write_key(&key, key_path);
			
		let data_s = read_data(in_file);
		if data_s.len() == 0 {return;};

		let cipher = Aes256Gcm::new(&(key.into()));
		let ciphertext = match cipher.encrypt(nw_nonce, data_s.as_ref()) {
      Ok(encrypted_text) => encrypted_text,
      Err(err) => {
        println!("Failed to encrypt. {}",err.to_string());
        return;
      }
    };
    let payload_path = (&args[5]).trim().to_string();
		write_encrypt(&ciphertext, payload_path);
	}		
	
	else {
		println!("Unrecognized command. Options : \"-e\" or \"-d\"");
	}
}

/*
	 write_decrypt will take the decrypted data and write it to
	 the file decrypt.txt.
 */
fn write_decrypt(hex: Vec<u8>, path: String) {
	let mut file = match File::create(path) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create decrypted file. Error: {}",err);
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
	write_encrypt will write the encrypted buffer to the file 
	encrypted.txt.
 */
fn write_encrypt(buf: &Vec<u8>, path: String) {
	let mut out_file = match File::create(path) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create encrypted.txt file. Error: {}",err);
				return;
			}
	};

	match out_file.write_all(buf) {
		Ok(data) => data,
		Err(err) => {
			println!("Unable to write encrypted data to file encrypted.txt. Error: {}",err);
			return;
		}
	};
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

fn read_nonce(file : &str) -> Vec<u8> {
	let mut n_file = match File::open(file) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to open specified key file. Error: {}",err);
				return Vec::new();
			}
	};
	let mut nonce_vec = Vec::new();
	match n_file.read_to_end(&mut nonce_vec) {
		Ok(nb) => nb,
			Err(err) => {
				println!("Unable to read bytes in file. Error: {}",err);
				return Vec::new();
			}
	};
	nonce_vec
}
/*
	 write_nonce will write the generated 96 bit key to the file
	 key.txt
 */
fn write_nonce(nonce: &[u8; 12], path: String) {
	let mut out_file = match File::create(path) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create key.txt file. Error: {}",err);
				return;
			}
	};

	match out_file.write_all(nonce) {
		Ok(b) => b,
			Err(err) => {
				println!("Unable to write key to file key.txt. Error: {}",err);
				return;
			}
	};
}

/*
	write_key will write the generated key to the file key.txt.
*/
fn write_key(key: &[u8; 32], path: String) {
	let mut out_file = match File::create(path) {
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