// -$t@$h    QVLx Labs

use zxcvbn::zxcvbn;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() > 2 {
    println!("Too many arguments given.");
    return;
  } 
  if args.len() < 2 {
    println!("Too few arguments given. Give me a password.");
    return;
  } 
  let estimate = zxcvbn(args[1].trim(), &[]).unwrap();
  println!("Password rating: {}", estimate.score());
  match estimate.score() {
    0 => {println!("Terrible password strength detected.");},
    1 => {println!("Very weak password strength detected.");},
    2 => {println!("Weak password strength detected.");},
    3 => {println!("Relatively weak password strength detected.");},
    4 => {println!("Acceptable password strength detected.");},
    _ => {println!("error - rating out of bounds");},
  }
}
