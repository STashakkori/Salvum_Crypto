// -$t@$h     QVLx Labs

use std::fs;
use std::io::{stdin, stdout, Write};
use labrador_ldpc::LDPCCode;
use dialoguer::{theme::ColorfulTheme, Select};
use console::{style, Style};
use colored::Colorize;

fn main() {
  let temp= String::from("Hello and welcome to Labrador LDPC ECC tool.");
  println!("{} ", temp.bright_black());
  
  let temp= String::from("Would you like to encode or decode? (E or D): ");
  println!("{} ", temp.bright_black());
  let mut is_enc = String::new();
  match stdin().read_line(&mut is_enc){
    Ok(_)=> (),
    Err(_)=> { println!("read_line failed."); return; }
  };
  let mut is_encode: bool = true;
  let is_enc_str = is_enc.trim().to_string();
  if is_enc_str.eq("D") || is_enc_str.eq("d") {
		is_encode = false;
		println!("Arg {} received. Setting to decode mode.", is_enc_str);
  }
  else if is_enc_str.eq("E") || is_enc_str.eq("e") {
    println!("Arg {} received. Setting to encode mode.", is_enc_str);
  }
  else {
    let warning = String::from("Invalid argument. Defaulting to encode mode."); 
    println!("{}", warning.bright_magenta()); 
  }
  
  /* Get user options for Labrador ECC code */
  let temp= String::from("Please select LDPC code to use");
  println!("{} ", temp.bright_black());
  let selections = &["TC128"]; /*\r","TC256\r","TC512\r","TM1280\r","TM1536\r","TM2048\r","TM5120\r","TM6144\r","TM8192"];*/

	//create theme
  let my_theme = ColorfulTheme {
		defaults_style: Style::new().for_stderr().cyan(),
		prompt_style: Style::new().for_stderr().cyan().bright(),
		prompt_prefix: style("== ".to_string()).for_stderr().cyan().bright(),
		prompt_suffix: style(" ==\n----------------".to_string()).for_stderr().cyan().bright(),
		success_prefix: style("✔".to_string()).for_stderr().green().bright(),
		success_suffix: style("".to_string()).for_stderr().black().bright(),
		error_prefix: style("✘".to_string()).for_stderr().magenta().bright(),
		error_style: Style::new().for_stderr().red(),
		hint_style: Style::new().for_stderr().black().bright(),
		values_style: Style::new().for_stderr().green(),
		active_item_style: Style::new().for_stderr().black().bold().on_white(),
		inactive_item_style: Style::new().for_stderr().black().bright(),
		active_item_prefix: style("❯".to_string()).for_stderr().white().bright().blink(),
		inactive_item_prefix: style("".to_string()).for_stderr(),
		checked_item_prefix: style("✔".to_string()).for_stderr().green(),
		unchecked_item_prefix: style("✔".to_string()).for_stderr().black(),
		picked_item_prefix: style("❯".to_string()).for_stderr().green(),
		unpicked_item_prefix: style("".to_string()).for_stderr(),
		inline_selections: true,
  };

  //get codec option from user
  let selection = match Select::with_theme(&my_theme).default(0).items(&selections[..]).interact_opt(){
    Ok(s)=> s,
    Err(e)=> { println!("read_line failed: {}", e); return; }
  };
  let mut picked = String::new();
  if let Some(selection) = selection {
    print!("You selected: {}\n", selections[selection].bright_green()); 
    picked.push_str(&selections[selection].to_string().trim());
  }
  else { println!("You didn't select anything."); }

  //get input file path
  let infile_prmpt = String::from("Ok, now please give me the path to your input file: "); 
  println!("{}", infile_prmpt.bright_black());
  let _=stdout().flush();
  let mut input_file = String::new();
  match stdin().read_line(&mut input_file) {
    Ok(_)=> (),
    Err(_)=> { println!("read_line failed."); return; } 
  };
  print!("You entered: {}", input_file.bright_green());  

  //read the input file into a buffer
  let buffer = match std::fs::read(input_file.to_string().trim()) {
    Ok(v) => v,
      Err(e) => { println!("{}", e); return; },
  };

  //print out the input file buffer
  println!("{:?}", buffer);

  //prompt the user for output file option
  let mut is_out: bool = false;
  let is_ofile_prmpt = String::from("Would you like me to write the output to a file? (Y or N)");
  println!("{}", is_ofile_prmpt.bright_black()); 
  let mut is_outfile = String::new();
  match stdin().read_line(&mut is_outfile){
    Ok(_)=> (),
    Err(_)=> { println!("read_line failed."); return; }
  }
  is_outfile = is_outfile.trim().to_string();
  if is_outfile.eq("Y") || is_outfile.eq("y") {
		is_out = true;
		println!("Arg Y received. Writing output to file."); 
  }
  else if is_outfile.eq("N") || is_outfile.eq("n") {
    println!("Arg N received. Writing output to console.");
  }
  else {
    let warn = String::from("Invalid argument. Defaulting to console output."); 
    println!("{}", warn.bright_magenta()); 
  }

  //get ouput file path if needed
  let mut output_file = String::new();
  if is_out {
    let outfile_prmpt = String::from("Ok, now please give me the path to your output file: "); 
    println!("{}", outfile_prmpt.bright_black());
    let _=stdout().flush();
    match stdin().read_line(&mut output_file) {
      Ok(_)=> (),
      Err(_)=> { println!("read_line failed."); return; }
    }
    print!("You entered: {}", output_file.bright_green());
  }
  
  //////////////////////////////////////////////////////////////////////////////
  match &picked[..] {
    "TC128" => {
			if is_encode { encode_tc128(buffer, is_out, output_file); }
      else { decode_tc128(buffer, is_out, output_file); }
    },
	//////////////////////////////////////////////////////////////////////////////
   /* 
   "TC256" => {
			if is_encode { encode_tc256(buffer, is_out, output_file); }
      else { decode_tc256(buffer, is_out, output_file); }
    },
    
	  //////////////////////////////////////////////////////////////////////////////
    "TC512" => {
			if is_encode { encode_tc512(buffer, is_out, output_file); }
      else { decode_tc512(buffer, is_out, output_file); }
    },
	  //////////////////////////////////////////////////////////////////////////////
    "TM1280" => {
			if is_encode { encode_tm1280(buffer, is_out, output_file); }
      else { decode_tm1280(buffer, is_out, output_file); }
    },
	  //////////////////////////////////////////////////////////////////////////////
    "TM1536" => {
			if is_encode { encode_tm1536(buffer, is_out, output_file); }
      else { decode_tm1536(buffer, is_out, output_file); }
    },
	  //////////////////////////////////////////////////////////////////////////////
    "TM2048" => {
			if is_encode { encode_tm2048(buffer, is_out, output_file); }
      else { decode_tm2048(buffer, is_out, output_file); }
    },
	  //////////////////////////////////////////////////////////////////////////////
    "TM5120" => {
			if is_encode { encode_tm5120(buffer, is_out, output_file); }
      else { decode_tm5120(buffer, is_out, output_file); }
    },
	  //////////////////////////////////////////////////////////////////////////////
    "TM6144" => {
			if is_encode { encode_tm6144(buffer, is_out, output_file); }
      else { decode_tm6144(buffer, is_out, output_file); }
    },
	  //////////////////////////////////////////////////////////////////////////////
    "TM8192" => {
			if is_encode { encode_tm8192(buffer, is_out, output_file); }
      else { decode_tm8192(buffer, is_out, output_file); }
    },*/
    _ => { println!("Code not valid. Please see usage."); return; }
    
  }
}
//////////////////////////////////////////////////////////////////////////////
fn encode_tc128(buffer: Vec<u8>, is_out: bool, output_file: String) {
	const BLOCKSIZE: usize = 8;
	let mut blocks64: Vec<[u8; BLOCKSIZE]> = Vec::<[u8; BLOCKSIZE]>::new();
	let mut block: [u8; BLOCKSIZE] = [0; BLOCKSIZE];
	let mut count: usize = 0;
	for byte in buffer {
		block[count] = byte;
		count += 1;
		if count == BLOCKSIZE as usize {  
			blocks64.push(block);
			block = [0; BLOCKSIZE];
			count = 0;
		}
	}
	if count > 0 {
		blocks64.push(block)
	}
	let code = LDPCCode::TC128;
	let mut file_buff: Vec<u8> = Vec::new();
	let mut txcode = vec![0u8; code.n()/8];
	for b in blocks64 {
		txcode = vec![0u8; code.n()/8]; // notice its dynamically calculated based on the code
		code.copy_encode(&b, &mut txcode);  // blocks64 = 8 bytes(64 bits) ; txcode = >8bytes (64+ bits)
		//write txcode to a buffer
		for byte in &txcode {
			file_buff.push(*byte);
		}
	}
	if is_out { 
          let mut outfile = match fs::File::create(output_file.trim()) {
            Ok(o) => o,
            Err(e) => { println!("File creation failed: {}", e); return; }
          }; //this is a temporary line
	  match outfile.write_all(&file_buff) {
            Ok(_) => (),
            Err(_) => { println!("Write_all failed."); return; }
          };
	}
	else { println!("{:?}", file_buff); }
}
//////////////////////////////////////////////////////////////////////////////
fn encode_tc256(buffer: Vec<u8>, is_out: bool, output_file: String) {
	const BLOCKSIZE: usize = 16;
	let mut blocks64: Vec<[u8; BLOCKSIZE]> = Vec::<[u8; BLOCKSIZE]>::new();
	let mut block: [u8; BLOCKSIZE] = [0; BLOCKSIZE];
	let mut count: usize = 0;
	for byte in buffer {
		block[count] = byte;
		count += 1;
		if count == BLOCKSIZE as usize {  
			blocks64.push(block);
			block = [0; BLOCKSIZE];
			count = 0;
		}
	}
	if count > 0 {
		blocks64.push(block)
	}
	let code = LDPCCode::TC256;
	let mut file_buff: Vec<u8> = Vec::new();
	let mut txcode = vec![0u8; code.n()/8];
	for b in blocks64 {
		txcode = vec![0u8; code.n()/8]; // notice its dynamically calculated based on the code
		code.copy_encode(&b, &mut txcode);  // blocks64 = 8 bytes(64 bits) ; txcode = >8bytes (64+ bits)
		//write txcode to a buffer
		for byte in &txcode {
			file_buff.push(*byte);
		}
	}
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
                  Ok(o) => o,
                  Err(e) => { println!("File creation failed: {}", e); return; }
                };
		outfile.write_all(&file_buff);
	}
	else { println!("{:?}", file_buff); }
}
//////////////////////////////////////////////////////////////////////////////
fn encode_tc512(buffer: Vec<u8>, is_out: bool, output_file: String) {
	const BLOCKSIZE: usize = 32;
	let mut blocks64: Vec<[u8; BLOCKSIZE]> = Vec::<[u8; BLOCKSIZE]>::new();
	let mut block: [u8; BLOCKSIZE] = [0; BLOCKSIZE];
	let mut count: usize = 0;
	for byte in buffer {
		block[count] = byte;
		count += 1;
		if count == BLOCKSIZE as usize {  
			blocks64.push(block);
			block = [0; BLOCKSIZE];
			count = 0;
		}
	}
	if count > 0 {
		blocks64.push(block)
	}
	let code = LDPCCode::TC512;
	let mut file_buff: Vec<u8> = Vec::new();
	let mut txcode = vec![0u8; code.n()/8];
	for b in blocks64 {
		txcode = vec![0u8; code.n()/8]; // notice its dynamically calculated based on the code
		code.copy_encode(&b, &mut txcode);  // blocks64 = 8 bytes(64 bits) ; txcode = >8bytes (64+ bits)
		//write txcode to a buffer
		for byte in &txcode {
			file_buff.push(*byte);
		}
	}
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
                  Ok(o) => o,
                  Err(e) => { println!("File creation failed: {}", e); return; }
                };
		outfile.write_all(&file_buff);
	}
	else { println!("{:?}", file_buff); }
}
//////////////////////////////////////////////////////////////////////////////
fn encode_tm1280(buffer: Vec<u8>, is_out: bool, output_file: String) {
	const BLOCKSIZE: usize = 128;
	let mut blocks64: Vec<[u8; BLOCKSIZE]> = Vec::<[u8; BLOCKSIZE]>::new();
	let mut block: [u8; BLOCKSIZE] = [0; BLOCKSIZE];
	let mut count: usize = 0;
	for byte in buffer {
		block[count] = byte;
		count += 1;
		if count == BLOCKSIZE as usize { 
			blocks64.push(block);
			block = [0; BLOCKSIZE];
			count = 0;
		}
	}
	if count > 0 {
		blocks64.push(block)
	}
	let code = LDPCCode::TM1280;
	let mut file_buff: Vec<u8> = Vec::new();
	let mut txcode = vec![0u8; code.n()/8];
	for b in blocks64 {
		txcode = vec![0u8; code.n()/8]; // notice its dynamically calculated based on the code
		code.copy_encode(&b, &mut txcode);  // blocks64 = 8 bytes(64 bits) ; txcode = >8bytes (64+ bits)
		//write txcode to a buffer
		for byte in &txcode {
			file_buff.push(*byte);
		}
	}
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
                  Ok(o) => o,
                  Err(e) => { println!("File creation failed: {}", e); return; }
                };
		outfile.write_all(&file_buff);
	}
	else { println!("{:?}", file_buff); }
}
//////////////////////////////////////////////////////////////////////////////
fn encode_tm1536(buffer: Vec<u8>, is_out: bool, output_file: String) {
	const BLOCKSIZE: usize = 128;
	let mut blocks64: Vec<[u8; BLOCKSIZE]> = Vec::<[u8; BLOCKSIZE]>::new();
	let mut block: [u8; BLOCKSIZE] = [0; BLOCKSIZE];
	let mut count: usize = 0;
	for byte in buffer {
		block[count] = byte;
		count += 1;
		if count == BLOCKSIZE as usize { 
			blocks64.push(block);
			block = [0; BLOCKSIZE];
			count = 0;
		}
	}
	if count > 0 {
		blocks64.push(block)
	}
	let code = LDPCCode::TM1536;
	let mut file_buff: Vec<u8> = Vec::new();
	let mut txcode = vec![0u8; code.n()/8];
	for b in blocks64 {
		txcode = vec![0u8; code.n()/8]; // notice its dynamically calculated based on the code
		code.copy_encode(&b, &mut txcode);  // blocks64 = 8 bytes(64 bits) ; txcode = >8bytes (64+ bits)
		//write txcode to a buffer
		for byte in &txcode {
			file_buff.push(*byte);
		}
	}
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()){
                  Ok(o) => o,
                  Err(e) => { println!("File creation failed: {}", e); return; }
                };
		outfile.write_all(&file_buff);
	}
	else { println!("{:?}", file_buff); }
}
//////////////////////////////////////////////////////////////////////////////
fn encode_tm2048(buffer: Vec<u8>, is_out: bool, output_file: String) {
	const BLOCKSIZE: usize = 128;
	let mut blocks64: Vec<[u8; BLOCKSIZE]> = Vec::<[u8; BLOCKSIZE]>::new();
	let mut block: [u8; BLOCKSIZE] = [0; BLOCKSIZE];
	let mut count: usize = 0;
	for byte in buffer {
		block[count] = byte;
		count += 1;
		if count == BLOCKSIZE as usize { 
			blocks64.push(block);
			block = [0; BLOCKSIZE];
			count = 0;
		}
	}
	if count > 0 {
		blocks64.push(block)
	}
	let code = LDPCCode::TM2048;
	let mut file_buff: Vec<u8> = Vec::new();
	let mut txcode = vec![0u8; code.n()/8];
	for b in blocks64 {
		txcode = vec![0u8; code.n()/8]; // notice its dynamically calculated based on the code
		code.copy_encode(&b, &mut txcode);  // blocks64 = 8 bytes(64 bits) ; txcode = >8bytes (64+ bits)
		//write txcode to a buffer
		for byte in &txcode {
			file_buff.push(*byte);
		}
	}
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
                  Ok(o) => o,
                  Err(e) => { println!("File creation failed: {}", e); return; }
                };
		outfile.write_all(&file_buff);
	}
	else { println!("{:?}", file_buff); }
}
//////////////////////////////////////////////////////////////////////////////
fn encode_tm5120(buffer: Vec<u8>, is_out: bool, output_file: String) {
	const BLOCKSIZE: usize = 512;
	let mut blocks64: Vec<[u8; BLOCKSIZE]> = Vec::<[u8; BLOCKSIZE]>::new();
	let mut block: [u8; BLOCKSIZE] = [0; BLOCKSIZE];
	let mut count: usize = 0;
	for byte in buffer {
		block[count] = byte;
		count += 1;
		if count == BLOCKSIZE as usize { 
			blocks64.push(block);
			block = [0; BLOCKSIZE];
			count = 0;
		}
	}
	if count > 0 {
		blocks64.push(block)
	}
	let code = LDPCCode::TM5120;
	let mut file_buff: Vec<u8> = Vec::new();
	let mut txcode = vec![0u8; code.n()/8];
	for b in blocks64 {
		txcode = vec![0u8; code.n()/8]; // notice its dynamically calculated based on the code
		code.copy_encode(&b, &mut txcode);  // blocks64 = 8 bytes(64 bits) ; txcode = >8bytes (64+ bits)
		//write txcode to a buffer
		for byte in &txcode {
			file_buff.push(*byte);
		}
	}
	if is_out { 
		let mut outfile = fs::File::create(output_file.trim()).unwrap(); //this is a temporary line
		outfile.write_all(&file_buff);
	}
	else { println!("{:?}", file_buff); }
}
//////////////////////////////////////////////////////////////////////////////
fn encode_tm6144(buffer: Vec<u8>, is_out: bool, output_file: String) {
	const BLOCKSIZE: usize = 512;
	let mut blocks64: Vec<[u8; BLOCKSIZE]> = Vec::<[u8; BLOCKSIZE]>::new();
	let mut block: [u8; BLOCKSIZE] = [0; BLOCKSIZE];
	let mut count: usize = 0;
	for byte in buffer {
		block[count] = byte;
		count += 1;
		if count == BLOCKSIZE as usize { 
			blocks64.push(block);
			block = [0; BLOCKSIZE];
			count = 0;
		}
	}
	if count > 0 {
		blocks64.push(block)
	}
	let code = LDPCCode::TM6144;
	let mut file_buff: Vec<u8> = Vec::new();
	let mut txcode = vec![0u8; code.n()/8];
	for b in blocks64 {
		txcode = vec![0u8; code.n()/8]; // notice its dynamically calculated based on the code
		code.copy_encode(&b, &mut txcode);  // blocks64 = 8 bytes(64 bits) ; txcode = >8bytes (64+ bits)
		//write txcode to a buffer
		for byte in &txcode {
			file_buff.push(*byte);
		}
	}
	if is_out { 
		let mut outfile = fs::File::create(output_file.trim()).unwrap(); //this is a temporary line
		outfile.write_all(&file_buff);
	}
	else { println!("{:?}", file_buff); }
}
//////////////////////////////////////////////////////////////////////////////
fn encode_tm8192(buffer: Vec<u8>, is_out: bool, output_file: String) {
	const BLOCKSIZE: usize = 512;
	let mut blocks64: Vec<[u8; BLOCKSIZE]> = Vec::<[u8; BLOCKSIZE]>::new();
	let mut block: [u8; BLOCKSIZE] = [0; BLOCKSIZE];
	let mut count: usize = 0;
	for byte in buffer {
		block[count] = byte;
		count += 1;
		if count == BLOCKSIZE as usize {  
			blocks64.push(block);
			block = [0; BLOCKSIZE];
			count = 0;
		}
	}
	if count > 0 {
		blocks64.push(block)
	}
	let code = LDPCCode::TM8192;
	let mut file_buff: Vec<u8> = Vec::new();
	let mut txcode = vec![0u8; code.n()/8];
	for b in blocks64 {
		txcode = vec![0u8; code.n()/8]; // notice its dynamically calculated based on the code
		code.copy_encode(&b, &mut txcode);  // blocks64 = 8 bytes(64 bits) ; txcode = >8bytes (64+ bits)
		//write txcode to a buffer
		for byte in &txcode {
			file_buff.push(*byte);
		}
	}
	if is_out { 
		let mut outfile = fs::File::create(output_file.trim()).unwrap(); //this is a temporary line
		outfile.write_all(&file_buff);
	}
	else { println!("{:?}", file_buff); }
}
//////////////////////////////////////////////////////////////////////////////
fn decode_tc128(buffer: Vec<u8>, is_out: bool, output_file: String) {
  //create codec struct
  let code = LDPCCode::TC128;
  //working data buffer
  let mut working = vec![0u8; code.decode_bf_working_len()];
  //recieving data buffer
  let mut rxdata = vec![0u8; code.output_len()];
	//buffer to write to the output file
  let mut file_buff: Vec<u8> = Vec::new();
  //counter used to break up the input buffer
  let mut count: i32 = 0;
  //block of data from the input buffer
  let mut block: Vec<u8> = Vec::new();
	for b in buffer {
    working = vec![0u8; code.decode_bf_working_len()]; //clear working buffer
    rxdata = vec![0u8; code.output_len()]; //clear recieving buffer
    let block_size = code.n()/8; //calculate the block size from the codec
    block.push(b); //add byte to the block
	  count += 1;
    if count == block_size as i32 { //entire block is filled
      code.decode_bf(&block, &mut rxdata, &mut working, 20); //decode the block
      let snip = (code.n()/8) - 8; //get the index of ecc to be snipped off
      //snip off the ecc data and push to output buffer
      for byte in &rxdata[..snip] {
        file_buff.push(*byte);
      }
      block = Vec::new(); //reset block
      count = 0; //reset count
    }
	}
  //get the index of the last non-zero byte in the output buffer
  let mut indx: usize = 0; 
  for i in (0..file_buff.len()).rev() {
    if file_buff[i] != 0 { 
      indx = i;
      break;
    }
  }

  //write to file
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
      Ok(f) => f,
      Err(e) => {
        println!("Unable to create output file : {} Error : {}", output_file, e);
        return;
      }
    };
		//write all bytes to a file EXCEPT the padding bytes at the end of the buffer
    match outfile.write_all(&file_buff[..indx + 1]) {
      Ok(_) => {},
      Err(e) => {
        println!("Unable to write data to the output file : {} Error : {}", output_file, e);
        return;
      }
    }
	}
  //print to terminal
	else { println!("{:?}", file_buff); }
}

