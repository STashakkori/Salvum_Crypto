// QVLx Labs

/*
	The os_rand application can generate integers and arrays for the following
	data sizes: 8, 16, 32, 64, 128, 256 and 512 bits.
*/
use rand_core::{RngCore,OsRng};
use rand::prelude::*;
use std::env;
use num_bigint::BigUint;
use num_bigint::RandBigInt;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 || args[1] == "-h" {
		println!("Usage: ./os_rand <data size>");
		println!("       ./os_rand <data size> -a <array length>");
		return;
	}
	if args.len() > 4 {
		println!("Expected fewer arguments, found {}",args.len());
		return;
	}
	if args.len() < 2 {
		println!("Expected more arguments, found {}", args.len());
		return;
	}	
	let size = match args[1].parse::<u16>() {
		Ok(v) => v,
			Err(err) => {
				println!("Unable to parse string literal. Error: {}",err);
				return;
			}
	};

	if args.len() == 4 && args[2] == "-a" { 
		match &args[3].parse::<usize>() {
			Ok(v) => gen_arra(size, *v),
				Err(err) => {
					println!("Unable to parse string literal. Error: {}",err);
					return;
				}
		};

	}
	else if args.len() == 3 && args[2] == "-a" {
		println!("Unable to generate array, no array length specified.");
		return;
	}
	else {
		gen_int(size);
	}


}

/*
	gen_int generates a random integer based on the given size.
*/
fn gen_int(size: u16) {
	let mut _rng = StdRng::from_entropy();
	match size {
		8 => {
			let x:u8 = _rng.gen();
			println!("{}",x);
		},
			16 => {
				let x:u16 = _rng.gen();
				println!("{}",x);
			},
			32 => {
				let x:u32 = _rng.gen();
				println!("{}",x);
			},
			64 => {
				let x:u64 = _rng.gen();
				println!("{}",x);
			},
			128 => {
				let x:u128 = _rng.gen();
				println!("{}",x);
			},
			256 => {
				let mut r = rand::thread_rng();
				let y = r.gen_biguint(256);
				println!("{}",y);
			},
			512 => {
				let mut r = rand::thread_rng();
				let y = r.gen_biguint(512);
				println!("{}",y);
			},

			_ => {
				println!("Unrecognized size.");
				println!("Acceptable sizes: 8, 16, 32, 64, 128, 256, 512");
				return;
			}
	};
}

/*
	gen_arra generates an array of given data size and length.
*/
fn gen_arra(size: u16, arr_len: usize) {
	match size {
		8 => {
			let mut key = vec![0u8; arr_len];
			OsRng.fill_bytes(&mut key);
			println!("{:?}",key);
		},
			16 => {
				let mut key = vec![0u16; arr_len];
				match OsRng.try_fill(&mut key[..]) {
					Ok(v) => v,
						Err(err) => {
							println!("Unable to fill array with random bytes. Error: {}",err);
							return;
						}
				};
				println!("{:?}",key);
			},
			32 => {
				let mut key = vec![0u32; arr_len];
				match OsRng.try_fill(&mut key[..]) {
					Ok(v) => v,
						Err(err) => {
							println!("Unable to fill array with random bytes. Error: {}",err);
							return;
						}
				};
				println!("{:?}",key);
			},
			64 => {
				let mut key = vec![0u64; arr_len];
				match OsRng.try_fill(&mut key[..]) {
					Ok(v) => v,
						Err(err) => {
							println!("Unable to fill array with random bytes. Error: {}",err);
							return;
						}
				};
				println!("{:?}",key);
			},
			128 => {
				let mut key = vec![0u128; arr_len];
				match OsRng.try_fill(&mut key[..]) {
					Ok(v) => v,
						Err(err) => {
							println!("Unable to fill array with random bytes. Error: {}",err);
							return;
						}
				};
				println!("{:?}",key);
			},
			256 => println!("{:?}",fill_bytes_bigu(256, arr_len)),
			512 => println!("{:?}",fill_bytes_bigu(512, arr_len)),

			_ => {
				println!("Unrecognized size.");
				println!("Acceptable sizes: 8, 16, 32, 64, 128, 256, 512");
				return;
			}
	};
}

/*
	fill_bytes_bigu generates a vector of a given length and of data type BigUint 
	which is meant to accept values of data sizes 256 and 512 bits. 
*/
fn fill_bytes_bigu(size: u16, arr_len: usize) -> Vec<BigUint>{
	let mut r = rand::thread_rng();
	let mut vec = Vec::new();
	for _ in 0..arr_len {
		vec.push(r.gen_biguint(size.into()));
	}
	vec
}
