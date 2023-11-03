
use std::path::PathBuf;
use clap::Parser;
use scan_dir::ScanDir;

mod parse_rust;
mod generate_rust;

use parse_rust::*;



/// Simple tool that looks for [signal] and [func] godot attributed functions in files inside input directory.
/// These would normally be accessed as string e.g "player_signal",
/// but instead output directory will be filled with files containing constants in same-named structs and functions.
/// Example usage in code: emit(signal::Player::player_signal.into())
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
		println!("input-directory path is not directory!");
	}

	if !args.output_directory.is_dir(){
		println!("output-directory path is not directory!");
	}

	let files: Vec<PathBuf> = ScanDir::files().read(args.input_directory.clone(), |iter| {
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
	
	generate_rust::generate_rust_module(&String::from(args.output_directory.to_str().unwrap()), &structs_and_functions).unwrap();



}