// -n3wmAn     QVLx Labs

use std::fs;
use std::path::Path;
use std::env;
use sha2::{Sha256,Sha512,Digest};
use std::str;
use std::io::Write;

fn main() {
  //usage string
  let usage = "\nUsage: sha2 [SIZE] [INPUT] [OUTPUT]\n\n\tSIZE   : size of the final hash '256' or '512'\n\tINPUT  : path to file to be hashed\n\tOUTPUT : path in which the hash will be written to\n\nExample: sha2 256 /path/to/input.txt /path/to/output.sha256\n";
  //println!("{}",usage);

  //get command line arguements
  let args: Vec<String> = env::args().collect();

  if args.len() != 4 {
    println!("{}",usage);
    println!("Invalid number of arguements : {}",args.len());
    return;
  }

  //1st arguement: size
  let size: u16 = match args[1].trim().parse() {
    Ok(sz) => sz,
    Err(err) => {
      println!("{}",usage);
      println!("Unable to parse the size arguement. Error : {}",err);
      return;
    }
  };

  //2nd arguement: input file path
  let path_input = Path::new(args[2].trim());
  if !path_input.exists() {
    println!("{}",usage);
    println!("Input file path does not exist: '{}'", args[2].trim());
    return;
  }

  //3rd arguement: output file path
  let path_output = Path::new(args[3].trim());
  let mut file_output = match fs::File::create(path_output) {
    Ok(f) => f,
    Err(err) => {
      println!("{}",usage);
      println!("Failed to create the output file : {}",err);
      return;
    }
  };

  //read the input file into a byte vector
  let vec_input: Vec<u8> = match fs::read(path_input) {
    Ok(vec) => vec,
    Err(err) => {
      println!("{}",usage);
      println!("Failed to read the input file : {}",err);
      return;
    }
  };

  //handle the 256-bit hash option 
  if size == 256 {
    let mut hasher256 = Sha256::new(); //create the hasher object
    hasher256.update(vec_input); //feed the hashing object the input file as a byte vector
    let result256 = hasher256.finalize(); //hash the file
    let mut string_result256: String = hex::encode(result256); //encode the results to a string
    match file_output.write_all(string_result256.as_bytes()) { //write the encoded string to a file
      Ok(_) => {}, //write successful
      Err(err) => {
        println!("{}",usage);
        println!("Unable to write to output file. Error : {}",err); //unable to write the block to the outload file
        return;
      }
    }
  }
  //handle the 512-bit hash option 
  else if size == 512 {
    let mut hasher512 = Sha512::new(); //create the hasher object
    hasher512.update(vec_input); //feed the hashing object the input file as a byte vector
    let result512 = hasher512.finalize(); //hash the file
    let mut string_result512: String = hex::encode(result512); //encode the results to a string
    match file_output.write_all(string_result512.as_bytes()) { //write the encoded string to a file
      Ok(_) => {}, //write successful
      Err(err) => {
        println!("{}",usage);
        println!("Unable to write to output file. Error : {}",err); //unable to write the block to the outload file
        return;
      }
    }
  }
}