//////////////////////////////////////////////////////////////////////////////
fn decode_tc256(buffer: Vec<u8>, is_out: bool, output_file: String) {
  //create codec struct
  let code = LDPCCode::TC256;
  const BLOCKSIZE: usize = 16;
  //working data buffer
  let mut working = vec![0u8; code.decode_bf_working_len()];
  //recieving data buffer
  let mut rxdata = vec![0u8; code.output_len()];
	//buffer to write to the output file
  let mut file_buff: Vec<u8> = Vec::new();
  //counter used to break up the input buffer
  let mut count: i32 = 0;
  //block of data from the input buffer
  let mut block: Vec<u8> = Vec::new();
	for b in buffer {
    working = vec![0u8; code.decode_bf_working_len()]; //clear working buffer
    rxdata = vec![0u8; code.output_len()]; //clear recieving buffer
    let block_size = code.n()/BLOCKSIZE; //calculate the block size from the codec
    block.push(b); //add byte to the block
	  count += 1;
    if count == block_size as i32 { //entire block is filled
      code.decode_bf(&block, &mut rxdata, &mut working, 20); //decode the block
      let snip = (code.n()/BLOCKSIZE) - BLOCKSIZE; //get the index of ecc to be snipped off
      //snip off the ecc data and push to output buffer
      for byte in &rxdata[..snip] {
        file_buff.push(*byte);
      }
      block = Vec::new(); //reset block
      count = 0; //reset count
    }
	}
  //get the index of the last non-zero byte in the output buffer
  let mut indx: usize = 0; 
  for i in (0..file_buff.len()).rev() {
    if file_buff[i] != 0 { 
      indx = i;
      break;
    }
  }

  //write to file
  if is_out { 
    let mut outfile = match fs::File::create(output_file.trim()) {
      Ok(f) => f,
      Err(e) => {
	println!("Unable to create output file : {} Error : {}", output_file, e);
	return;
      }
   };
		//write all bytes to a file EXCEPT the padding bytes at the end of the buffer
    match outfile.write_all(&file_buff[..indx + 1]) {
      Ok(_) => {},
      Err(e) => {
        println!("Unable to write data to the output file : {} Error : {}", output_file, e);
        return;
      }
    }
	}
  //print to terminal
  else { println!("{:?}", file_buff); }
}

