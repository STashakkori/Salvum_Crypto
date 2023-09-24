//  QVLx Labs

use crc::{Crc, CRC_16_ISO_IEC_14443_3_A, CRC_32_CKSUM, CRC_64_GO_ISO};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
  let args: Vec<String> = env::args().collect();
  let usage = "Salvum CRC tool arguments:\n(1) input file path.\n(2) bit length: '16', '32', '64'";
  
  // Prevent bad arguments from crashing the tool
  if args.len() < 3 || args.len() > 4{
    println!("Missing arguments.");
    println!("{}", usage);
    return;
  }

  // Process file script argument
  let path_infile = Path::new(args[1].trim());
  if !path_infile.exists() {
    println!("Invalid path given.");
    println!("{}", usage);
    return;
  }
  
  let infile = fs::read_to_string(path_infile).expect("failed to read payload file");
   
  // Processing other script arguments
  // Starting with and depending on bit length
  match &args[2][..] {
    // 16-bit CRC logic
    "16" => {
      pub const SXTBIT: Crc<u16> = Crc::<u16>::new(&CRC_16_ISO_IEC_14443_3_A);
      let crc_out = SXTBIT.checksum(infile.as_bytes());
      println!("16-bit CRC: {}", crc_out); 
    },

    // 32-bit CRC logic
    "32" => {
      pub const THTBIT: Crc<u32> = Crc::<u32>::new(&CRC_32_CKSUM);
      let crc_out = THTBIT.checksum(infile.as_bytes());
      println!("32-bit CRC: {}", crc_out); 
    },

    // 64-bit CRC logic
    "64" => {
      pub const SFRBIT: Crc<u64> = Crc::<u64>::new(&CRC_64_GO_ISO);
      let crc_out = SFRBIT.checksum(infile.as_bytes());
      println!("64-bit CRC: {}", crc_out);  
     },
    _ => {
      println!("Invalid bit length.");
      println!("{}", usage );
    }
  }
}
