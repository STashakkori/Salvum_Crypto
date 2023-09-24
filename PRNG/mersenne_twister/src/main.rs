// -r00r00   QVLx Labs

use rand_mt::Mt64;
use std::env;

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

  let mut rng = Mt64::new(seed);
  let num: u64 = rng.next_u64() >> (64 - bits);
  println!("{}", num);
}
