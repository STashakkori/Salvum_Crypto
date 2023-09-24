// -r00r00   QVLx Labs

use reed_solomon::{Decoder, Encoder};
use std::fs::{File};
use std::io::{Read, Write};

enum Code {
  Unknown,
  Encode,
  Decode,
}

fn main() {
  // Arguments check
  let args: Vec<String> = std::env::args().collect();
  let (code, file_in, file_out, known) = parse_args(args);

  let ecc_len = 8;
  let enc = Encoder::new(ecc_len);
  let dec = Decoder::new(ecc_len);

  let mut data: Vec<u8> = Vec::new();
  match File::open(file_in) {
    Ok(mut f) => f.read_to_end(&mut data),
    Err(err) => {
      println!("Failed to read from file. {}", err);
      return;
    }
  };
  let mut file = match File::create(file_out.clone()) {
    Ok(f) => f,
    Err(err) => {
      println!("Failed to create file. {}", err);
      return;
    }
  };

  match code {
    Code::Encode => {
      // Write the encoded message to the output file
      let encoded = enc.encode(&data);
      match file.write_all(&encoded) {
        Ok(_) => {}
        Err(err) => {
          println!("Can't write to file. {}", err);
          return;
        }
      }
    }
    Code::Decode => {
      // Attempt to recover the data
      let known_erasures = [known];
      let recovered = match dec.correct(&data, Some(&known_erasures)) {
        Ok(r) => r,
        Err(err) => {
          println!("Failed to recover the file. {:?}", err);
          return;
        }
      };

      // If successful write it to a file
      match file.write_all(&recovered.data()) {
        Ok(_) => {}
        Err(err) => {
          println!("Can't write to file. {}", err);
          return;
        }
      }

      println!("Successfully recovered the file to {}.", file_out);
    }
    _ => {
      println!("Not an option.");
    }
  }
}

fn parse_args(args: Vec<String>) -> (Code, String, String, u8) {
  let (mut code, mut filein, mut fileout, mut known) = (Code::Unknown, String::new(), String::new(), 0);

  let mut i = 1;
  while i < args.len() {
    match &args[i][..] {
      "=e" => code = Code::Encode,
      "-d" => {
	code = Code::Decode;
        known = match args[i + 1].trim().parse() {
          Ok(out) => out,
          Err(_err) => {
            println!("Please type a positive number for known erasure.");
            0
          }
        };
	i += 1;
      }
      "-i" => {
	filein = args[i + 1].clone();
	i += 1;
      }
      "-o" => {
	fileout = args[i + 1].clone();
	i += 1;
      }
      "-h" => {
        println!("This tool will take a file and either encode it, or attempt\n
		  to decode it after a corruption has occurred.\n\n\
                  Usage: reed_solomon [[options <value>] ... ]\n\
		  \t-e          : Encode
		  \t-d <Number> : Decode, with specified bit number of known erasure
                  \t-i <String> : Input file\n\
                  \t-o <String> : Output file");
        std::process::exit(0);
      }
      _ => {
        println!("Bad argument {}", &args[i][..]);
        std::process::exit(0);
      }
    }
    i += 1;
  }
  return (code, filein, fileout, known);
}
