mod runtime;

use clap::{App, Arg};
use runtime::{data::Data, vm::VM};
use std::fs;
use std::io;

const ARG_BINARY: &str = "BINARY";

fn main() {
	let matches = App::new("Synacor Challenge Runtime")
		.arg(
			Arg::with_name(ARG_BINARY)
				.required(true)
				.help("A path to the binary you wish to operate on."),
		)
		.get_matches();

	let file = match fs::read(matches.value_of(ARG_BINARY).unwrap()) {
		Ok(f) => f,
		Err(e) => {
			eprintln!("Error when loading binary file. {}", e);
			return;
		}
	};
	let memory: Vec<_> = file
		.chunks_exact(2)
		.map(|c| u16::from_le_bytes([c[0], c[1]]))
		.collect();

	let data = Data::new(&memory);
	let mut vm = VM::new(data);

	let result = vm.run(&mut io::stdin(), &mut io::stdout());
	if let Err(e) = result {
		eprintln!("{}", e);
	}
}
