mod compiler;
mod runtime;

use clap::{App, Arg, ArgMatches, SubCommand};
use runtime::{data::Data, vm::VM};
use std::fs;
use std::io;

const ARG_BINARY: &str = "binary";
const COMMAND_DECOMPILE: &str = "decompile";
const PARAM_OUT: &str = "out";

fn main() {
	let matches = App::new("Synacor Challenge Runtime")
		.arg(
			Arg::with_name(ARG_BINARY)
				.required(true)
				.help("A path to the binary you wish to operate on."),
		)
		.subcommand(
			SubCommand::with_name(COMMAND_DECOMPILE)
				.about("Writes the binary in human readable text.")
				.arg(
					Arg::with_name(PARAM_OUT)
						.long("out")
						.short("o")
						.takes_value(true)
						.help(
							"A path where to write the output, any existing \
							file will be overwritten. If not specified, the \
							output will be written to the terminal.",
						),
				),
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

	let result = match matches.subcommand() {
		(COMMAND_DECOMPILE, Some(m)) => decompile(&memory, m),
		_ => execute(&memory),
	};

	if let Err(e) = result {
		eprintln!("{}", e);
	}
}

fn execute(memory: &[u16]) -> Result<(), String> {
	let data = Data::new(&memory);
	let mut vm = VM::new(data);

	vm.run(&mut io::stdin(), &mut io::stdout())
}

fn decompile(memory: &[u16], args: &ArgMatches) -> Result<(), String> {
	match args.value_of(PARAM_OUT) {
		Some(out_path) => match fs::File::create(out_path) {
			Ok(mut o) => compiler::decompile(memory, &mut o),
			Err(e) => Err(format!("Error when opening out file. {}", e)),
		},
		None => compiler::decompile(memory, &mut io::stdout()),
	}
}