fn decode_tc512(buffer: Vec<u8>, is_out: bool, output_file: String) {
  //create codec struct
  let code = LDPCCode::TC512;
  //working data buffer
  let mut working = vec![0u8; code.decode_bf_working_len()];
  //recieving data buffer
  let mut rxdata = vec![0u8; code.output_len()];
	//buffer to write to the output file
  let mut file_buff: Vec<u8> = Vec::new();
  //counter used to break up the input buffer
  let mut count: i32 = 0;
  //block of data from the input buffer
  let mut block: Vec<u8> = Vec::new();
	for b in buffer {
    working = vec![0u8; code.decode_bf_working_len()]; //clear working buffer
    rxdata = vec![0u8; code.output_len()]; //clear recieving buffer
    let block_size = code.n()/8; //calculate the block size from the codec
    block.push(b); //add byte to the block
	  count += 1;
    if count == block_size as i32 { //entire block is filled
      code.decode_bf(&block, &mut rxdata, &mut working, 20); //decode the block
      let snip = (code.n()/8) - 8; //get the index of ecc to be snipped off
      //snip off the ecc data and push to output buffer
      for byte in &rxdata[..snip] {
        file_buff.push(*byte);
      }
      block = Vec::new(); //reset block
      count = 0; //reset count
    }
	}
  //get the index of the last non-zero byte in the output buffer
  let mut indx: usize = 0; 
  for i in (0..file_buff.len()).rev() {
    if file_buff[i] != 0 { 
      indx = i;
      break;
    }
  }

  //write to file
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
      Ok(f) => f,
      Err(e) => {
        println!("Unable to create output file : {} Error : {}", output_file, e);
        return;
      }
    };
		//write all bytes to a file EXCEPT the padding bytes at the end of the buffer
    match outfile.write_all(&file_buff[..indx + 1]) {
      Ok(_) => {},
      Err(e) => {
        println!("Unable to write data to the output file : {} Error : {}", output_file, e);
        return;
      }
    }
	}
  //print to terminal
	else { println!("{:?}", file_buff); }
}

