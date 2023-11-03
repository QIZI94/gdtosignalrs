use std::fs::File;
use std::fs::create_dir_all;
use std::io::prelude::*;
use std::io::Error;
use std::collections::HashMap;

use crate::parse_rust::{FunctionType, StructAndFunctions};

type FunctionsOfSameType = Vec<(String, usize)>;
type FunctionsInStruct = HashMap<String, FunctionsOfSameType>;

const GENERATED_FUNCTION_ALIAS_STRUCT: &str =
"
pub struct GeneratedFunctionAlias{
	pub func: &'static str,
	pub arg_count: usize
}
\n
";

fn create_structs_and_functions(functions_in_structs: &FunctionsInStruct) -> String {
	const WARNING_SUPPRESSION: &str = "#[allow(non_upper_case_globals)]\n";
	const BEGIN: &str = "{\n";
	const END: &str = "}\n\n";
	const STRUCT: &str = "pub struct ";
	const IMPL: &str = "impl ";
	let mut source = String::new();
	source.push_str(GENERATED_FUNCTION_ALIAS_STRUCT);

	for (struct_name, functions) in functions_in_structs{
		source.push_str(STRUCT);

		source.push_str(struct_name);
		source.push_str(";\n");
		
		source.push_str(IMPL);
		source.push_str(struct_name);
		source.push_str(BEGIN);
		for (function_name, arg_count) in functions{
			source.push_str("\t");
			
			source.push_str(WARNING_SUPPRESSION);
			source.push_str("\t");
			source.push_str("pub const ");

			source.push_str(function_name);
			source.push_str(": GeneratedFunctionAlias = ");
			source.push_str("GeneratedFunctionAlias{\n");

			source.push_str("\t\tfunc: \"");
			source.push_str(function_name);
			source.push_str("\",\n");
			source.push_str("\t\targ_count: ");
			source.push_str(&arg_count.to_string());
			source.push_str("\n\t};\n\n");

		}

		source.push_str(END);
	}

	source
}


pub fn generate_rust_module(dir_path: &String, structs_and_functions: &StructAndFunctions) -> Result<(), Error>{

	create_dir_all(dir_path)?;
	
	let mod_path = dir_path.clone() + "mod.rs";
	let signal_path = dir_path.clone() + "signal.rs";
	let func_path = dir_path.clone() + "func.rs";

	
	let signals: FunctionsInStruct = structs_and_functions.iter().filter_map(
		|(key,	functions)|{
			let mut functions_array = FunctionsOfSameType::new();
			for func in functions {
				if let FunctionType::Signal(name, arg_count) = func{
					functions_array.push((name.clone(), *arg_count))
				}
			}
			if functions_array.is_empty(){
				return None
			}

			Some((key.clone(), functions_array))		
		}
	).collect::<HashMap<_, _>>();

	let funcs: FunctionsInStruct = structs_and_functions.iter().filter_map(
		|(key,	functions)|{
			let mut functions_array = FunctionsOfSameType::new();
			for func in functions {
				if let FunctionType::Func(name, arg_count) = func{
					functions_array.push((name.clone(), *arg_count))
				}
			}
			if functions_array.is_empty(){
				return None
			}

			Some((key.clone(), functions_array))		
		}
	).collect::<HashMap<_, _>>();
	//for (struct_key, functions) in structs_and_functions{

	//}
	
	let funcs_content = create_structs_and_functions(&funcs);
	let mut funcs_file = File::create(func_path)?;
	funcs_file.write(funcs_content.as_bytes())?;

	let signals_content = create_structs_and_functions(&signals);
	let mut singals_file = File::create(signal_path)?;
	singals_file.write(signals_content.as_bytes())?;

	let mod_content: &str = "pub mod signal;\npub mod func;\n";
	let mut mod_file = File::create(mod_path)?;
	mod_file.write(mod_content.as_bytes())?;
	
	Ok(())
}