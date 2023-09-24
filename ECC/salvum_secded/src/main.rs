// -n3wmAn    QVLx Labs

use std::fs;
use std::path::Path;
use std::io::Write;
use std::env;
use secded::{SecDed64,SecDed128,SecDedCodec};

fn main() {
  //usage string
  let usage = "\nusage: salvum_secded [option] [codec block size] [path input] [path output]\n";

  if args.len() < 5 {
    println!("{}",usage);
    return;
  }

  let mut blocksize: i32 = 0;
  //1st arg decode or encode option
  let option = args[1].trim();
  if !(option.eq("e") || option.eq("d")) {
    println!("{}\nInvalid option : {}",usage,option);
    return;
  }
  else if option.eq("e") {
    blocksize -= 1;
  }

  //2nd arg codec size option 64 or 128
  let size_codec: u8 = match args[2].trim().parse() { //parse for the size
    Ok(sz) => sz,
    Err(err) => {
      println!("{}\nUnable to parse : {}\nError : {}",usage,args[2].trim(),err);
      return;
    }
  };
  if size_codec == 64 {
    blocksize += 8;
  }
  else if size_codec == 128 {
    blocksize += 16
  }
  else {
    println!("{}\nInvalid size : {}\nValid sizes : '64' '128'",usage,size_codec);
    return;
  }

  //3rd arg path to the input data
  let path_input = &String::from(args[3].trim());
  let path_input = Path::new(path_input);
  if !path_input.exists() {
    println!("{}\nInput path doesn't exist",usage);
    return;
  }

  //4th arg path in which encoded or decoded data will be written
  let path_output = &String::from(args[4].trim());
  let path_output = Path::new(path_output);

  //create the outload file
  let mut file_output = match fs::File::create(path_output) {
    Ok(f) => f,
    Err(err) => {
      println!("{}",usage);
      println!("failed to create output file. error : {}", err);
      return;
    }
  };

  //read the input file into a byte vector
  let vec_input: Vec<u8> = match fs::read(path_input) {
    Ok(input) => input,
    Err(err) => {
      println!("failed to read the input file : {}\nError : {}",path_input.to_string_lossy(),err);
      return;
    }
  };
  
  //create the codec struct
  let secded64 = SecDed64::new(56);
  let secded128 = SecDed128::new(120);

  if size_codec == 64 {
    //create blocks data structure. this will eventually be written to the file
    let mut blocks64: Vec<[u8; 8]> = Vec::<[u8; 8]>::new();
    let mut block: [u8; 8] = [0,0,0,0,0,0,0,0];
    
    //count variable used to keep track of num bytes read
    let mut count: usize = 0;

    //iterate through the input byte vector
    for byte in vec_input {
      block[count] = byte;
      count += 1;
      if count == blocksize as usize { //grab every 7 bytes of data
        blocks64.push(block);
        block = [0,0,0,0,0,0,0,0];
        count = 0;
      }
    }
    //push the remaining bytes (< blocksize)
    //this will push 0's into the vector that will need to be removed later
    if count > 0 {
      blocks64.push(block)
    }

    //handle the encoding option
    if option.eq("e") {
      for blk in blocks64.iter_mut() {
        secded64.encode(blk);
      }
    }
    //handle the decoding option
    else if option.eq("d") {
      for blk in blocks64.iter_mut() {
        secded64.decode(blk).expect("unable to decode block");
      }
    }

    //write to output file
    for blk in blocks64.iter_mut() {
      //create a temp buffer that will be written to the output file
      let mut newblk: Vec<u8> = Vec::new();
      for byte in blk {
        if option.eq("e") { //push encoded byte to the buffer (including the padding 0's)
          newblk.push(*byte);
        }
        else if *byte > 0 { //push decoded byte to the buffer (excluding the padding 0's)
          newblk.push(*byte);
        }
      }
      match file_output.write_all(&newblk) { //write the block to the output file
        Ok(_) => {},
        Err(err) => {
          println!("Unable to write to the output file. Error : {}",err);
        }
      };
    }
  }
  else if size_codec == 128 {
    //create blocks data structure. this will eventually be written to the file
    let mut blocks128: Vec<[u8; 16]> = Vec::<[u8; 16]>::new();
    let mut block: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    
    //count variable used to keep track of num bytes read
    let mut count: usize = 0;

    //iterate through the input byte vector
    for byte in vec_input {
      block[count] = byte;
      count += 1;
      if count == blocksize as usize { //grab every 15 bytes of data
        blocks128.push(block);
        block = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        count = 0;
      }
    }
    //push the remaining bytes (< blocksize)
    //this will push 0's into the vector that will need to be removed later
    if count > 0 {
      blocks128.push(block)
    }

    //handle the encoding option
    if option.eq("e") {
      for blk in blocks128.iter_mut() {
        secded128.encode(blk);
      }
    }
    //handle the decoding option
    else if option.eq("d") {
      for blk in blocks128.iter_mut() {
        secded128.decode(blk).expect("unable to decode block");
      }
    }

    //write to output file
    for blk in blocks128.iter_mut() {
      //create a temp buffer that will be written to the output file
      let mut newblk: Vec<u8> = Vec::new();
      for byte in blk {
        if option.eq("e") { //push encoded byte to the buffer (including the padding 0's)
          newblk.push(*byte);
        }
        else if *byte > 0 { //push decoded byte to the buffer (excluding the padding 0's)
          newblk.push(*byte);
        }
      }
      match file_output.write_all(&newblk) { //write the block to the output file
        Ok(_) => {},
        Err(err) => {
          println!("Unable to write to the output file. Error : {}",err);
        }
      };
    }
  }
}

fn gencodec<T: SecDedCodec>(codec: T, ) {
  let size_codec = 64;
  if codec.encodable_size < 57 {

  }
  else {

  }
  if size_codec == 64 {
    //create blocks data structure. this will eventually be written to the file
    let mut blocks64: Vec<[u8; 8]> = Vec::<[u8; 8]>::new();
    let mut block: [u8; 8] = [0,0,0,0,0,0,0,0];
    
    //count variable used to keep track of num bytes read
    let mut count: usize = 0;

    //iterate through the input byte vector
    for byte in vec_input {
      block[count] = byte;
      count += 1;
      if count == blocksize as usize { //grab every 7 bytes of data
        blocks64.push(block);
        block = [0,0,0,0,0,0,0,0];
        count = 0;
      }
    }
    //push the remaining bytes (< blocksize)
    //this will push 0's into the vector that will need to be removed later
    if count > 0 {
      blocks64.push(block)
    }

    //handle the encoding option
    if option.eq("e") {
      for blk in blocks64.iter_mut() {
        secded64.encode(blk);
      }
    }
    //handle the decoding option
    else if option.eq("d") {
      for blk in blocks64.iter_mut() {
        secded64.decode(blk).expect("unable to decode block");
      }
    }

    //write to output file
    for blk in blocks64.iter_mut() {
      //create a temp buffer that will be written to the output file
      let mut newblk: Vec<u8> = Vec::new();
      for byte in blk {
        if option.eq("e") { //push encoded byte to the buffer (including the padding 0's)
          newblk.push(*byte);
        }
        else if *byte > 0 { //push decoded byte to the buffer (excluding the padding 0's)
          newblk.push(*byte);
        }
      }
      match file_output.write_all(&newblk) { //write the block to the output file
        Ok(_) => {},
        Err(err) => {
          println!("Unable to write to the output file. Error : {}",err);
        }
      };
    }
  }
}