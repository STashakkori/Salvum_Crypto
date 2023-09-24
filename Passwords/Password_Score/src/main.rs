// -$t@$h     QVLx Labs

use std::env;
use passwords::{analyzer, scorer};

fn main() {
  
  //usage string
  let usage = "\nsalvum_pass_score [password]\n\tpassword : the password to score";
  let strength = "0 - 20 very insecure\n20 - 40 insecure\n40 - 60 very weak\n60 - 80 weak\n80 - 90 good\n90 - 95 strong\n95 - 99 very strong\n99 - 100 invulnerable\n";
  //get command line arguements
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("{}\n{:?}",usage,args);
    return;
  }

  //1st arg: password to score
  let password = args[1].trim();

  let score = scorer::score(&analyzer::analyze(password));

  println!("score : {}",score);

}
