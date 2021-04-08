use super::data::Data;
use std::io::{Read, Write};
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};

type Handler<I, O> = fn(&mut Data, usize, &mut I, &mut O) -> Result<Action, String>;

enum Action {
	Move(u16),
	Jump(u16),
	Halt(),
}

#[derive(Clone)]
pub struct VM<'a> {
	pub data: Data<'a>,
	pub pointer: usize,
}

impl<'a> VM<'a> {
	pub fn new(data: Data<'a>) -> Self {
		Self { data, pointer: 0 }
	}

	pub fn step<I: Read, O: Write>(
		&mut self,
		input: &mut I,
		output: &mut O,
	) -> Result<bool, String> {
		if self.pointer >= self.data.length_memory() {
			return Err(format!("Out of range {}!", self.pointer));
		}

		let handler = get_handler(self.data.get_number(self.pointer).unwrap());
		match handler(&mut self.data, self.pointer, input, output) {
			Ok(Action::Move(m)) => self.pointer += m as usize,
			Ok(Action::Jump(j)) => self.pointer = j as usize,
			Ok(Action::Halt()) => return Ok(false),
			Err(err) => {
				return Err(format!("Error at {}:\n\t{}", self.pointer, err));
			}
		};

		Ok(true)
	}

	pub fn run<I: Read, O: Write>(&mut self, input: &mut I, output: &mut O) -> Result<(), String> {
		let running = Arc::new(AtomicBool::new(true));
		let r = running.clone();

		ctrlc::set_handler(move || r.store(false, Ordering::SeqCst))
			.or_else(|_| Err("Could not set Ctrl-C handler!".to_string()))?;

		while self.step(input, output)? && running.load(Ordering::SeqCst) {}

		Ok(())
	}
}

fn get_handler<I: Read, O: Write>(opcode: u16) -> Handler<I, O> {
	match opcode {
		0 => halt,
		1 => set,
		2 => push,
		3 => pop,
		4 => eq,
		5 => gt,
		6 => jmp,
		7 => jt,
		8 => jf,
		9 => add,
		10 => mul,
		11 => mod_op,
		12 => and,
		13 => or,
		14 => not,
		15 => rmem,
		16 => wmem,
		17 => call,
		18 => ret,
		19 => out,
		20 => in_op,
		21 => noop,
		_ => unknown,
	}
}

fn halt<I: Read, O: Write>(_: &mut Data, _: usize, _: &mut I, _: &mut O) -> Result<Action, String> {
	Ok(Action::Halt())
}

fn set<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	data.set_number(i + 1, data.get_number(i + 2)?)?;
	Ok(Action::Move(3))
}

fn push<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	data.push_stack(data.get_number(i + 1)?);
	Ok(Action::Move(2))
}

fn pop<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let value = data.pop_stack()?;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(2))
}

fn eq<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let value = (data.get_number(i + 2)? == data.get_number(i + 3)?) as u16;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(4))
}

fn gt<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let value = (data.get_number(i + 2)? > data.get_number(i + 3)?) as u16;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(4))
}

fn jmp<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	Ok(Action::Jump(data.get_number(i + 1)?))
}

fn jt<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	if data.get_number(i + 1)? != 0 {
		Ok(Action::Jump(data.get_number(i + 2)?))
	} else {
		Ok(Action::Move(3))
	}
}

fn jf<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	if data.get_number(i + 1)? == 0 {
		Ok(Action::Jump(data.get_number(i + 2)?))
	} else {
		Ok(Action::Move(3))
	}
}

fn add<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let value = (data.get_number(i + 2)? + data.get_number(i + 3)?) % 32768;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(4))
}

fn mul<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let value =
		(((data.get_number(i + 2)? as u64) * (data.get_number(i + 3)? as u64)) % 32768) as u16;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(4))
}

fn mod_op<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let value = data.get_number(i + 2)? % data.get_number(i + 3)?;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(4))
}

fn and<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let value = data.get_number(i + 2)? & data.get_number(i + 3)?;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(4))
}

fn or<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let value = data.get_number(i + 2)? | data.get_number(i + 3)?;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(4))
}

fn not<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let value = 0x7FFF ^ data.get_number(i + 2)?;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(3))
}

fn rmem<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let address = data.get_number(i + 2)?;
	let value = data.read_memory(address)?;
	data.set_number(i + 1, value)?;
	Ok(Action::Move(3))
}

fn wmem<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let address = data.get_number(i + 1)?;
	let value = data.get_number(i + 2)?;
	data.write_memory(address, value)?;
	Ok(Action::Move(3))
}

fn call<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	let next_addr = (i + 2) as u16;
	data.push_stack(next_addr);
	Ok(Action::Jump(data.get_number(i + 1)?))
}

fn ret<I: Read, O: Write>(
	data: &mut Data,
	_: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	if let Ok(ret_addr) = data.pop_stack() {
		Ok(Action::Jump(ret_addr))
	} else {
		Ok(Action::Halt())
	}
}

fn out<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	output: &mut O,
) -> Result<Action, String> {
	let ascii = data.get_number(i + 1)?;
	match String::from_utf16(&[ascii]) {
		Ok(str) => {
			write!(output, "{}", str)
				.or_else(|_| Err(format!("Could not write {} to output!", str)))?;
			Ok(Action::Move(2))
		}
		Err(_) => Err(format!("Could not encode {} as a character!", ascii)),
	}
}

fn in_op<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	input: &mut I,
	output: &mut O,
) -> Result<Action, String> {
	let mut buf = [0];
	match input.read(&mut buf) {
		Ok(1) => {
			if buf[0] == 13 {
				in_op(data, i, input, output)
			} else {
				data.set_number(i + 1, buf[0] as u16)?;
				Ok(Action::Move(2))
			}
		}
		Ok(0) => {
			return Ok(Action::Halt());
		}
		_ => Err("Could not read from input!".to_string()),
	}
}

fn noop<I: Read, O: Write>(_: &mut Data, _: usize, _: &mut I, _: &mut O) -> Result<Action, String> {
	Ok(Action::Move(1))
}

fn unknown<I: Read, O: Write>(
	data: &mut Data,
	i: usize,
	_: &mut I,
	_: &mut O,
) -> Result<Action, String> {
	Err(format!("Unknown opcode {}!", data.get_number(i)?))
}
