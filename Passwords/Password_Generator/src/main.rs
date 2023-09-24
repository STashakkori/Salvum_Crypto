// -n3wmAn     QVLx Labs

use std::env;
use passwords::PasswordGenerator;

fn main() {
  //usage string
  let usage = "\nsalvum_passwd [length] [options]\n\t-n : include numeric characters\n\t-l : include lowercase characters\n\t-u : include uppercase characters\n\t-s : include symbol characters\n";

  //fetch command line args
  let mut args: Vec<String> = env::args().collect();

  //ensure a valid number of args
  if args.len() < 3 || args.len() > 6 {
    println!("{}",usage);
    return;
  }

  //1st arg: password length
  let passlen: usize = match args[1].trim().parse() {
    Ok(len) => len,
    Err(err) => {
      println!("Unable to parse length arg : {}\nError : {}",args[1].trim(),err);
      return;
    }
  };
  //length must be > 3
  if passlen <= 0 {
    println!("{}\nPassword length must be > 0",usage);
    return;
  }

  //remove command name arg
  args.remove(0);
  //remove length args
  args.remove(0);
  
  //create character options
  let mut number: bool = false;
  let mut lowercase: bool = false;
  let mut uppercase: bool = false;
  let mut symbol: bool = false;
  
  //parse for options
  for arg in args {
    if arg.eq("-h") { //help
      println!("{}",usage);
      return;
    }
    else if arg.eq("-n") { //enable numeric characters
      number = true;
    }
    else if arg.eq("-l") { //enable lowercase characters
      lowercase = true;
    }
    else if arg.eq("-u") { //enable uppercase characters
      uppercase = true;
    }
    else if arg.eq("-s") { //enable symbol characters
      symbol = true;
    }
  }

  //atleast one character option must be enabled
  if number == false && lowercase == false && uppercase == false && symbol == false {
    println!("{}\nMust enable atleast one type of character",usage);
  }

  //create the generator struct
  let generator = PasswordGenerator {
    length: passlen,
    numbers: number,
    lowercase_letters: lowercase,
    uppercase_letters: uppercase,
    symbols: symbol,
    spaces: false,
    exclude_similar_characters: false,
    strict: true
  };

  //generate the password
  let passwd = match generator.generate_one() {
    Ok(pass) => pass,
    Err(err) => {
      println!("unable to generate password\nError : {}",err);
      return;
    }
  };

  //print the password to the console
  println!("\n{}\n",passwd);
}
