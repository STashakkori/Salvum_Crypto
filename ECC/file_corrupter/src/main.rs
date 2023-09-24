// -r00r00   QVLx Labs

use rand::{RngCore, SeedableRng, StdRng};
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

fn main() {
  // Arguments check
  let args: Vec<String> = env::args().collect();
  let (seed, file_in, file_out, corruptions, lower_bound, mut upper_bound) = parse_args(args);

  // Read from the input file
  let data = match fs::read(file_in.clone()) {
    Ok(out) => out,
    Err(err) => {
      println!("Can't open the file. {}", err);
      return;
    }
  };

  if upper_bound > data.len() {
    upper_bound = data.len();
  }

  // Corrupt the string
  let mut corrupted = data.clone();
  let mut my_rand = StdRng::from_seed([seed; 32]);
  for _ in 0..corruptions {
    let ri: usize = (my_rand.next_u64() as usize % upper_bound) + lower_bound;
    corrupted[ri] = my_rand.next_u64() as u8;
  }

  // Create the output file
  let mut file = match File::create(file_out.clone()) {
    Ok(f) => f,
    Err(err) => {
      println!("Can't create file. {}", err);
      return;
    }
  };

  // Write corruption to new file
  match file.write_all(&corrupted) {
    Ok(_) => {}
    Err(err) => {
      println!("Can't write to file. {}", err);
      return;
    }
  }
  println!("Corrupted the contents of {} to {}.", file_in, file_out);

  // Hex dump it for them
  let child = match Command::new("hexdump").args(vec!["-C", &file_out]).spawn() {
    Ok(out) => out,
    Err(err) => {
      println!("Failed to execute hexdump. {}", err);
      return;
    }
  };
  match child.wait_with_output() {
    Ok(_) => {}
    Err(err) => {
      println!("Failed to execute hexdump. {}", err);
      return;
    }
  }
}

fn parse_args(args: Vec<String>) -> (u8, String, String, usize, usize, usize) {
  let (mut seed, mut filein, mut fileout, mut corruptions, mut lower_bound, mut upper_bound) = (0, String::new(), String::new(), 0, 0, 0);

  let mut i = 1;
  while i < args.len() {
    match &args[i][..] {
      "-s" => {
        seed = match args[i + 1].trim().parse() {
          Ok(out) => out,
          Err(_err) => {
            println!("Please type a positive number for seed.");
            0
          }
        };
      }
      "-i" => filein = args[i + 1].clone(),
      "-o" => fileout = args[i + 1].clone(),
      "-c" => {
        corruptions = match args[i + 1].trim().parse() {
          Ok(out) => out,
          Err(_err) => {
            println!("Please type a positive number for corruptions.");
            0
          }
        };
      }
      "-l" => {
        lower_bound = match args[i + 1].trim().parse() {
          Ok(out) => out,
          Err(_err) => {
            println!("Please type a positive number for lower bound.");
            0
          }
        };
      }
      "-u" => {
        upper_bound = match args[i + 1].trim().parse() {
          Ok(out) => out,
          Err(_err) => {
            println!("Please type a positive number for upper bound.");
            0
          }
        };
      }
      "-h" => {
        println!("This tool will take a file and swap bytes to random values to\n\
                  mimic a corruption, the original file is untouched and the corruption\n\
                  is copied to a new file 'corrupted_<filename>.txt\n\n\
                  Usage: filecorrupter [[options <value>] ... ]\n\
                  \t-f <String> : Specify the file to corrupt\n\
                  \t-c <Number> : Number of times to replace bytes\n\
                  \t-l <Number> : Lower bound of the file to corrupt\n\
                  \t-u <Number> : Upper bound of the file to corrupt");
        std::process::exit(0);
      }
      _ => {
        println!("Bad argument {}", &args[i][..]);
        std::process::exit(0);
      }
    }
    i = i + 2;
  }
  return (seed, filein, fileout, corruptions, lower_bound, upper_bound);
}
