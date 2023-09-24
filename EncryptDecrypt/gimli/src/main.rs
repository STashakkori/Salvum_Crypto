// -m0nZSt3r -Matzr3lla -$t@$h     QVLx Labs

use std::process::Command;
use std::env;
use std::path::Path;
/*
	
	gimli v1: it is a simple more elegant version of gimli_rs, which has garbage usage and is not easy to use
	 - There are a few more additional features of gimli_rs that are not implemented as they did not seem necessary or useful
	
	TODO: 
		multiple file encryption and decryption seems doable, just would need to implement in second version
		
	examples: cargo run -- hash -file test.txt output.txt                              (hashes test.txt contents into output.txt)
						cargo run -- hash -input letsgoooo output.txt                            (hashes input string into output.txt)
						cargo run -- encrypt skeydaddle -file test.txt outenc.txt                (encrypts contents of test.txt into outenc.txt using the key skeydaddle)
						cargo run -- encrypt skeydaddle -input letsgooo outenc.txt -length 64    (encrypts input into outenc.txt using the key skeydaddle and a custom length of 64, default is 32)
						cargo run -- decrypt skeydaddle -file outenc.txt outdec.txt              (decrypts contents of encrypted outenc.txt into outdec.txt using the key skeydaddle)
*/
fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 5 || args.len() > 9 {
			println!("  Usage: gimli operation [-arguments] ");
			println!("               hash (-file,-input) input output [-length]");
			println!("               encrypt keyname (-file,-input) input output [-length]");
			println!("               decrypt keyname (-file,-input) input output [-length]");
			println!("                                                           default=32");
			return; 
	}
	match args[1].trim() {
		"hash" => hash(args),
		"encrypt" => encrypt(args),
		"decrypt" => decrypt(args),
		_ => return,
	};
}

fn hash(vec: Vec<String>) {
	let in_type = vec[2].trim().to_string();
	let input = vec[3].trim().to_string();
	let output = vec[4].trim().to_string();
	let mut args_list: Vec<String> = Vec::new();
	
	// check for optional length flag, if so push onto vec
	if vec.len() < 8 && vec.len() > 6 {
		args_list.push("-l".to_string());
		args_list.push(vec[6].trim().to_string());
	}
	// check for either -file or -input and push onto vec
	if in_type.contains("-file") {
		if path_check(input.clone()) {
		args_list.push("-f".to_string());
		}
	}
	else if !in_type.contains("-input") {
		return;
	}
	// complete the rest of the hash arguments and execute
	args_list.push("--input".to_string());
	args_list.push(input);
	args_list.push("-o".to_string());
	args_list.push(output);
	execute(args_list);
}

fn encrypt(vec: Vec<String>) {
	// double check theres at least the min num of args
	if vec.len() < 6 {return;}	
	let keyname = vec[2].trim().to_string();
	let in_type = vec[3].trim().to_string();
	let input = vec[4].trim().to_string();
	let output = vec[5].trim().to_string();
	let mut args_list: Vec<String> = Vec::new();
	
	// check for optional length flag, if so push onto vec	
	if vec.len() < 9 && vec.len() > 7 {
		args_list.push("-l".to_string());
		args_list.push(vec[7].trim().to_string());
	}

	// verify the key exists and push onto vec
	if !path_check(keyname.clone()) {return;}
	args_list.push("-k".to_string());
	args_list.push(keyname);	

	// check for -file or -input and push onto vec
	if in_type.contains("-file") {
		if path_check(input.clone()) {
		args_list.push("-f".to_string());
		}
	}
	else if !in_type.contains("-input") {
		return;
	}	
	
	// push rest of args onto vec and execute
	args_list.push("-m".to_string());	
	args_list.push("encrypt".to_string());	
	args_list.push("--input".to_string());
	args_list.push(input);
	args_list.push("-o".to_string());
	args_list.push(output);
	execute(args_list);
	
}

fn decrypt(vec: Vec<String>) {
	// double check theres at least the min num of args
	if vec.len() < 6 {return;}
	let keyname = vec[2].trim().to_string();
	let in_type = vec[3].trim().to_string();
	let input = vec[4].trim().to_string();
	let output = vec[5].trim().to_string();
	let mut args_list: Vec<String> = Vec::new();

	if vec.len() < 9 && vec.len() > 7 {
		args_list.push("-l".to_string());
		args_list.push(vec[7].trim().to_string());
	}

	// verify the key exists and push onto vec
	if !path_check(keyname.clone()) {return;}
	args_list.push("-k".to_string());
	args_list.push(keyname);	

	// check for -file or -input and push onto vec
	if in_type.contains("-file") {
		if path_check(input.clone()) {
		args_list.push("-f".to_string());
		}
	}
	else if !in_type.contains("-input") {
		return;
	}
	
	// push rest of args onto vec and execute
	args_list.push("-m".to_string());	
	args_list.push("decrypt".to_string());	
	args_list.push("--input".to_string());
	args_list.push(input);
	args_list.push("-o".to_string());
	args_list.push(output);
	execute(args_list);
}

fn execute(vec: Vec<String>) {
	let out = match Command::new("gimli_rs").args(vec).output() {
			Ok(out) => out,
			Err(err) => {
				println!("Unable to execute gimli. Error: {}",err);
				return
			}
	};
		
	let stdout_str = String::from_utf8_lossy(&out.stdout);
	let stderr_str = String::from_utf8_lossy(&out.stderr);	
	if stderr_str != "" {
		print!("{}", stderr_str);
	}
	else {
		print!("{}", stdout_str);
	}
}

fn path_check(path: String) -> bool {
	let ppath = Path::new(&path);
	if ppath.exists() {
		return true;
	}
	else {
		return false;
	}
}