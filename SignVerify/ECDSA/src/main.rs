// QVLx Labs

/*
	 Application generates ecdsa key pairs and is capable of signing and
	 verifying messages using ecdsa. When generating keys provide dummy input for
	 secret argument. 
 */
use clap::{ArgGroup, Clap, ValueHint};
use std::path::PathBuf;
use std::str::FromStr;
extern crate hex;
extern crate secp256k1;
use secp256k1::bitcoin_hashes::sha256;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use secp256k1::{schnorrsig, Message, PublicKey, Secp256k1, SecretKey, Signature};
#[derive(Clap, Debug, PartialEq)]
enum SigType {
	ECDSA,
	Schnorr,
}

/*
	 write_keys_schnorr() will write the public/secret keys to seperate files.
 */
fn write_keys_schnorr((sk,pk): (String, String)) {

	let mut p_key_file = match File::create("schnorr_key.pub"){
		Ok(file) => file,
		Err(err) => {
			println!("Unable to create file \"schnorr_key.pri\". Error: {}",err);
			return;
		}
	};
	let mut s_key_file = match File::create("schnorr_key.pri"){
		Ok(file) => file,
			Err(err) => {
				println!("Unable to create file \"schnorr_key.pri\". Error: {}",err);
				return;
			}
	};
	match p_key_file.write(pk.as_bytes()) {
		Ok(out) => out,
			Err(err) => {
				println!("Unable to write to \"schnorr_key.pub\". Error: {}",err);
				return;
			}
	};
	match s_key_file.write(sk.as_bytes()) { 
		Ok(out) => out,
			Err(err) => {
				println!("Unable to write to \"schnorr_key.pri\". Error: {}",err);
				return;
			}
	};
}
/*
	 generate_schnorr_keypair() will generate a pair of keys for schnorr signature
 */
fn generate_schnorr_keypair(seed: Vec<u8>) {
	let s = Secp256k1::new();
	let secret_key = match SecretKey::from_slice(&seed) {
		Ok(key) => key,
			Err(err) => {
				println!("Unable to create secret key from seed slice. Error: {}",err);
				return;
			}
	};
	let public_key = PublicKey::from_secret_key(&s,&secret_key);
	write_keys_schnorr((secret_key.to_string(),public_key.to_string())); 
}
/*
	 write_sig_schnorr() will write the generated signature to a seperate file
 */
fn write_sig_schnorr(sig: String , pk: String) {
	let mut out_file = match File::create("sig_schnorr.txt") {
		Ok(out) => out,
			Err(err) => {
				println!("Unable to create out file \"sig.txt\". Error: {}",err);
				return;
			}
	};
	let mut pk_file = match File::create("schnorr_key.pub") {
		Ok(file) => file,
			Err(err) => {
				println!("Unable to create file \"schnorr_key.pub\". Error: {}",err);
				return;
			}
	};
	match write!(out_file,"{}",sig) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to write schnorr signature to file \"sig_schnorr.txt\". Error: {}",err);
				return;
			}
	};
	match write!(pk_file,"{}",pk) {
		Ok(p) => p,
			Err(err) => {
				println!("Unable to write new schnorr public key to file \"schnorr_key.pub\". Error: {}",err);
				return;
			}
	};

}
/*
	 sign_schnorr() will use the given seckey and message to create a signature
	 and a new public key. Use the new public key when verifying and not original.
 */
fn sign_schnorr(seckey: String, msg: String) {
	let s = Secp256k1::new();
	let keypair = match schnorrsig::KeyPair::from_seckey_str(&s, &seckey) {
		Ok(kp) => kp,
			Err(err) => {
				println!("Unable to create keypair from secret key. Error: {}",err);
				return;
			}
	};

	let message = Message::from_hashed_data::<sha256::Hash>(msg.as_bytes());
	let sig = s.schnorrsig_sign_no_aux_rand(&message, &keypair);
	let pk = schnorrsig::PublicKey::from_keypair(&s,&keypair);
	write_sig_schnorr(sig.to_string(),pk.to_string());

}
/*
	 verify_schnorr() will take a String signature, message and public key to
	 verify the message which will either output true or false.
 */
