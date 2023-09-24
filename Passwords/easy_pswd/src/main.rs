// QVLx Labs

use easy_password::bcrypt::hash_password;

fn main() {
  let bcrypt_rounds = 12; // Secure default
  let hash: String = hash_password("my_password", b"secure_key", 12).unwrap();
  println!("Generating secure hash...");
  println!("---------------------------------------------");
  println!("{}", hash);
  println!("---------------------------------------------");
}
