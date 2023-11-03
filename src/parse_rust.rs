use std::error::Error;
use std::fs::File;
use std::io::Read;
use syn::FnArg;
use std::collections::HashMap;

pub enum FunctionType{
	Func(String, usize),
	Signal(String, usize)
}

pub type StructAndFunctions = HashMap<String, Vec<FunctionType>>;

pub fn parse_rust_file(structs_and_functions: &mut StructAndFunctions, path: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let ast = syn::parse_file(&content)?;
    /*if let Some(shebang) = ast.shebang {
        println!("{}", shebang);
    }*/

	//let mut structs_and_functions = StructAndFunctions::new();

	for item in ast.items.iter() {
		if let syn::Item::Impl(impl_block) = item{

			if let syn::Type::Path(type_path) = impl_block.self_ty.as_ref() {
				//println!("struct: {:?}", type_path.path.segments.last().unwrap().ident.to_string());
			
				let struct_name = type_path.path.segments.last().unwrap().ident.to_string();
			
			/*/
			if let Some((_,path, _)) = &impl_block.trait_ {
	
				println!("Block: {:?}", path.segments.last().unwrap().ident.to_string()); 
			}*/

			

				for item in impl_block.items.iter(){
					if let syn::ImplItem::Fn(func) = item{
						if let Some(att) = func.attrs.last(){
							//println!("a {:?}", att.path().segments.last().unwrap().ident.to_string());
							let attributes = &att.path().segments;
							
							let attribute = {
								let maybe_attribute = attributes.iter().find(
									|&value| {
										let attr_ident = value.ident.to_string();
										"signal" ==  attr_ident || "func" == attr_ident
									}
								);

								if let Some(att) = maybe_attribute{
									att.ident.to_string()
								}
								else {
									continue;
								}
							};
							
							let struct_entry = {
								if !structs_and_functions.contains_key(&struct_name) {
									structs_and_functions.insert(struct_name.clone(), Vec::default());
									
								}
								structs_and_functions.get_mut(&struct_name)								
							};

							if let Some(struct_entry_unwraped) = struct_entry{
								let func_name = func.sig.ident.to_string();
								let mut arg_count = func.sig.inputs.len();

								let has_self = func
									.sig
									.inputs
									.iter()
									.find(|&input| {
										if let FnArg::Receiver(_) = input {
											return  true;
										}
										return  false;
									})
									.is_some();

								if has_self {
									arg_count -= 1;
								}
								
								if attribute == "signal"{
									struct_entry_unwraped.push(FunctionType::Signal(func_name, arg_count));
								}
								else if attribute == "func" {
									struct_entry_unwraped.push(FunctionType::Func(func_name, arg_count));
								}
							}
						}
					}				
				}
			}
		}
	}

    Ok(())
}



pub fn parse_rust_from_multiple_file(paths: &[String]) -> Result<StructAndFunctions, Box<dyn Error>> {
	let mut structs_and_functions = StructAndFunctions::new();
	
	for path in paths{
		parse_rust_file(&mut structs_and_functions, path.into())?;
		//structs_and_functions
	}
	 

	Ok(structs_and_functions)
}