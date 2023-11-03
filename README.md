# GD to signal .rs


## Motivation
Using Rust in Godot engine, means that most of the connections and signals are addressed via string names of the functions in question. This also means whenever functions is deleted or renamed, functions connected to it, or emitting of that functions will result in runtime error.
I have decided with my still novice experience in rust that I should make a tool that will make the wage functions names strings into something more syntactic, that upon deletion or name change will produce compile errors.

## Description
Simple tool that looks for [signal] and [func] godot attributed functions in files inside input directory. These would normally be accessed as string e.g "player_signal", but instead output directory will be filled with files containing constants in same-named structs and functions. Example usage in code: emit(signal::Player::player_signal.into())

## Building
Follow instructions on https://www.rust-lang.org/learn/get-started.

Then just run:
```bash
cargo build --release
```
Binary files should be located in target/release/gdtosignalrs.

## Usage
```bash
gdtosignalrs --input-directory <INPUT_DIRECTORY> --output-directory <OUTPUT_DIRECTORY>
```
Use --help for more information.

## Example (input/output)

### Example of input
This example is using function signatures as string names.
```rust

fn read(&mut self){
	let callable = self.base.callable("player_signal_ex");
	self.base.connect("player_signal".into(), callable);
}
//...
#[godot_api]
impl Player{
	#[func]
	fn helloworld(&mut self, string: GodotString){
		godot_print!("hello world!!!!!!");
		self.base.emit_signal("player_signal".into(), &[GodotString::from("test").to_variant()]);
	}

	#[func]
	fn player_signal_ex(text: GodotString){
		godot_print!("Signal handler executed", text);
	}

	#[signal]
	fn player_signal(text: GodotString){}
}
```
### Example of output

Both outputs will contain a version of:
```rust
pub struct GeneratedFunctionAlias{
	pub func: &'static str,
	pub arg_count: usize
}
```
This is done so it can be distinguished between func and signal types:
```rust
type GeneratedSignalAlias = signal::GeneratedFunctionAlias;
type GeneratedFuncAlias = func::GeneratedFunctionAlias;
```

#### signal.rs
```rust


pub struct Player;
impl Player{
	#[allow(non_upper_case_globals)]
	pub const player_signal: GeneratedFunctionAlias = GeneratedFunctionAlias{
		func: "player_signal",
		arg_count: 1
	};
}
```
#### func.rs
```rust
pub struct Player;
impl Player{
#[allow(non_upper_case_globals)]
pub const helloworld: GeneratedFunctionAlias = GeneratedFunctionAlias{
	func: "helloworld",
	arg_count: 1
};

#[allow(non_upper_case_globals)]
pub const player_signal_ex: GeneratedFunctionAlias = GeneratedFunctionAlias{
	func: "player_signal_ex",
	arg_count: 1
};

}
```

### Now to put everything together
We can replace string version in project and use statically typed ones instead:

```rust
use generated_dir::signal;
use generated_dir::func;
///...
fn read(&mut self){
	let callable = self.base.callable(func::Player::player_signal_ex);
	self.base.connect(signal::Player::player_signal.into(), callable);
}
//...
#[godot_api]
impl Player{
	#[func]
	fn helloworld(&mut self, string: GodotString){
		godot_print!("hello world!!!!!!");
		self.base.emit_signal(signal::Player::player_signal.into(), &[GodotString::from("test").to_variant()]);
	}

	#[func]
	fn player_signal_ex(text: GodotString){
		godot_print!("Signal handler executed", text);
	}

	#[signal]
	fn player_signal(text: GodotString){}
}
```

As long as gdtosignalrs is called each change/save in editor of you choice,
there should not be situation where renamed signal or func be undetected past build.

### Note
gdtosignalrs will also generate mod.rs module file containing only:
```rust
pub mod signal;
pub mod func;
```

## Contribution
I do not expect changes, therefore I do not accept contributions.

## License
MIT