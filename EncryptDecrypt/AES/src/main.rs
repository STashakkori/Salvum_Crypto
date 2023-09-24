//     QVLx Labs

//usage: salvum_aes OPTION KEYSIZE PATH_INPUT PATH_KEY PATH_OUTPUT
//OPTION = d or e
//KEYSIZE = 128, 192, or 256 length in bits 
//PATH_INPUT = path to the input payload being either encrypted or decrypted
//PATH_KEY = path to the key used for encryption or decryption
//PATH_OUTPUT = path to the file in which the encrypted or decrypted data will be written to

use std::fs;
use std::path::Path;
use std::env;
use aes::{Aes128, Aes192, Aes256, Block};
use aes::cipher::{
  BlockEncrypt, BlockDecrypt, NewBlockCipher, generic_array::GenericArray
};
use aes::cipher::consts;
use std::io::Write;

/*
 * main
 * 
 * @brief salvum_aes - salvum's solution to AES support
 * @param none
 * @return none
 */
fn main() {
  //usage string
  let usage = "usage: salvum_aes OPTION KEYSIZE PATH_INPUT PATH_KEY PATH_OUTPUT\n\tOPTION = d or e\n\tKEYSIZE = 128, 192, or 256 length in bits\n\tPATH_INPUT = path to the input payload being either encrypted or decrypted\n\tPATH_KEY = path to the key used for encryption or decryption\n\tPATH_OUTPUT = path to the file in which the encrypted or decrypted data will be written to";

  //get command line arguements
  let args: Vec<String> = env::args().collect();

  if args.len() == 1 || args.len() != 6 {
    println!("{}",usage);
    return;
  }

  //1st arguement: decrypt or encrypt option
  let option: String = String::from(args[1].trim());
  if !(option.eq(&String::from("e")) || option.eq(&String::from("d"))) {
    println!("{}",usage);
    println!("invalid option: {} please use d for decrypt and e for encrypt", option); //panic if the option is invalid
    return;
  }

  //2nd arguement: AES key size
  //let keysize: u16 = args[2].trim().parse().expect("failed to parse key size arg");
  let keysize: u16 = match args[2].trim().parse() {
    Ok(ks) => ks,
    Err(err) => {
      println!("{}",usage);
      println!("unable to parse the key size. Error : {}",err);
      return;
    }
  };
  if !(keysize == 128 || keysize == 192 || keysize == 256) {
    println!("{}",usage);
    println!("invalid key size: {} please use keysize 128, 192, or 256", keysize); //panic if the keysize is invalid
    return;
  }

  //3rd arguement: path to PAYLOAD file
  let path_payload = Path::new(args[3].trim());
  if !path_payload.exists() {
    println!("{}",usage);
    println!("Payload path does not exist: '{}'", args[3].trim()); //panic if the payload file does not exist
    return;
  }

  //4th arguement: path to KEY file
  let path_key = Path::new(args[4].trim());
  if !path_payload.exists() {
    println!("{}",usage);
    println!("Key path does not exist: '{}'", args[4].trim()); //panic if the key file does not exist
    return;
  }

  //5th arguement: path to OUTLOAD file to create
  let path_outload = Path::new(args[5].trim());

  //create the outload file
  let mut file_outload = match fs::File::create(path_outload) {
    Ok(f) => f,
    Err(err) => {
      println!("{}",usage);
      println!("failed to create outload file error : {}", err);
      return;
    }
  };

  //read the payload file into a byte vector
  let vec_payload: Vec<u8> = match fs::read(path_payload){
    Ok(vec) => vec,
    Err(err) => {
      println!("failed to read payload file. {}",err);
      return;
    }
  };

  //read the user provided key file into a GenericArray
  let vec_key = match fs::read(path_key) {
    Ok(vec) => vec,
    Err(err) => {
      println!("failed to read key file. {}",err);
      return;
    }
  };

  if (keysize / 8) != vec_key.len() as u16 {
    println!("{}",usage);
    println!("invalid key\nspecified key size : {}\ngiven key size : {}",keysize,vec_key.len()*8);
    return;
  }
 
  //create and initialize cipher objects
  let mut cipher128: Aes128 = Aes128::new(GenericArray::from_slice(&[0u8; 16]));
  let mut cipher192: Aes192 = Aes192::new(GenericArray::from_slice(&[0u8; 24]));
  let mut cipher256: Aes256 = Aes256::new(GenericArray::from_slice(&[0u8; 32]));

  //create a 128-bit key and associated AES object
  if keysize == 128 {
    let mut key128: GenericArray<u8, consts::U16> = *GenericArray::from_slice(&[0u8; 16]); //initialize the key to all 0's
    for i in 0..16 {
      key128[i] = vec_key[i] as u8; //read in bytes from user key
    }
    cipher128 = Aes128::new(&key128); //create the AES object
  }

  //create a 192-bit key and associated AES object
  else if keysize == 192 {
    let mut key192: GenericArray<u8, consts::U24> = *GenericArray::from_slice(&[0u8; 24]); //initialize the key to all 0's
    for i in 0..24 {
      key192[i] = vec_key[i] as u8; //read in bytes from user key 
    }
    cipher192 = Aes192::new(&key192); //create the AES object
  }

  //create a 256-bit key and associated AES object
  else if keysize == 256 {
    let mut key256: GenericArray<u8, consts::U32> = *GenericArray::from_slice(&[0u8; 32]); //initialize the key to all 0's
    for i in 0..32 {
      key256[i] = vec_key[i] as u8; //read in bytes from user key
    }
    cipher256 = Aes256::new(&key256); //create the AES object
  }

  //create a new vector of blocks
  let mut blocks: Vec<Block> = Vec::new();
  let mut count = 0; //counter to track the number of bytes read
  
  //Box object to store a block on the heap so that the block isnt freed between loops
  let mut copyblock: Box<Block> = Box::new(Block::default()); 
  
  //iterate through the payload vector and split into 128-bit(16byte) blocks
  for byte in vec_payload.iter() {
    copyblock[count] = *byte; //copy the byte
    count += 1;
    if count == 16 {
      blocks.push(*copyblock); //copy 16 bytes into blocks vector
      copyblock = Box::new(Block::default()); //reset the copyblock
      count = 0; //reset count
    }
  }

  //copy the remaining bytes (< 16) to the blocks vector
  if count > 0 {
    blocks.push(*copyblock);
  }
  
  //handle the encryption option
  if option.eq(&String::from("e")) {
    for mut blk in blocks {
      match keysize {
        128 => cipher128.encrypt_block(&mut blk), //encrypt block for 128-bit key
        192 => cipher192.encrypt_block(&mut blk), //encrypt block for 192-bit key
        256 => cipher256.encrypt_block(&mut blk), //encrypt block for 256-bit key
        _ => panic!("something bad happened") //this panic should never execute, for keysize input was already validated
      }
      match file_outload.write_all(&blk) { //write the encrypted block to a file
        Ok(_) => {}, //no error block written successfully
        Err(err) => {
          println!("Unable to write to outload file. Error : {}",err); //unable to write the block to the outload file
          return;
        }
      } 
    }
  }

  //handle the decryption option
  else if option.eq(&String::from("d")) {
    for mut blk in blocks {
      match keysize {
        128 => cipher128.decrypt_block(&mut blk), //decrypt block for 128-bit key 
        192 => cipher192.decrypt_block(&mut blk), //decrypt block for 192-bit key
        256 => cipher256.decrypt_block(&mut blk), //decrypt block for 256-bit key
        _ => panic!("something bad happened") //this panic should never execute, for keysize input was already validated
      }
      match file_outload.write_all(&blk) { //write the decrypted block to a file
        Ok(_) => {}, //no error block written successfully
        Err(err) => {
          println!("Unable to write to outload file. Error : {}",err); //unable to write the block to the outload file
          return;
        }
      }
    }
  }
}


//fn print_block(block: Block) {
//  for byte in block {
//    print!{"{} ",byte};
//  }
//  print!{"\n"};
//}

//let mut makekey192 = fs::File::create(Path::new("src/key192")).expect("");
//let mut makekey256 = fs::File::create(Path::new("src/key256")).expect("");

//for i in 0..24 {
//  makekey192.write_all(&vec![i]).expect("");
//}
//for i in 0..32 {
//  makekey256.write_all(&vec![i]).expect("");
//}
//file_outload.write_all(&blk)
//create the outload file
//let mut file_outload = match fs::File::create(path_outload)
