// -n3wmAn    QVLx Labs

use std::env;
use passwords::hasher;
use hex::encode;
fn main() {
  //usage string
  let usage = "salvum_saltyhash [password] [options]\n\t-h : print this help menu\n\t-s : specify your own salt\n\t-p : spice up the hash by specifying your pepper too\n\texample : ./salvum_saltyhash mypassword -s mysalt -p mypepper\n";

  //get command line arguements
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("{}",usage);
    return;
  }

  let mut salt_str = "".to_string();
  let mut pepper_str = "".to_string();
  for (i, arg) in args.iter().enumerate() {
    if arg.eq(&"-h") {
      println!("{}",usage);
      return;
    }
    else if arg.eq(&"-s") {
      salt_str = args[i+1].trim().to_string();
    }
    else if arg.eq(&"-p") {
      pepper_str = args[i+1].trim().to_string();
    }
  }

  let password = args[1].to_string();
  
  let mut salt = hasher::gen_salt();
  if salt_str.len() != 0 && salt_str.len() != 16 {
    println!("{}Invalid salt size. Must be 16 characters long.",usage);
    return;
  }
  else if salt_str.len() == 16 {
    for (i, byte) in salt_str.as_bytes().iter().enumerate() {
      salt[i] = *byte;
    }
  }

  let mut hashedpass = match hasher::bcrypt(10, &salt, &password) {
    Ok(hash) => hash,
    Err(err) => {
      println!("Unable to hash password : {}",err);
      return;
    }
  };

  let mut pepper: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
  if pepper_str.len() != 0 && pepper_str.len() != 16 {
    println!("{}Invalid pepper size. Must be 16 characters long.",usage);
    return;
  }
  if pepper_str.len() == 16 {
    for (i, byte) in pepper_str.as_bytes().iter().enumerate() {
      pepper[i] = *byte;
    }
    println!("pepper used   : {}",encode(pepper));
    hashedpass = match hasher::bcrypt(10, &pepper, &hashedpass) {
      Ok(hash) => hash,
      Err(err) => {
        println!("Unable to hash password : {}",err);
        return;
      }
    };
  }



  println!("salt used     : {}\nhash produced : {}",encode(salt),encode(hashedpass));

}
