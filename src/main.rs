mod compiler;
mod runtime;

use std::{fs, io};

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use runtime::{data::Data, vm::VM};

const COMMAND_EXECUTE: &str = "execute";
const COMMAND_DECOMPILE: &str = "decompile";
const COMMAND_COMPILE: &str = "compile";
const ARG_BINARY: &str = "binary";
const ARG_SOURCE: &str = "source";
const PARAM_OUT: &str = "out";

fn main() {
	let binary_arg = Arg::with_name(ARG_BINARY)
		.required(true)
		.help("A path to the binary you wish to operate on.");
	let matches = App::new("Synacor Challenge Runtime")
		.subcommand(SubCommand::with_name(COMMAND_EXECUTE).arg(binary_arg.clone()))
		.subcommand(
			SubCommand::with_name(COMMAND_DECOMPILE)
				.about("Writes the binary in human readable text.")
				.arg(binary_arg.clone())
				.arg(
					Arg::with_name(PARAM_OUT)
						.long("out")
						.short("o")
						.takes_value(true)
						.help(
							"A path where to write the output, any existing file will be \
							 overwritten. If not specified, the output will be written to the \
							 terminal.",
						),
				),
		)
		.subcommand(
			SubCommand::with_name(COMMAND_COMPILE)
				.about("Compiles some human readable text to an executable binary.")
				.arg(
					Arg::with_name(ARG_SOURCE)
						.required(true)
						.help("A path to the file you wish to compile."),
				)
				.arg(Arg::with_name(PARAM_OUT).required(true).help(
					"A path where to write the output, any existing file will be overwritten.",
				)),
		)
		.setting(AppSettings::SubcommandRequired)
		.get_matches();

	let result = match matches.subcommand() {
		(COMMAND_EXECUTE, Some(m)) => execute(m),
		(COMMAND_DECOMPILE, Some(m)) => decompile(m),
		(COMMAND_COMPILE, Some(m)) => compile(m),
		_ => Err("No subcommand provided!".to_string()),
	};

	if let Err(e) = result {
		eprintln!("{}", e);
	}
}

fn load_binary(args: &ArgMatches) -> Result<Vec<u16>, String> {
	fs::read(args.value_of(ARG_BINARY).unwrap())
		.map(|f| {
			f.chunks_exact(2)
				.map(|c| u16::from_le_bytes([c[0], c[1]]))
				.collect()
		})
		.map_err(|e| format!("Error when loading binary file. {}", e))
}

fn execute(args: &ArgMatches) -> Result<(), String> {
	let memory = load_binary(args)?;
	let data = Data::new(&memory);
	let mut vm = VM::new(data);

	vm.run(&mut io::stdin(), &mut io::stdout())
}

fn decompile(args: &ArgMatches) -> Result<(), String> {
	let memory = load_binary(args)?;
	match args.value_of(PARAM_OUT) {
		Some(out_path) => match fs::File::create(out_path) {
			Ok(mut o) => compiler::decompile(&memory, &mut o),
			Err(e) => Err(format!("Error when opening out file. {}", e)),
		},
		None => compiler::decompile(&memory, &mut io::stdout()),
	}
}

fn compile(args: &ArgMatches) -> Result<(), String> {
	let source = fs::File::open(args.value_of(ARG_SOURCE).unwrap())
		.map_err(|e| format!("Error when opening source file. {}", e))?;
	let parsing = compiler::parse(source)?;
	let mut file = fs::File::create(args.value_of(PARAM_OUT).unwrap())
		.map_err(|e| format!("Error when opening out file. {}", e))?;
	compiler::compile(&parsing, &mut file)
}
