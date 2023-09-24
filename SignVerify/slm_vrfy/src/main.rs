// -$t@$h    QVLx Labs

use include_crypt::{include_crypt, EncryptedFile};
use zeroize::Zeroize;
use shielded::Shielded;
use checksumdir;
use sha1_checksumdir;

fn main() {
  // Embeds hashes file into slm binary at compile time
  static FILE: EncryptedFile = include_crypt!("hashes.txt");

  // Decrypt hashes file at runtime
  let mut decrypted_str = match FILE.decrypt_str(){
    Ok(o) => o,
   Err(_) => return,
  };

  // Encrypt hashes in memory for protection at rest
  let mut shielded_str = shielded::Shielded::new(decrypted_str.as_bytes().to_vec());
 
  // Secure-delete unecrypted hashes from memory
  decrypted_str.zeroize();

  // Generate hashes in slm binary of each directory in ext using repair tool naming scheme
  println!("cksm: {}", checksumdir::checksumdir("test_dir/a").unwrap());
  println!("cksm2: {}", sha1_checksumdir::checksumdir("test_dir/a").unwrap());
  println!("cksm: {}", checksumdir::checksumdir("test_dir/b").unwrap());
  println!("cksm2: {}", sha1_checksumdir::checksumdir("test_dir/b").unwrap());
  
  // Decrypt when ready to read the hashes in.
  let unshielded = shielded_str.unshield();
  print!("{}", String::from_utf8_lossy(unshielded.as_ref()));

}