fn decode_tm1280(buffer: Vec<u8>, is_out: bool, output_file: String) {
  //create codec struct
  let code = LDPCCode::TM1280;
  //working data buffer
  let mut working = vec![0u8; code.decode_bf_working_len()];
  //recieving data buffer
  let mut rxdata = vec![0u8; code.output_len()];
	//buffer to write to the output file
  let mut file_buff: Vec<u8> = Vec::new();
  //counter used to break up the input buffer
  let mut count: i32 = 0;
  //block of data from the input buffer
  let mut block: Vec<u8> = Vec::new();
	for b in buffer {
    working = vec![0u8; code.decode_bf_working_len()]; //clear working buffer
    rxdata = vec![0u8; code.output_len()]; //clear recieving buffer
    let block_size = code.n()/8; //calculate the block size from the codec
    block.push(b); //add byte to the block
	  count += 1;
    if count == block_size as i32 { //entire block is filled
      code.decode_bf(&block, &mut rxdata, &mut working, 20); //decode the block
      let snip = (code.n()/8) - 8; //get the index of ecc to be snipped off
      //snip off the ecc data and push to output buffer
      for byte in &rxdata[..snip] {
        file_buff.push(*byte);
      }
      block = Vec::new(); //reset block
      count = 0; //reset count
    }
	}
  //get the index of the last non-zero byte in the output buffer
  let mut indx: usize = 0; 
  for i in (0..file_buff.len()).rev() {
    if file_buff[i] != 0 { 
      indx = i;
      break;
    }
  }

  //write to file
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
      Ok(file) => file,
      Err(err) => {
        println!("Unable to create output file : {} Error : {}",output_file,err);
        return;
      }
    };
		//write all bytes to a file EXCEPT the padding bytes at the end of the buffer
    match outfile.write_all(&file_buff[..indx + 1]) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to write data to the output file : {} Error : {}",output_file,err);
        return;
      }
    }
	}
  //print to terminal
	else { println!("{:?}", file_buff); }
}

