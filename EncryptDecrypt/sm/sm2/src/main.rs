// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

/*
	This application implements the signing and verifying of public/secret keys,
	the generation of private/public keys and deriving a public key from a 
	private key.
*/
use libsm::sm2::signature::SigCtx; 
use std::env;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use libsm::sm2::signature::Signature;
use num_bigint::BigUint;
/*
	parse_vec32 is used for us to get our data into the appropriate form required by
  the libsm function calls, parsevec32 takes an input file and returns the input
  strings as u32s.
*/
fn parse_vec32(vec: Vec<&str>) -> Vec<u32> {
	let mut sk_parse = Vec::new();
	for ele in vec.iter() {
		let p = match ele.parse::<u32>() {
			Ok(par) => par,
				Err(err) => {
					println!("Unable to parse value. Error: {}",err);
					0
				}
		};
		if p == 0 {return Vec::<u32>::new()}
		sk_parse.push(p);
	}
	sk_parse
}

/*
	parse_vec8 works identically to parse_vec32 but instead of parsing strings into 
	u32s its into u8s.
*/
fn parse_vec8(vec: Vec<&str>) -> Vec<u8> {
	let mut sk_parse = Vec::new();
	for ele in vec.iter() {
		let p = match ele.parse::<u8>() {
			Ok(par) => par,
				Err(err) => {
					println!("Unable to parse value. Error: {}",err);
					0
				}
		};
		if p == 0 {return Vec::<u8>::new()}
		sk_parse.push(p);
	}
	sk_parse
}
/*
	format_bigUint takes a parameter of type BigUint and formats it
	to extract the data segment and return it as a string.
*/
fn format_biguint(bui : BigUint) -> String {
	let mut sk_format = match File::create("sk_format.txt") {
		Ok(file) => file,
			Err(err) => {
				println!("Unable to create file \"sk_format.txt\". Error: {}",err);
				return "e".to_string()
			}
	};
	let mut sk_reader = match File::open("sk_format.txt") {
		Ok(file) => file,
			Err(err) => {
				println!("Unable to create file \"sk_format.txt\". Error: {}",err);
				return "e".to_string()

			}
	};

	match write!(sk_format,"{:?}",bui) {
		Ok(output) => output,
			Err(err) => {
				println!("Unable to write to file \"sk_format.txt\". Error: {}",err);
				return "e".to_string()

			}
	};


	let mut sk_string = String::new();
	match sk_reader.read_to_string(&mut sk_string) {
		Ok(num_bytes) => num_bytes,
			Err(err) => {
				println!("Unable to read file data. Error: {}",err);
				return "e".to_string()

			}
	}; 

	let p1 = match sk_string.find("[") {
		Some(val) => val + 1,
			None => { println!("unable to find element.");
				return "e".to_string()

			}
	};
	let p2 = match sk_string.find("]") {
		Some(val) => val,
			None => {
				println!("unable to find element.");
				return "e".to_string()

			}
	};	
	let sk_format = (&sk_string[p1..p2].to_string()).replace(" ","");
	sk_format
}