fn verify_schnorr(signature: String, msg: String, pubkey: String) -> bool {
	let s = Secp256k1::new();
	let pubkey = match schnorrsig::PublicKey::from_str(&pubkey) {
		Ok(pk) => pk,
			Err(err) => {
				println!("Unable to retrieve Schnorr public key to verify. Error: {}",err);
				return false;
			}
	};
	let sig = match schnorrsig::Signature::from_str(&signature) {
		Ok(sig) => sig,
			Err(err) => {
				println!("Unable to verify Schnorr signature. Error: {}",err);
				return false;
			}
	};
	let message = Message::from_hashed_data::<sha256::Hash>(msg.as_bytes());
	if s.schnorrsig_verify(&sig, &message, &pubkey).is_ok() {
		true
	} else {
		false
	}
}

/*
	 write_keys() will take the tuple of keys generated for ecdsa and write
	 them to seperate files.
 */
fn write_keys((sk,pk): (String, String)) {

	let mut p_key_file = match File::create("key.pub"){
		Ok(file) => file,
			Err(err) => {
				println!("Unable to create file \"key.pub\". Error: {}",err);
				return;
			}
	};
	let mut s_key_file = match File::create("key.pri"){
		Ok(file) => file,
			Err(err) => {
				println!("Unable to create file \"key.pri\". Error: {}",err);
				return;
			}
	};
	match p_key_file.write(pk.as_bytes()) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to write public key to file \"key.pub\". Error:{}",err);
				return;
			}
	};
	match s_key_file.write((sk.trim()).as_bytes()) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to write secret key to file \"key.pri\". Error:{}",err);
				return;
			}
	};

}

/*
	 generate_keypair() will generate a secret/public key for the ecdsa
 */
fn generate_keypair(seed: Vec<u8>) {
	let secp = Secp256k1::new();
	let secret_key = match SecretKey::from_slice(&seed) {
		Ok(sk) => sk,
			Err(err) => {
				println!("Unable to retrieve secret key from seed slice. Error: {}",err);
				return;
			}
	};
	let public_key = PublicKey::from_secret_key(&secp, &secret_key);
	write_keys((secret_key.to_string(), public_key.to_string()));
}

/*
	 write_sig() will write the signature and the new public key generated to seperate
	 files
 */
fn write_sig(sig: String, pk: String) {
	let mut out_file = match File::create("sig.txt") {
		Ok(out) => out,
			Err(err) => {
				println!("Unable to create signature file \"sig.txt\". Error: {}",err);
				return;
			}
	};
	let mut pk_file = match File::create("key.pub") {
		Ok(out) => out,
			Err(err) => {
				println!("Unable to create public key file \"key.pub\". Error: {}",err);
				return;
			}
	};

	match out_file.write_all((sig.trim()).as_bytes()) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to write signature to file \"sig.txt\". Error: {}",err);
				return;
			}
	};
	match write!(pk_file,"{}",pk) {
		Ok(f) => f,
			Err(err) => {
				println!("Unable to write public key to file \"key.pub\". Error: {}",err);
				return;
			}
	};
}

/*
	 sign() will take the secrety key and message provided to create a signature, the original public
	 key generated will be overwritten for the new public key generated to be later verified 
 */
fn sign(seckey: String, msg: String) {
	let seckey = match SecretKey::from_str(&seckey) {
		Ok(k) => k,
			Err(err) => {
				println!("Unable to retrieve secret key for signing. Error: {}",err);
				return;
			}
	};

	let message = Message::from_hashed_data::<sha256::Hash>(msg.as_bytes());
	let secp = Secp256k1::new();
	let sig = secp.sign(&message, &seckey);
	let public_key = PublicKey::from_secret_key(&secp, &seckey);
	write_sig(sig.to_string(),public_key.to_string());
}

/*
	 verify() will take a signature , message and public key and will return
	 a boolean determining if the message has been successfully verified.
 */