fn decode_tm1536(buffer: Vec<u8>, is_out: bool, output_file: String) {
   //create codec struct
   let code = LDPCCode::TM1536;
    //working data buffer
    let mut working = vec![0u8; code.decode_bf_working_len()];
    //recieving data buffer
    let mut rxdata = vec![0u8; code.output_len()];
    //buffer to write to the output file
    let mut file_buff: Vec<u8> = Vec::new();
    //counter used to break up the input buffer
    let mut count: i32 = 0;
    //block of data from the input buffer
    let mut block: Vec<u8> = Vec::new();
    for b in buffer {
      working = vec![0u8; code.decode_bf_working_len()]; //clear working buffer
      rxdata = vec![0u8; code.output_len()]; //clear recieving buffer
      let block_size = code.n()/8; //calculate the block size from the codec
      block.push(b); //add byte to the block
      count += 1;
      if count == block_size as i32 { //entire block is filled
        code.decode_bf(&block, &mut rxdata, &mut working, 20); //decode the block
        let snip = (code.n()/8) - 8; //get the index of ecc to be snipped off
        //snip off the ecc data and push to output buffer
        for byte in &rxdata[..snip] {
          file_buff.push(*byte);
        }
        block = Vec::new(); //reset block
        count = 0; //reset count
      }
    }
    //get the index of the last non-zero byte in the output buffer
    let mut indx: usize = 0; 
    for i in (0..file_buff.len()).rev() {
      if file_buff[i] != 0 { 
        indx = i;
        break;
      }
    }
  
    //write to file
    if is_out { 
      let mut outfile = match fs::File::create(output_file.trim()) {
        Ok(file) => file,
        Err(err) => {
          println!("Unable to create output file : {} Error : {}",output_file,err);
          return;
        }
      };
      //write all bytes to a file EXCEPT the padding bytes at the end of the buffer
      match outfile.write_all(&file_buff[..indx + 1]) {
        Ok(_) => {},
        Err(err) => {
          println!("Unable to write data to the output file : {} Error : {}",output_file,err);
          return;
        }
      }
    }
    //print to terminal
    else { println!("{:?}", file_buff); }
}