fn main() {

	let args: Vec<String> = env::args().collect();
	if args[1] == "-usage" {
		println!("cargo run [--] <arg> <file>");
		println!("-kg : Generates key pairs and writes them to specified file.");
		println!("-d : Derives a secret key from a public key.");
		println!("-s : Sign takes some data, a public key and a private key and generates a
						  signature that can later then be verified it parses the relevant files
						  to obtain the elements neccessary and uses some libsm functions to retrieve a 
							public key and a secret key.");
		println!("-v : Verify takes a chunk of data with a signature and a public key and will 
									 verify that the signature is legitimate using libsm functions.");
	}

	let ctx = SigCtx::new();
	let arg:&str = &args[1];
	let f_name = &args[2];
	match arg {
		"-kg" => {
			let mut out_fname = match File::create(f_name) {
				Ok(file) => file,
					Err(err) => {
						println!("unable to create file Err: {}", err);
						return;
					}
			};

			let (pk, sk) = ctx.new_keypair(); 
			let sk_format = ctx.serialize_seckey(&sk);
			let pk_format = ctx.serialize_pubkey(&pk,true);
			match write!(out_fname,"{:?}\n{:?}",pk_format,sk_format){
				Ok(output) => output,
					Err(err) => {
						println!("Unable to write keygeneration to file \"kg\". Error: {}",err);
						return;
					}
			};
		}
		"-d" => {
			let mut file = match File::open(f_name) {
				Ok(file) => file,
					Err(err) => {
						println!("unable to open file : {}. Error: {}",args[2], err);
						return;
					}
			};
			let mut data_string = String::new();
			match file.read_to_string(&mut data_string) {
				Ok(out) => out,
					Err(err) => {
						println!("unable to read data from specified file. Error: {}", err);
						return;
					}
			};
			let sk = match (data_string.trim()).parse::<u32>() {
				Ok(parse) => parse,
					Err(err) => {
						println!("Unable to parse secret key to type u32. Error: {}",err);
						return;
					}
			};
			let mut sk_vec: Vec<u32> = Vec::new();
			sk_vec.push(sk);
			let bu = BigUint::new(sk_vec);
			let pk = ctx.pk_from_sk(&bu); 
			let mut out_file = match File::create("keys.txt") {
				Ok(file) => file,
					Err(err) => {
						println!("unable to create file Err: {}", err);
						return;
					}
			};
			match write!(out_file,"x= {:?}\ny= {:?}\nz= {:?}\n{:#?}",pk.x.value,pk.y.value,pk.z.value,&sk) {
				Ok(output) => output,
					Err(err) => {
						println!("Unable to write keygeneration to file \"kg\". Error: {}",err);
						return;
					}
			};

		}	
		"-s" => {
			let mut file = match File::open(f_name) {
				Ok(file) => file,
					Err(err) => {
						println!("unable to open file : {}. Error: {}",args[2], err);
						return;
					}
			};
			let mut data_string = String::new();
			match file.read_to_string(&mut data_string) {
				Ok(out) => out,
					Err(err) => {
						println!("unable to read data from specified file. Error: {}", err);
						return;
					}
			};

			let edit_data = (data_string.replace("[","")).replace("]","");
			let lines : Vec<&str> = edit_data.split("\n").collect();

			let msg = lines[0].as_bytes();

			let s = lines[1].replace(" ","");
			let p = lines[2].replace(" ","");

			let s_str_vec : Vec<&str> = s.split(",").collect();
			let p_str_vec : Vec<&str> = p.split(",").collect();

			let ctx = SigCtx::new();
			let parse_p = parse_vec8(p_str_vec);
			let parse_s = parse_vec8(s_str_vec);
			let deser_p = match ctx.load_pubkey(&parse_p) {
											Ok(par) => par,
											Err(err) => {
												println!("Unable to load public key. Error: {}",err);
												return;
											}
									 	};
			let deser_s = match ctx.load_seckey(&parse_s) {
											Ok(par) => par,
											Err(err) => {
												println!("Unable to load public key. Error: {}",err);
												return;
											}
									 	};

			let sig = ctx.sign(msg,&deser_s ,&deser_p);
			
			let r_format = format_biguint(sig.r);
			let s_format = format_biguint(sig.s);
			
			let r: Vec<&str> = r_format.split(",").collect();
			let s: Vec<&str> = s_format.split(",").collect();
			let r_parse = parse_vec32(r);
			let s_parse = parse_vec32(s);
			let mut out_file = match File::create("sig.txt") {
				Ok(file) => file,
					Err(err) => {
						println!("unable to create file Err: {}", err);
						return;
					}
			};
			match write!(out_file,"{:?}\n{:?}\n{:?}\n{:?}\n",msg,parse_p,s_parse,r_parse) {
				Ok(output) => output,
					Err(err) => {
						println!("Unable to write keygeneration to file \"kg\". Error: {}",err);
						return;
					}
			};

		}
		"-v" => {
			let mut file = match File::open(f_name) {
				Ok(file) => file,
					Err(err) => {
						println!("unable to open file : {}. Error: {}",args[2], err);
						return;
					}
			};
			let mut data_string = String::new();
			match file.read_to_string(&mut data_string) {
				Ok(out) => out,
					Err(err) => {
						println!("unable to read data from specified file. Error: {}", err);
						return;
					}
			};
			let edit_data = (data_string.replace("[","")).replace("]","");
			let lines : Vec<&str> = edit_data.split("\n").collect();
			let msg = lines[0].replace(" ","");

			let l1 = lines[1].replace(" ","");
			let l2 = lines[2].replace(" ","");
			let l3 = lines[3].replace(" ","");
			let pk : Vec<&str> = l1.split(",").collect();
			let sig_s : Vec<&str> = l2.split(",").collect();
			let sig_r : Vec<&str> = l3.split(",").collect();
			
			let pk = parse_vec8(pk);
			let r = parse_vec32(sig_r);
			let s = parse_vec32(sig_s);
			let msg_vec:Vec<u8> = parse_vec8(msg.split(",").collect());
			let ctx = SigCtx::new();
			let sig = Signature { r: BigUint::new(r), s: BigUint::new(s)};
			let point = match ctx.load_pubkey(&pk) {
										Ok(par) => par,
											Err(err) => {
												println!("Unable to load public key. Error: {}",err);
												return;
											}
									};

			let result: bool = ctx.verify(&msg_vec, &point, &sig);
			match result {
				true => {
					println!("Verification Successful.");
				},
						 false => {
							 println!("Verification Failed.");
						 }
			};

		}
		_ => {
			println!("Invalid argument.");
			return;
		}
	}
}

