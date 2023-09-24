// -r00r00    QVLx Labs

use rand::{jitter::JitterRng, RngCore};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    println!("Too few arguments, you need a seed and number of bits!");
    return;
  }
  else if args.len() > 3 {
    println!("Too many arguments, you only need seed and number of bits!");
    return;
  }

  let _seed = args[1].trim().parse();
  let seed: u64 = match _seed {
    Ok(_seed) => _seed,
    Err(_error) => {
      println!("Please type a positive number for seed!");
      return;
    }
  };
  let _bits = args[2].trim().parse();
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
  
  const ROUNDS: usize = 1_000_000;

  let mut rng = match JitterRng::new() {
    Ok(out) => out,
    Err(_) => { return; }
  };
  let mut deltas_variable: Vec<u8> = Vec::with_capacity(ROUNDS);
  let mut deltas_minimal: Vec<u8> = Vec::with_capacity(ROUNDS);

  for _ in 0..ROUNDS {
    deltas_variable.push(rng.timer_stats(true) as u8);
    deltas_minimal.push(rng.timer_stats(false) as u8);
  }

  match File::create("jitter_rng_var.bin") {
    Ok(mut out) => out.write_all(&deltas_variable),
    Err(_) => { return; }
  };
  match File::create("jitter_rng_min.bin") {
    Ok(mut out) => out.write_all(&deltas_minimal),
    Err(_) => { return; }
  };
}