fn decode_tm2048(buffer: Vec<u8>, is_out: bool, output_file: String) {
  //create codec struct
  let code = LDPCCode::TM2048;
  //working data buffer
  let mut working = vec![0u8; code.decode_bf_working_len()];
  //recieving data buffer
  let mut rxdata = vec![0u8; code.output_len()];
	//buffer to write to the output file
  let mut file_buff: Vec<u8> = Vec::new();
  //counter used to break up the input buffer
  let mut count: i32 = 0;
  //block of data from the input buffer
  let mut block: Vec<u8> = Vec::new();
	for b in buffer {
    working = vec![0u8; code.decode_bf_working_len()]; //clear working buffer
    rxdata = vec![0u8; code.output_len()]; //clear recieving buffer
    let block_size = code.n()/8; //calculate the block size from the codec
    block.push(b); //add byte to the block
	  count += 1;
    if count == block_size as i32 { //entire block is filled
      code.decode_bf(&block, &mut rxdata, &mut working, 20); //decode the block
      let snip = (code.n()/8) - 8; //get the index of ecc to be snipped off
      //snip off the ecc data and push to output buffer
      for byte in &rxdata[..snip] {
        file_buff.push(*byte);
      }
      block = Vec::new(); //reset block
      count = 0; //reset count
    }
	}
  //get the index of the last non-zero byte in the output buffer
  let mut indx: usize = 0; 
  for i in (0..file_buff.len()).rev() {
    if file_buff[i] != 0 { 
      indx = i;
      break;
    }
  }

  //write to file
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
      Ok(file) => file,
      Err(err) => {
        println!("Unable to create output file : {} Error : {}",output_file,err);
        return;
      }
    };
		//write all bytes to a file EXCEPT the padding bytes at the end of the buffer
    match outfile.write_all(&file_buff[..indx + 1]) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to write data to the output file : {} Error : {}",output_file,err);
        return;
      }
    }
	}
  //print to terminal
	else { println!("{:?}", file_buff); }
}

