// -$t@$h   QVLx Labs

use checksumdir;
use sha1_checksumdir;
use std::fs::File;
use std::io::Write;

fn main() {
  // Create hashes file to get embedded in slm binary
  let mut file = File::create("hashes.txt").expect("");
  
  /////////////////////////////////XXX Directory a XXX///////////////////////////////
  // Hash directory a
  let blake2_a = checksumdir::checksumdir("test_dir/a").unwrap();
  let sha1_a = sha1_checksumdir::checksumdir("test_dir/a").unwrap();
  
  // Debug printing for directory a
  println!("blake2_a: {}", blake2_a);
  println!("cksm2: {}", sha1_a);
  
  // Write directory a hashes to file
  file.write("a".as_bytes()).expect("");
  file.write("\n".as_bytes()).expect("");
  file.write(blake2_a.as_bytes()).expect("");
  file.write("\n".as_bytes()).expect("");
  file.write(sha1_a.as_bytes()).expect("");
  file.write("\n".as_bytes()).expect("");
  
  /////////////////////////////////XXX Directory b XXX///////////////////////////////
  // Hash directory b
  let blake2_b = checksumdir::checksumdir("test_dir/b").unwrap();
  let sha1_b = sha1_checksumdir::checksumdir("test_dir/b").unwrap();

  // Write directory b hashes to file
  println!("cksm: {}", blake2_b);
  println!("cksm2: {}", sha1_b);
  
  // Write directory a hashes to file
  file.write("a".as_bytes()).expect("");
  file.write("\n".as_bytes()).expect("");
  file.write(blake2_b.as_bytes()).expect("");
  file.write("\n".as_bytes()).expect("");
  file.write(sha1_b.as_bytes()).expect("");
  file.write("\n".as_bytes()).expect("");

  println!("Directory hashing complete");
}
