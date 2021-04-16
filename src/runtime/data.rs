use std::collections::HashMap;

#[derive(Clone)]
pub struct Data<'a> {
	memory: &'a [u16],
	memory_changes: HashMap<usize, u16>,
	registers: [u16; 8],
	stack: Vec<u16>,
}

impl<'a> Data<'a> {
	pub fn new(memory: &'a [u16]) -> Self {
		Self {
			memory,
			memory_changes: HashMap::new(),
			registers: [0; 8],
			stack: Vec::new(),
		}
	}

	pub fn get_number(&self, i: usize) -> Result<u16, String> {
		let value = self.read_memory(i as u16)?;
		if value > 32775 {
			Err(format!("Number at {} ({}) is too large!", i, value))
		} else if value > 32767 {
			Ok(self.registers[value as usize - 32768])
		} else {
			Ok(value)
		}
	}

	pub fn set_number(&mut self, r: usize, value: u16) -> Result<(), String> {
		let register = self.read_memory(r as u16)? as usize;
		if 32767 < register && register < 32776 {
			self.registers[register - 32768] = value;
			Ok(())
		} else {
			Err(format!("Number at {} ({}) is not a register!", r, register))
		}
	}

	pub fn push_stack(&mut self, value: u16) {
		self.stack.push(value);
	}

	pub fn pop_stack(&mut self) -> Result<u16, String> {
		self.stack
			.pop()
			.ok_or_else(|| "Stack was empty when popping!".to_string())
	}

	pub fn read_memory(&self, address: u16) -> Result<u16, String> {
		let addr = address as usize;
		if let Some(value) = self.memory_changes.get(&addr).cloned() {
			Ok(value)
		} else if let Some(value) = self.memory.get(address as usize).cloned() {
			Ok(value)
		} else {
			Err(format!("Reading from out of range address {}!", addr))
		}
	}

	pub fn write_memory(&mut self, address: u16, value: u16) -> Result<(), String> {
		let addr = address as usize;
		if addr < self.memory.len() {
			self.memory_changes.insert(addr, value);
			Ok(())
		} else {
			Err(format!("Writing to out of range address {}!", address))
		}
	}

	pub fn length_memory(&self) -> usize {
		self.memory.len()
	}
}