fn verify(signature: String, msg: String, pubkey: String) -> bool {
	let pubkey = match PublicKey::from_str(&pubkey) {
		Ok(pk) => pk,
			Err(err) => {
				println!("Unable to verify public key. Error: {}",err);
				return false;
			}
	};
	let sig = match Signature::from_str(&signature) {
		Ok(s) => s,
			Err(err) => {
				println!("Unable to verify signature. Error: {}",err);
				return false;
			}
	};

	let message = Message::from_hashed_data::<sha256::Hash>(msg.as_bytes());
	let secp = Secp256k1::new();

	if secp.verify(&message, &sig, &pubkey).is_ok() {
		true
	} else {
		false
	}
}

#[derive(Debug, Clap)]
#[clap(group = ArgGroup::new("seck").required(true))]
pub struct CmdSign {
	/// Path to private key (Not implemented)
#[clap(parse(from_os_str), value_hint = ValueHint::AnyPath, short = 'f', group="seck")]
seckey_file: Option<PathBuf>,
							 /// Secret in hex
#[clap(group = "seck", short)]
							 secret: Option<String>,
							 /// Message to sign.
#[clap(required = true)]
							 msg: String,
							 /// Signature type
#[clap(arg_enum, default_value = "ecdsa", short = 't')]
							 sig_type: SigType,
}

#[derive(Debug, Clap)]
#[clap(group = ArgGroup::new("msg").required(true))]
pub struct CmdVerify {
	/// Signature in hex
#[clap(required = true)]
signature: String,
						 /// Message string
#[clap(group = "msg", required = true)]
						 message: String,
						 /// Public key in hex
#[clap(required = true)]
						 pubkey: String,
#[clap(arg_enum, default_value = "ecdsa", short = 't')]
						 //debug test
						 sig_type: SigType,
}

#[derive(Clap, Debug)]
#[clap(name = "musig-cli")]
/// Generate secp256k1 keys, sign and verify messages with ECDSA and Schnorr
enum Opt {
	/// Generate a public key from a secret (private key/seed/secret key)
	Generate {
		/// Secret (also known as seed, private key or secret key) in hex (64 chars).
secret: String,
				/// Type of signature.
#[clap(arg_enum, default_value = "ecdsa", short = 't')]
				sig_type: SigType,
	},

	/// Sign a message. Signature is returned.
	Sign(CmdSign),

	/// Verify a signature for a given message. True is returned for a valid signature otherwise False.
	Verify(CmdVerify),
}

fn main() {
	let matches = Opt::parse();

	match matches {
		Opt::Generate {secret:_,sig_type } => {
			let output = match Command::new("xxd").args(vec!["-l","32","-p","/dev/urandom"]).output() {
				Ok(o) => o,
					Err(e) => { println!("{}",e); return; }
			};
			let rand_hex_str = String::from_utf8(output.stdout).expect("").replace("\n","");
			let seed_bytes = match hex::decode(&rand_hex_str[..]) {
				Ok(d) => d,
					Err(err) => {
						println!("Failed to generate seed bytes. Error: {}",err);
						return;
					}
			};

			match sig_type {
				SigType::ECDSA => {
					generate_keypair(seed_bytes);
				}
				SigType::Schnorr => {
					generate_schnorr_keypair(seed_bytes);
				}
			};
		}
		Opt::Sign(cmd) => {
			match cmd.sig_type {
				SigType::ECDSA => {
					let s = match cmd.secret {
						Some(sec) => sec,
							None => {
								println!("Unable to retrieve private key string for ecdsa.");
								return;
							}
					};
					sign(s, cmd.msg);
				}
				SigType::Schnorr => {
					let s = match cmd.secret {
						Some(sec) => sec,
							None => {
								println!("Unable to retrieve private key string for Schnorr.");
								return;
							}
					};

					sign_schnorr(s, cmd.msg);
				}
			};
		}
		Opt::Verify(cmd) => {
			match cmd.sig_type {
				SigType::ECDSA => {
					let res = verify(cmd.signature, cmd.pubkey ,cmd.message);
					if res {
						println!("True");
					} else {
						println!("False");
					}
				}
				SigType::Schnorr => {
					let res = verify_schnorr(cmd.signature, cmd.pubkey ,cmd.message );
					if res {
						println!("True");
					} else {
						println!("False");
					}
				}
			};
		}
	};
}
