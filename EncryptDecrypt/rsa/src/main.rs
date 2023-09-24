// QVLx Labs

/*
	 This application can encrypt/decrypt using the RSA algorithm.
 */
use rsa::{RsaPublicKey, RsaPrivateKey, PaddingScheme};
use rsa::PublicKey;
use rsa::pkcs1::{	FromRsaPrivateKey,ToRsaPrivateKey, ToRsaPublicKey};
use rand::rngs::OsRng;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {

	let args: Vec<String> = env::args().collect();
	if args.len() == 1 || args[1] == "-h" {
		println!("Usage: ./rsa -e <data file>");
		println!("Usage: ./rsa -d <encrypted file> <pem private key file>");
		return;
	}
	else if args.len() > 4 {
		println!("Less arguments expected, found {}",args.len());
		return;
	}
	else if args.len() < 3 {
		println!("More arguments expected, found {}",args.len());
		return;
	}

	let enc_file;
	if args[1] == "-d" && args.len() == 4 {
		enc_file = args[2].trim();
		let pri_key_file = Path::new(args[3].trim());
		let pri_key = match FromRsaPrivateKey::read_pkcs1_pem_file(&pri_key_file) {
			Ok(pri) => pri,
				Err(err) => {
					println!("Unable to read private key from pem file. Error: {}",err);
					return;
				}
		};
		let enc_data = read_data(enc_file);

		decrypt(enc_data,pri_key);
	}		
	else if args[1] == "-e" && args.len() == 3 {
		let data_file = args[2].trim();
		let data_vec = read_data(data_file);
		if data_vec.len() == 0 {return;}
		let mut rng = OsRng;
		let bits = 2048;
		let pri_key = match RsaPrivateKey::new(&mut rng, bits) {
			Ok(pk) => pk,
				Err(err) => {
					println!("Unable to generate private key. Error: {}",err);
					return;
				}
		};

		let pub_key = RsaPublicKey::from(&pri_key);
		let sk = match pri_key.to_pkcs1_pem() {
			Ok(pri) => pri,
				Err(err) => {
					println!("Unable to put private key in pem file format. Error: {}",err);
					return;
				}
		};

		let pk = match pub_key.to_pkcs1_pem() {
			Ok(pu) => pu,
				Err(err) => {
					println!("Unable to put public key in pem file format. Error: {}",err);
					return;
				}
		};
		write_pub(pk);
		write_pri(sk.to_string());
		encrypt(data_vec,pri_key);	
	}
	else {
		println!("No argument specifier was detected. Expected: \"-e\" or \"-d\"");
	}
}

fn encrypt(msg: Vec<u8>, pri_key: RsaPrivateKey) {
	let pub_k = RsaPublicKey::from(&pri_key);
	let mut rng = OsRng;
	let mut last:usize = 0;
	let mut enc_vec = Vec::new();
	let mut enc_data;
	let mut i:usize = 0;

	while i < msg.len() {

		let padding = PaddingScheme::new_pkcs1v15_encrypt();
		if i % 245 == 0 && i != 0 {
			enc_data = match pub_k.encrypt(&mut rng, padding, &msg[last..i]) {
				Ok(enc) => enc,
					Err(err) => {
						println!("Unable to encrypt data. Error: {}",err);
						return;
					}
			};
			enc_vec.append(&mut enc_data);
			last = i; 
		}
		else if i == msg.len() - 1 {

			enc_data = match pub_k.encrypt(&mut rng, padding, &msg[last..i+1]) {
				Ok(enc) => enc,
					Err(err) => {
						println!("Unable to encrypt data. Error: {}",err);
						return;
					}
			};

			enc_vec.append(&mut enc_data);
		}

		i += 1;
	}
	write_enc(enc_vec);
}	


fn decrypt(data: Vec<u8>, pri_key: RsaPrivateKey) {

	let mut i:usize = 0;
	let mut last:usize = 0;
	let mut dec_vec = Vec::new();
	let mut dec_data;

	while i < data.len() {

		let padding = PaddingScheme::new_pkcs1v15_encrypt();
		if i % 256 == 0 && i != 0 {

			dec_data = match pri_key.decrypt(padding, &data[last..i]) {
				Ok(dec) => dec,
					Err(err) => {
						println!("Unable to decrypt data. Error: {}",err);
						return;
					}
			};
			dec_vec.append(&mut dec_data);
			last = i;
		}
		else if i == data.len() - 1 {
			dec_data = match pri_key.decrypt(padding, &data[last..]) {
				Ok(dec) => dec,
					Err(err) => {
						println!("Unable to decrypt data. Error: {}",err);
						return;
					}
			};
			dec_vec.append(&mut dec_data);
		}
		i += 1;
	}
	write_dec(dec_vec);
}

fn write_enc(enc_data: Vec<u8>) {
	let mut out_file = match File::create("out/rsa/encrypted/encrypted.txt")	{
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create file \"encrypted.txt\". Error: {}",err);
				return;
			}
	};

	match out_file.write_all(&enc_data) {
		Ok(out) => out,
			Err(err) => {
				println!("Unable to write encrypted bytes to file. Error: {}",err);
				return;
			}
	};
	println!("Encrypted text written to out/rsa/encrypted/encrypted.txt");
}

fn write_dec(dec_data: Vec<u8>) {
	let mut out_file = match File::create("out/rsa/decrypted/decrypted.txt")	{
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create file \"decrypted.txt\". Error: {}",err);
				return;
			}
	};

	match out_file.write_all(&dec_data) {
		Ok(out) => out,
			Err(err) => {
				println!("Unable to write decrypted bytes to file. Error: {}",err);
				return;
			}
	};
	println!("Decrypted text written to out/rsa/decrypted/decrypted.txt");
}

fn read_data(file: &str) -> Vec<u8> {
	let path = Path::new(file);
	if path.exists() {
		let mut file = match File::open(file) {
			Ok(file) => file,
				Err(err) => {
					println!("Unable to open file. Error: {}",err);
					return Vec::new();
				}
		};
		let mut vec = Vec::new();
		match file.read_to_end(&mut vec) {
			Ok(d) => d,
				Err(err) => {
					println!("Unable to read data from file. Error: {}",err);
					return Vec::new();
				}
		};
		vec
	}
	else {
		println!("File path does not exist.");
		return Vec::new();
	}
}

fn write_pub(pub_k: String) {
	let mut out_file = match File::create("out/rsa/keys/pem_public_key.txt")	{
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create file \"pem_public_key.txt\". Error: {}",err);
				return;
			}
	};

	match out_file.write_all(pub_k.as_bytes()) {
		Ok(out) => out,
			Err(err) => {
				println!("Unable to write public key bytes to file. Error: {}",err);
				return;
			}
	};
	println!("Pem public key written to out/rsa/keys/pem_public_key.txt");
}

fn write_pri(pri_k: String) {
	let mut out_file = match File::create("out/rsa/keys/pem_private_key.txt")	{
		Ok(f) => f,
			Err(err) => {
				println!("Unable to create file \"pem_private_key.txt\". Error: {}",err);
				return;
			}
	};

	match out_file.write_all(pri_k.as_bytes()) {
		Ok(out) => out,
			Err(err) => {
				println!("Unable to write private key bytes to file. Error: {}",err);
				return;
			}
	};
	println!("Pem private key written to out/rsa/keys/pem_private_key.txt");
}
