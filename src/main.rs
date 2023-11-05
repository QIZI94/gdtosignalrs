
use std::path::PathBuf;
use std::process;
use clap::Parser;
use scan_dir::ScanDir;

mod parse_rust;
mod generate_rust;

use parse_rust::*;


const ABOUT: &str = 
"
Simple tool that looks for [signal] and [func] godot attributed functions in files inside input directory.
These would normally be accessed as string e.g \"player_signal\",
but instead output directory will be filled with files containing constants in same-named structs and functions.

Example usage in code: emit(signal::Player::player_signal.into())";

/// Simple tool that looks for [signal] and [func] godot attributed functions in files inside input directory.
/// These would normally be accessed as string e.g "player_signal",
/// but instead output directory will be filled with files containing constants in same-named structs and functions.
/// Example usage in code: emit(signal::Player::player_signal.into())
#[derive(Parser, Debug)]
#[command(author, version, about=ABOUT, long_about = None)]
struct Args {
	/// Directory which will be recursively scanned for files containing [signal] and [func] godot attributed functions
	#[arg(short, long)]
	input_directory: PathBuf,
	/// Outputs signal.rs, func.rs and mod.rs into chosen directory
	#[arg(short, long)]
	output_directory: PathBuf,
}

fn main(){

	let args = Args::parse();

	if !args.input_directory.is_dir(){
		eprintln!("Error input-directory path is not directory!");
		process::exit(1);
	}

	if args.output_directory.exists() && !args.output_directory.is_dir() {
		eprintln!("Error output-directory path is not directory!");
		process::exit(1);
	}

	let files: Vec<PathBuf> = ScanDir::files().walk(args.input_directory.clone(), |iter| {
		iter.filter(|&(_, ref name)| name.ends_with(".rs"))
			.map(|(entry, _)| entry.path())
			.collect()
	}).unwrap();

	// make paths strings
	let files_str: Vec<String> = files.iter()
		.map(|path| String::from(path.to_str().unwrap()))
		.collect();
	
	


	let structs_and_functions_result = parse_rust_from_multiple_file(&files_str);
	let structs_and_functions = structs_and_functions_result.unwrap();
	
	let mut output_path: String = args.output_directory.to_str().unwrap().into();
	if !output_path.ends_with("/") {
		output_path.push('/');
	}

	generate_rust::generate_rust_module(&output_path, &structs_and_functions).unwrap();



}