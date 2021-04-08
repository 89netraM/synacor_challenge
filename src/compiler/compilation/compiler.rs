use super::parser::{get_size, Instruction, Parsing, Token};
use std::collections::HashMap;
use std::io::{Error, Write};

pub fn compile<O: Write>(parsing: &Parsing, output: &mut O) -> Result<(), String> {
	let mut pointer = 0;

	while let Some(parser_instruction) = parsing.instructions.get(&pointer) {
		compile_instruction(&parser_instruction.instruction, &parsing.labels, output).map_err(
			|e| {
				format!(
					"Error when compiling line {}.\n\t{}",
					parser_instruction.line_number, e
				)
			},
		)?;
		pointer += get_size(&parser_instruction.instruction);
	}

	output.flush().map_err(could_not_write)
}

fn compile_instruction<O: Write>(
	instruction: &Instruction,
	labels: &HashMap<String, u16>,
	output: &mut O,
) -> Result<(), String> {
	match instruction {
		Instruction::Halt() => halt(output),
		Instruction::Set(a1, a2) => set(a1, a2, output),
		Instruction::Push(a1) => push(a1, output),
		Instruction::Pop(a1) => pop(a1, output),
		Instruction::Eq(a1, a2, a3) => eq(a1, a2, a3, output),
		Instruction::Gt(a1, a2, a3) => gt(a1, a2, a3, output),
		Instruction::Jmp(a1) => jmp(a1, labels, output),
		Instruction::Jt(a1, a2) => jt(a1, a2, labels, output),
		Instruction::Jf(a1, a2) => jf(a1, a2, labels, output),
		Instruction::Add(a1, a2, a3) => add(a1, a2, a3, output),
		Instruction::Mul(a1, a2, a3) => mul(a1, a2, a3, output),
		Instruction::Mod(a1, a2, a3) => mod_op(a1, a2, a3, output),
		Instruction::And(a1, a2, a3) => and(a1, a2, a3, output),
		Instruction::Or(a1, a2, a3) => or(a1, a2, a3, output),
		Instruction::Not(a1, a2) => not(a1, a2, output),
		Instruction::RMem(a1, a2) => rmem(a1, a2, labels, output),
		Instruction::WMem(a1, a2) => wmem(a1, a2, labels, output),
		Instruction::Call(a1) => call(a1, labels, output),
		Instruction::Ret() => ret(output),
		Instruction::Out(a1) => out(a1, output),
		Instruction::In(a1) => in_op(a1, output),
		Instruction::Noop() => noop(output),
		Instruction::Data(a1) => data(a1, output),
	}
}

fn halt<O: Write>(output: &mut O) -> Result<(), String> {
	output.write_all(&[0, 0]).map_err(could_not_write)
}

fn set<O: Write>(register: &Token, value: &Token, output: &mut O) -> Result<(), String> {
	if let Token::Value(r) = register {
		if let Token::Value(v) = value {
			let r_bytes = r.to_le_bytes();
			let v_bytes = v.to_le_bytes();
			output
				.write_all(&[1, 0, r_bytes[0], r_bytes[1], v_bytes[0], v_bytes[1]])
				.map_err(could_not_write)
		} else {
			Err("The second argument of a set instruction must be a literal.".to_string())
		}
	} else {
		Err("The first argument of a set instruction must be a literal.".to_string())
	}
}

fn push<O: Write>(value: &Token, output: &mut O) -> Result<(), String> {
	if let Token::Value(v) = value {
		let v_bytes = v.to_le_bytes();
		output
			.write_all(&[2, 0, v_bytes[0], v_bytes[1]])
			.map_err(could_not_write)
	} else {
		Err("The argument of a push instruction must be a literal.".to_string())
	}
}