fn decode_tm5120(buffer: Vec<u8>, is_out: bool, output_file: String) {
  //create codec struct
  let code = LDPCCode::TM5120;
  //working data buffer
  let mut working = vec![0u8; code.decode_bf_working_len()];
  //recieving data buffer
  let mut rxdata = vec![0u8; code.output_len()];
	//buffer to write to the output file
  let mut file_buff: Vec<u8> = Vec::new();
  //counter used to break up the input buffer
  let mut count: i32 = 0;
  //block of data from the input buffer
  let mut block: Vec<u8> = Vec::new();
	for b in buffer {
    working = vec![0u8; code.decode_bf_working_len()]; //clear working buffer
    rxdata = vec![0u8; code.output_len()]; //clear recieving buffer
    let block_size = code.n()/8; //calculate the block size from the codec
    block.push(b); //add byte to the block
	  count += 1;
    if count == block_size as i32 { //entire block is filled
      code.decode_bf(&block, &mut rxdata, &mut working, 20); //decode the block
      let snip = (code.n()/8) - 8; //get the index of ecc to be snipped off
      //snip off the ecc data and push to output buffer
      for byte in &rxdata[..snip] {
        file_buff.push(*byte);
      }
      block = Vec::new(); //reset block
      count = 0; //reset count
    }
	}
  //get the index of the last non-zero byte in the output buffer
  let mut indx: usize = 0; 
  for i in (0..file_buff.len()).rev() {
    if file_buff[i] != 0 { 
      indx = i;
      break;
    }
  }

  //write to file
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
      Ok(file) => file,
      Err(err) => {
        println!("Unable to create output file : {} Error : {}",output_file,err);
        return;
      }
    };
		//write all bytes to a file EXCEPT the padding bytes at the end of the buffer
    match outfile.write_all(&file_buff[..indx + 1]) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to write data to the output file : {} Error : {}",output_file,err);
        return;
      }
    }
	}
  //print to terminal
	else { println!("{:?}", file_buff); }
}

