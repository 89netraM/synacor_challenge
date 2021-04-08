use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Token {
	Label(String),
	Value(u16),
}

#[derive(Debug)]
pub(super) enum Instruction {
	Halt(),
	Set(Token, Token),
	Push(Token),
	Pop(Token),
	Eq(Token, Token, Token),
	Gt(Token, Token, Token),
	Jmp(Token),
	Jt(Token, Token),
	Jf(Token, Token),
	Add(Token, Token, Token),
	Mul(Token, Token, Token),
	Mod(Token, Token, Token),
	And(Token, Token, Token),
	Or(Token, Token, Token),
	Not(Token, Token),
	RMem(Token, Token),
	WMem(Token, Token),
	Call(Token),
	Ret(),
	Out(Token),
	In(Token),
	Noop(),
}

#[derive(Debug)]
pub(super) struct ParsedInstruction {
	pub line_number: usize,
	pub instruction: Instruction,
}

#[derive(Debug)]
pub struct Parsing {
	pub(super) instructions: HashMap<u16, ParsedInstruction>,
	pub(super) labels: HashMap<String, u16>,
}

type Constructor = fn([Option<Token>; 3]) -> Result<Instruction, String>;

pub fn parse<I: Read>(input: I) -> Result<Parsing, String> {
	let mut reader = BufReader::new(input);

	let mut instructions = HashMap::new();
	let mut labels: HashMap<String, u16> = HashMap::new();
	let mut line = String::new();
	let mut line_number = 1;
	let mut pointer = 0;

	let mut label: Option<String>;
	let mut constructor: Option<Constructor>;
	let mut arguments: [Option<Token>; 3];
	let mut argument_count: usize;
	while reader
		.read_line(&mut line)
		.map_err(|_| format!("Error reading line {}!", line_number))?
		> 0
	{
		label = None;
		constructor = None;
		arguments = [None, None, None];
		argument_count = 0;
		for part in line.split_whitespace() {
			if part.starts_with('#') {
				break;
			} else if part.ends_with(':') {
				if label.is_none() {
					let name = &part[0..part.len() - 1];
					if let Ok(pointer_label) = name.parse::<u16>() {
						if pointer_label != pointer {
							return Err(format!(
								"Pointer label was {} but should have been {} on line {}.",
								pointer_label, pointer, line_number
							));
						}
					} else {
						label = Some(String::from(name));
					}
				} else {
					return Err(format!(
						"Only one label per line! Detected a \":\" in an unusual place on line {}.",
						line_number
					));
				}
			} else if constructor.is_none() {
				constructor = match get_constructor(part) {
					None => {
						return Err(format!("Unknown op \"{}\" at line {}.", part, line_number))
					}
					c => c,
				};
			} else {
				let arg = if let Ok(value) = part.parse::<u16>() {
					Token::Value(value)
				} else {
					Token::Label(String::from(part))
				};
				arguments[argument_count] = Some(arg);
				argument_count += 1;
			}
		}

		if let Some(label_name) = label {
			labels.insert(label_name, pointer);
		}

		if let Some(con) = constructor {
			let instruction = con(arguments)?;
			let size = get_size(&instruction);
			instructions.insert(
				pointer,
				ParsedInstruction {
					line_number,
					instruction,
				},
			);
			pointer += size;
		}

		line_number += 1;
		line.clear();
	}

	Ok(Parsing {
		instructions,
		labels,
	})
}

pub(super) fn get_size(instruction: &Instruction) -> u16 {
	match instruction {
		Instruction::Halt() => 1,
		Instruction::Set(_, _) => 3,
		Instruction::Push(_) => 2,
		Instruction::Pop(_) => 2,
		Instruction::Eq(_, _, _) => 4,
		Instruction::Gt(_, _, _) => 4,
		Instruction::Jmp(_) => 2,
		Instruction::Jt(_, _) => 3,
		Instruction::Jf(_, _) => 3,
		Instruction::Add(_, _, _) => 4,
		Instruction::Mul(_, _, _) => 4,
		Instruction::Mod(_, _, _) => 4,
		Instruction::And(_, _, _) => 4,
		Instruction::Or(_, _, _) => 4,
		Instruction::Not(_, _) => 3,
		Instruction::RMem(_, _) => 3,
		Instruction::WMem(_, _) => 3,
		Instruction::Call(_) => 2,
		Instruction::Ret() => 1,
		Instruction::Out(_) => 2,
		Instruction::In(_) => 2,
		Instruction::Noop() => 1,
	}
}