fn pop<O: Write>(register: &Token, output: &mut O) -> Result<(), String> {
	if let Token::Value(r) = register {
		let r_bytes = r.to_le_bytes();
		output
			.write_all(&[3, 0, r_bytes[0], r_bytes[1]])
			.map_err(could_not_write)
	} else {
		Err("The argument of a pop instruction must be a literal.".to_string())
	}
}

fn eq<O: Write>(
	register: &Token,
	value_a: &Token,
	value_b: &Token,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(r) = register {
		if let Token::Value(a) = value_a {
			if let Token::Value(b) = value_b {
				let r_bytes = r.to_le_bytes();
				let a_bytes = a.to_le_bytes();
				let b_bytes = b.to_le_bytes();
				output
					.write_all(&[
						4, 0, r_bytes[0], r_bytes[1], a_bytes[0], a_bytes[1], b_bytes[0],
						b_bytes[1],
					])
					.map_err(could_not_write)
			} else {
				Err("The third argument of a eq instruction must be a literal.".to_string())
			}
		} else {
			Err("The second argument of a eq instruction must be a literal.".to_string())
		}
	} else {
		Err("The first argument of a eq instruction must be a literal.".to_string())
	}
}

fn gt<O: Write>(
	register: &Token,
	value_a: &Token,
	value_b: &Token,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(r) = register {
		if let Token::Value(a) = value_a {
			if let Token::Value(b) = value_b {
				let r_bytes = r.to_le_bytes();
				let a_bytes = a.to_le_bytes();
				let b_bytes = b.to_le_bytes();
				output
					.write_all(&[
						5, 0, r_bytes[0], r_bytes[1], a_bytes[0], a_bytes[1], b_bytes[0],
						b_bytes[1],
					])
					.map_err(could_not_write)
			} else {
				Err("The third argument of a gt instruction must be a literal.".to_string())
			}
		} else {
			Err("The second argument of a gt instruction must be a literal.".to_string())
		}
	} else {
		Err("The first argument of a gt instruction must be a literal.".to_string())
	}
}

fn jmp<O: Write>(
	target: &Token,
	labels: &HashMap<String, u16>,
	output: &mut O,
) -> Result<(), String> {
	let t = match target {
		Token::Value(t) => t,
		Token::Label(l) => labels.get(l).ok_or(format!("Undefined label \"{}\"!", l))?,
	};
	let t_bytes = t.to_le_bytes();
	output
		.write_all(&[6, 0, t_bytes[0], t_bytes[1]])
		.map_err(could_not_write)
}

fn jt<O: Write>(
	value: &Token,
	target: &Token,
	labels: &HashMap<String, u16>,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(v) = value {
		let v_bytes = v.to_le_bytes();
		let t = match target {
			Token::Value(t) => t,
			Token::Label(l) => labels.get(l).ok_or(format!("Undefined label \"{}\"!", l))?,
		};
		let t_bytes = t.to_le_bytes();
		output
			.write_all(&[7, 0, v_bytes[0], v_bytes[1], t_bytes[0], t_bytes[1]])
			.map_err(could_not_write)
	} else {
		Err("The first argument of a jt instruction must be a literal.".to_string())
	}
}

fn jf<O: Write>(
	value: &Token,
	target: &Token,
	labels: &HashMap<String, u16>,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(v) = value {
		let v_bytes = v.to_le_bytes();
		let t = match target {
			Token::Value(t) => t,
			Token::Label(l) => labels.get(l).ok_or(format!("Undefined label \"{}\"!", l))?,
		};
		let t_bytes = t.to_le_bytes();
		output
			.write_all(&[8, 0, v_bytes[0], v_bytes[1], t_bytes[0], t_bytes[1]])
			.map_err(could_not_write)
	} else {
		Err("The first argument of a jf instruction must be a literal.".to_string())
	}
}

