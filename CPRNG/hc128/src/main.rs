// QVLx Labs

use rand::prng::hc128::Hc128Rng;
use rand::{RngCore, SeedableRng};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 6 {
    println!("Too few arguments, you need four 64-bit seeds and number of bits!");
    return;
  }
  else if args.len() > 6 {
    println!("Too many arguments, you need four 64-bit seeds and number of bits!");
    return;
  }

  let _seed64 = args[1].trim().parse();
  let seed64: u64 = match _seed64 {
    Ok(_seed64) => _seed64,
    Err(_error) => {
      println!("Please type a positive number for seed!");
      return;
    }
  };
  let _seed128 = args[2].trim().parse();
  let seed128: u64 = match _seed128 {
    Ok(_seed128) => _seed128,
    Err(_error) => {
      println!("Please type a positive number for seed!");
      return;
    }
  };
  let _seed192 = args[3].trim().parse();
  let seed192: u64 = match _seed192 {
    Ok(_seed192) => _seed192,
    Err(_error) => {
      println!("Please type a positive number for seed!");
      return;
    }
  };
  let _seed256 = args[4].trim().parse();
  let seed256: u64 = match _seed256 {
    Ok(_seed256) => _seed256,
    Err(_error) => {
      println!("Please type a positive number for seed!");
      return;
    }
  };
  let _bits = args[5].trim().parse();
  let bits: u64 = match _bits {
    Ok(_bits) => _bits,
    Err(_error) => {
      println!("Please type a positive number for bits!");
      return;
    }
  };
  if bits > 64 {
    println!("You can only specify up to 64 bits!");
    return;
  }

  let mut seed: [u8; 32] = [0; 32];

  // Build the 256-bit seed array
  for i in 0..8 {
    seed[i] = seed64.to_be_bytes()[i];
  }
  for i in 8..16 {
    seed[i] = seed128.to_be_bytes()[i-8];
  }
  for i in 16..24 {
    seed[i] = seed192.to_be_bytes()[i-16];
  }
  for i in 24..32 {
    seed[i] = seed256.to_be_bytes()[i-24];
  }

  let mut rng = Hc128Rng::from_seed(seed);
  let num: u64 = rng.next_u64() >> (64 - bits);
  println!("{}", num);
}