fn get_constructor(op: &str) -> Option<Constructor> {
	match op {
		"halt" => Some(halt),
		"set" => Some(set),
		"push" => Some(push),
		"pop" => Some(pop),
		"eq" => Some(eq),
		"gt" => Some(gt),
		"jmp" => Some(jmp),
		"jt" => Some(jt),
		"jf" => Some(jf),
		"add" => Some(add),
		"mult" => Some(mul),
		"mod" => Some(mod_op),
		"and" => Some(and),
		"or" => Some(or),
		"not" => Some(not),
		"rmem" => Some(rmem),
		"wmem" => Some(wmem),
		"call" => Some(call),
		"ret" => Some(ret),
		"out" => Some(out),
		"in" => Some(in_op),
		"noop" => Some(noop),
		_ => None,
	}
}

fn halt(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if [None, None, None] == args {
		Ok(Instruction::Halt())
	} else {
		Err("halt takes no arguments".to_string())
	}
}

fn set(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), None] = args {
		Ok(Instruction::Set(a1, a2))
	} else {
		Err("set takes two arguments".to_string())
	}
}

fn push(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), None, None] = args {
		Ok(Instruction::Push(a1))
	} else {
		Err("push takes one arguments".to_string())
	}
}

fn pop(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), None, None] = args {
		Ok(Instruction::Pop(a1))
	} else {
		Err("pop takes one argument".to_string())
	}
}

fn eq(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), Some(a3)] = args {
		Ok(Instruction::Eq(a1, a2, a3))
	} else {
		Err("eq takes three arguments".to_string())
	}
}

fn gt(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), Some(a3)] = args {
		Ok(Instruction::Gt(a1, a2, a3))
	} else {
		Err("gt takes three arguments".to_string())
	}
}

fn jmp(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), None, None] = args {
		Ok(Instruction::Jmp(a1))
	} else {
		Err("jmp takes one argument".to_string())
	}
}

fn jt(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), None] = args {
		Ok(Instruction::Jt(a1, a2))
	} else {
		Err("jt takes two arguments".to_string())
	}
}

fn jf(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), None] = args {
		Ok(Instruction::Jf(a1, a2))
	} else {
		Err("jf takes two arguments".to_string())
	}
}

fn add(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), Some(a3)] = args {
		Ok(Instruction::Add(a1, a2, a3))
	} else {
		Err("add takes three arguments".to_string())
	}
}

fn mul(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), Some(a3)] = args {
		Ok(Instruction::Mul(a1, a2, a3))
	} else {
		Err("mul takes three arguments".to_string())
	}
}

fn mod_op(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), Some(a3)] = args {
		Ok(Instruction::Mod(a1, a2, a3))
	} else {
		Err("mod takes three arguments".to_string())
	}
}

fn and(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), Some(a3)] = args {
		Ok(Instruction::And(a1, a2, a3))
	} else {
		Err("and takes three arguments".to_string())
	}
}

fn or(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), Some(a3)] = args {
		Ok(Instruction::Or(a1, a2, a3))
	} else {
		Err("or takes three arguments".to_string())
	}
}

fn not(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), None] = args {
		Ok(Instruction::Not(a1, a2))
	} else {
		Err("not takes two arguments".to_string())
	}
}

fn rmem(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), None] = args {
		Ok(Instruction::RMem(a1, a2))
	} else {
		Err("rmem takes two arguments".to_string())
	}
}

fn wmem(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), Some(a2), None] = args {
		Ok(Instruction::WMem(a1, a2))
	} else {
		Err("wmem takes two arguments".to_string())
	}
}

fn call(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), None, None] = args {
		Ok(Instruction::Call(a1))
	} else {
		Err("call takes one argument".to_string())
	}
}

fn ret(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if [None, None, None] == args {
		Ok(Instruction::Ret())
	} else {
		Err("ret takes no arguments".to_string())
	}
}

fn out(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), None, None] = args {
		Ok(Instruction::Out(a1))
	} else {
		Err("out takes one argument".to_string())
	}
}

fn in_op(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if let [Some(a1), None, None] = args {
		Ok(Instruction::In(a1))
	} else {
		Err("in takes one argument".to_string())
	}
}

fn noop(args: [Option<Token>; 3]) -> Result<Instruction, String> {
	if [None, None, None] == args {
		Ok(Instruction::Noop())
	} else {
		Err("noop takes no arguments".to_string())
	}
}