fn add<O: Write>(
	register: &Token,
	value_a: &Token,
	value_b: &Token,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(r) = register {
		if let Token::Value(a) = value_a {
			if let Token::Value(b) = value_b {
				let r_bytes = r.to_le_bytes();
				let a_bytes = a.to_le_bytes();
				let b_bytes = b.to_le_bytes();
				output
					.write_all(&[
						9, 0, r_bytes[0], r_bytes[1], a_bytes[0], a_bytes[1], b_bytes[0],
						b_bytes[1],
					])
					.map_err(could_not_write)
			} else {
				Err("The third argument of a add instruction must be a literal.".to_string())
			}
		} else {
			Err("The second argument of a add instruction must be a literal.".to_string())
		}
	} else {
		Err("The first argument of a add instruction must be a literal.".to_string())
	}
}

fn mul<O: Write>(
	register: &Token,
	value_a: &Token,
	value_b: &Token,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(r) = register {
		if let Token::Value(a) = value_a {
			if let Token::Value(b) = value_b {
				let r_bytes = r.to_le_bytes();
				let a_bytes = a.to_le_bytes();
				let b_bytes = b.to_le_bytes();
				output
					.write_all(&[
						10, 0, r_bytes[0], r_bytes[1], a_bytes[0], a_bytes[1], b_bytes[0],
						b_bytes[1],
					])
					.map_err(could_not_write)
			} else {
				Err("The third argument of a mult instruction must be a literal.".to_string())
			}
		} else {
			Err("The second argument of a mult instruction must be a literal.".to_string())
		}
	} else {
		Err("The first argument of a mult instruction must be a literal.".to_string())
	}
}

fn mod_op<O: Write>(
	register: &Token,
	value_a: &Token,
	value_b: &Token,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(r) = register {
		if let Token::Value(a) = value_a {
			if let Token::Value(b) = value_b {
				let r_bytes = r.to_le_bytes();
				let a_bytes = a.to_le_bytes();
				let b_bytes = b.to_le_bytes();
				output
					.write_all(&[
						11, 0, r_bytes[0], r_bytes[1], a_bytes[0], a_bytes[1], b_bytes[0],
						b_bytes[1],
					])
					.map_err(could_not_write)
			} else {
				Err("The third argument of a mod instruction must be a literal.".to_string())
			}
		} else {
			Err("The second argument of a mod instruction must be a literal.".to_string())
		}
	} else {
		Err("The first argument of a mod instruction must be a literal.".to_string())
	}
}

fn and<O: Write>(
	register: &Token,
	value_a: &Token,
	value_b: &Token,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(r) = register {
		if let Token::Value(a) = value_a {
			if let Token::Value(b) = value_b {
				let r_bytes = r.to_le_bytes();
				let a_bytes = a.to_le_bytes();
				let b_bytes = b.to_le_bytes();
				output
					.write_all(&[
						12, 0, r_bytes[0], r_bytes[1], a_bytes[0], a_bytes[1], b_bytes[0],
						b_bytes[1],
					])
					.map_err(could_not_write)
			} else {
				Err("The third argument of a and instruction must be a literal.".to_string())
			}
		} else {
			Err("The second argument of a and instruction must be a literal.".to_string())
		}
	} else {
		Err("The first argument of a and instruction must be a literal.".to_string())
	}
}

fn or<O: Write>(
	register: &Token,
	value_a: &Token,
	value_b: &Token,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(r) = register {
		if let Token::Value(a) = value_a {
			if let Token::Value(b) = value_b {
				let r_bytes = r.to_le_bytes();
				let a_bytes = a.to_le_bytes();
				let b_bytes = b.to_le_bytes();
				output
					.write_all(&[
						13, 0, r_bytes[0], r_bytes[1], a_bytes[0], a_bytes[1], b_bytes[0],
						b_bytes[1],
					])
					.map_err(could_not_write)
			} else {
				Err("The third argument of a or instruction must be a literal.".to_string())
			}
		} else {
			Err("The second argument of a or instruction must be a literal.".to_string())
		}
	} else {
		Err("The first argument of a or instruction must be a literal.".to_string())
	}
}

