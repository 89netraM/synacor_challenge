use std::io::{self, Write};

type Handler<O> = fn(&[u16], usize, &mut O) -> io::Result<usize>;

pub fn decompile<O: Write>(memory: &[u16], out: &mut O) -> Result<(), String> {
	let mut pointer = 0;
	while pointer < memory.len() {
		let handler = get_handler(memory[pointer]);
		pointer += handler(memory, pointer, out)
			.map_err(|e| format!("Could not write to output. {}", e))?;
	}
	Ok(())
}

fn get_handler<O: Write>(opcode: u16) -> Handler<O> {
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

fn halt<O: Write>(_: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\thalt", pointer)?;
	Ok(1)
}

fn set<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tset\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2]
	)?;
	Ok(3)
}

fn push<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\tpush\t{}", pointer, memory[pointer + 1])?;
	Ok(2)
}

fn pop<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\tpop\t{}", pointer, memory[pointer + 1])?;
	Ok(2)
}

fn eq<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\teq\t{}\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2],
		memory[pointer + 3]
	)?;
	Ok(4)
}

fn gt<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tgt\t{}\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2],
		memory[pointer + 3]
	)?;
	Ok(4)
}

fn jmp<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\tjmp\t{}", pointer, memory[pointer + 1])?;
	Ok(2)
}

fn jt<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tjt\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2]
	)?;
	Ok(3)
}

fn jf<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tjf\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2]
	)?;
	Ok(3)
}

fn add<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tadd\t{}\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2],
		memory[pointer + 3]
	)?;
	Ok(4)
}

fn mul<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tmul\t{}\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2],
		memory[pointer + 3]
	)?;
	Ok(4)
}

fn mod_op<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tmod\t{}\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2],
		memory[pointer + 3]
	)?;
	Ok(4)
}

fn and<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tand\t{}\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2],
		memory[pointer + 3]
	)?;
	Ok(4)
}

fn or<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tor\t{}\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2],
		memory[pointer + 3]
	)?;
	Ok(4)
}

fn not<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\tnot\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2]
	)?;
	Ok(3)
}

fn rmem<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\trmem\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2]
	)?;
	Ok(3)
}

fn wmem<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(
		out,
		"{}:\twmem\t{}\t{}",
		pointer,
		memory[pointer + 1],
		memory[pointer + 2]
	)?;
	Ok(3)
}

fn call<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\tcall\t{}", pointer, memory[pointer + 1])?;
	Ok(2)
}

fn ret<O: Write>(_: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\tret", pointer)?;
	Ok(1)
}

fn out<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\tout\t{}", pointer, memory[pointer + 1])?;
	Ok(2)
}

fn in_op<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\tin\t{}", pointer, memory[pointer + 1])?;
	Ok(2)
}

fn noop<O: Write>(_: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\tnoop", pointer)?;
	Ok(1)
}

fn unknown<O: Write>(memory: &[u16], pointer: usize, out: &mut O) -> io::Result<usize> {
	writeln!(out, "{}:\t{}", pointer, memory[pointer])?;
	Ok(1)
}