fn decode_tm6144(buffer: Vec<u8>, is_out: bool, output_file: String) {
  //create codec struct
  let code = LDPCCode::TM6144;
  //working data buffer
  let mut working = vec![0u8; code.decode_bf_working_len()];
  //recieving data buffer
  let mut rxdata = vec![0u8; code.output_len()];
	//buffer to write to the output file
  let mut file_buff: Vec<u8> = Vec::new();
  //counter used to break up the input buffer
  let mut count: i32 = 0;
  //block of data from the input buffer
  let mut block: Vec<u8> = Vec::new();
	for b in buffer {
    working = vec![0u8; code.decode_bf_working_len()]; //clear working buffer
    rxdata = vec![0u8; code.output_len()]; //clear recieving buffer
    let block_size = code.n()/8; //calculate the block size from the codec
    block.push(b); //add byte to the block
	  count += 1;
    if count == block_size as i32 { //entire block is filled
      code.decode_bf(&block, &mut rxdata, &mut working, 20); //decode the block
      let snip = (code.n()/8) - 8; //get the index of ecc to be snipped off
      //snip off the ecc data and push to output buffer
      for byte in &rxdata[..snip] {
        file_buff.push(*byte);
      }
      block = Vec::new(); //reset block
      count = 0; //reset count
    }
	}
  //get the index of the last non-zero byte in the output buffer
  let mut indx: usize = 0; 
  for i in (0..file_buff.len()).rev() {
    if file_buff[i] != 0 { 
      indx = i;
      break;
    }
  }

  //write to file
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
      Ok(file) => file,
      Err(err) => {
        println!("Unable to create output file : {} Error : {}",output_file,err);
        return;
      }
    };
		//write all bytes to a file EXCEPT the padding bytes at the end of the buffer
    match outfile.write_all(&file_buff[..indx + 1]) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to write data to the output file : {} Error : {}",output_file,err);
        return;
      }
    }
	}
  //print to terminal
	else { println!("{:?}", file_buff); }
}

fn decode_tm8192(buffer: Vec<u8>, is_out: bool, output_file: String) {
  //create codec struct
  let code = LDPCCode::TM8192;
  //working data buffer
  let mut working = vec![0u8; code.decode_bf_working_len()];
  //recieving data buffer
  let mut rxdata = vec![0u8; code.output_len()];
	//buffer to write to the output file
  let mut file_buff: Vec<u8> = Vec::new();
  //counter used to break up the input buffer
  let mut count: i32 = 0;
  //block of data from the input buffer
  let mut block: Vec<u8> = Vec::new();
	for b in buffer {
    working = vec![0u8; code.decode_bf_working_len()]; //clear working buffer
    rxdata = vec![0u8; code.output_len()]; //clear recieving buffer
    let block_size = code.n()/8; //calculate the block size from the codec
    block.push(b); //add byte to the block
	  count += 1;
    if count == block_size as i32 { //entire block is filled
      code.decode_bf(&block, &mut rxdata, &mut working, 20); //decode the block
      let snip = (code.n()/8) - 8; //get the index of ecc to be snipped off
      //snip off the ecc data and push to output buffer
      for byte in &rxdata[..snip] {
        file_buff.push(*byte);
      }
      block = Vec::new(); //reset block
      count = 0; //reset count
    }
	}
  //get the index of the last non-zero byte in the output buffer
  let mut indx: usize = 0; 
  for i in (0..file_buff.len()).rev() {
    if file_buff[i] != 0 { 
      indx = i;
      break;
    }
  }

  //write to file
	if is_out { 
		let mut outfile = match fs::File::create(output_file.trim()) {
      Ok(file) => file,
      Err(err) => {
        println!("Unable to create output file : {} Error : {}",output_file,err);
        return;
      }
    };
		//write all bytes to a file EXCEPT the padding bytes at the end of the buffer
    match outfile.write_all(&file_buff[..indx + 1]) {
      Ok(_) => {},
      Err(err) => {
        println!("Unable to write data to the output file : {} Error : {}",output_file,err);
        return;
      }
    }
	}
  //print to terminal
	else { println!("{:?}", file_buff); }
}