fn not<O: Write>(register: &Token, value: &Token, output: &mut O) -> Result<(), String> {
	if let Token::Value(r) = register {
		if let Token::Value(v) = value {
			let r_bytes = r.to_le_bytes();
			let v_bytes = v.to_le_bytes();
			output
				.write_all(&[14, 0, r_bytes[0], r_bytes[1], v_bytes[0], v_bytes[1]])
				.map_err(could_not_write)
		} else {
			Err("The second argument of a not instruction must be a literal.".to_string())
		}
	} else {
		Err("The first argument of a not instruction must be a literal.".to_string())
	}
}

fn rmem<O: Write>(
	register: &Token,
	target: &Token,
	labels: &HashMap<String, u16>,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(r) = register {
		let r_bytes = r.to_le_bytes();
		let t = match target {
			Token::Value(t) => t,
			Token::Label(l) => labels.get(l).ok_or(format!("Undefined label \"{}\"!", l))?,
		};
		let t_bytes = t.to_le_bytes();
		output
			.write_all(&[15, 0, r_bytes[0], r_bytes[1], t_bytes[0], t_bytes[1]])
			.map_err(could_not_write)
	} else {
		Err("The first argument of a rmem instruction must be a literal.".to_string())
	}
}

fn wmem<O: Write>(
	target: &Token,
	value: &Token,
	labels: &HashMap<String, u16>,
	output: &mut O,
) -> Result<(), String> {
	if let Token::Value(v) = value {
		let v_bytes = v.to_le_bytes();
		let t = match target {
			Token::Value(t) => t,
			Token::Label(l) => labels.get(l).ok_or(format!("Undefined label \"{}\"!", l))?,
		};
		let t_bytes = t.to_le_bytes();
		output
			.write_all(&[16, 0, t_bytes[0], t_bytes[1], v_bytes[0], v_bytes[1]])
			.map_err(could_not_write)
	} else {
		Err("The second argument of a wmem instruction must be a literal.".to_string())
	}
}

fn call<O: Write>(
	target: &Token,
	labels: &HashMap<String, u16>,
	output: &mut O,
) -> Result<(), String> {
	let t = match target {
		Token::Value(t) => t,
		Token::Label(l) => labels.get(l).ok_or(format!("Undefined label \"{}\"!", l))?,
	};
	let t_bytes = t.to_le_bytes();
	output
		.write_all(&[17, 0, t_bytes[0], t_bytes[1]])
		.map_err(could_not_write)
}

fn ret<O: Write>(output: &mut O) -> Result<(), String> {
	output.write_all(&[18, 0]).map_err(could_not_write)
}

fn out<O: Write>(value: &Token, output: &mut O) -> Result<(), String> {
	if let Token::Value(v) = value {
		let v_bytes = v.to_le_bytes();
		output
			.write_all(&[19, 0, v_bytes[0], v_bytes[1]])
			.map_err(could_not_write)
	} else {
		Err("The argument of a out instruction must be a literal.".to_string())
	}
}

fn in_op<O: Write>(register: &Token, output: &mut O) -> Result<(), String> {
	if let Token::Value(r) = register {
		let r_bytes = r.to_le_bytes();
		output
			.write_all(&[20, 0, r_bytes[0], r_bytes[1]])
			.map_err(could_not_write)
	} else {
		Err("The argument of a in instruction must be a literal.".to_string())
	}
}

fn noop<O: Write>(output: &mut O) -> Result<(), String> {
	output.write_all(&[21, 0]).map_err(could_not_write)
}

fn data<O: Write>(value: &Token, output: &mut O) -> Result<(), String> {
	if let Token::Value(t) = value {
		let t_bytes = t.to_le_bytes();
		output.write_all(&t_bytes).map_err(could_not_write)
	} else {
		Err("Data must be a literal.".to_string())
	}
}

fn could_not_write(e: Error) -> String {
	format!("Could not write to the output binary. {}", e)
}
