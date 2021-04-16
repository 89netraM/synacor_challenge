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

#[cfg(test)]
mod tests {
	use super::*;

	const MEMORY: &'static [u16] = &[21, 19, 77, 0, 32768];

	#[test]
	fn get_number() {
		let data = Data::new(MEMORY);
		assert_eq!(
			data.get_number(2),
			Ok(MEMORY[2]),
			"Reading a valid number at a valid position."
		);
	}

	#[test]
	fn get_register() {
		let data = Data::new(MEMORY);
		assert_eq!(data.get_number(4), Ok(0), "Reading a register address.");
	}

	#[test]
	fn get_invalid() {
		let data = Data::new(MEMORY);
		assert_eq!(
			data.get_number(5),
			Err("Reading from out of range address 5!".to_string()),
			"Reading a register address."
		);
	}

	#[test]
	fn update_register() {
		const TEST_VALUE: u16 = 42;
		let mut data = Data::new(MEMORY);
		data.set_number(4, TEST_VALUE).unwrap();
		assert_eq!(
			data.get_number(4),
			Ok(TEST_VALUE),
			"Updating the value of a register."
		);
	}

	#[test]
	fn update_non_register() {
		let mut data = Data::new(MEMORY);
		let result = data.set_number(2, 42);
		assert_eq!(
			result,
			Err(format!(
				"Number at {} ({}) is not a register!",
				2, MEMORY[2]
			)),
			"Updating the value of a address that's not a register."
		);
	}

	#[test]
	fn update_invalid() {
		let mut data = Data::new(MEMORY);
		let result = data.set_number(5, 42);
		assert_eq!(
			result,
			Err("Reading from out of range address 5!".to_string()),
			"Updating to an address out of range."
		);
	}

	#[test]
	fn push_and_pop() {
		const TEST_VALUE: u16 = 42;
		let mut data = Data::new(MEMORY);
		data.push_stack(TEST_VALUE);
		assert_eq!(
			data.pop_stack(),
			Ok(TEST_VALUE),
			"Push and then pop the same value."
		);
	}

	#[test]
	fn only_pop() {
		let mut data = Data::new(MEMORY);
		let result = data.pop_stack();
		assert_eq!(
			result,
			Err("Stack was empty when popping!".to_string()),
			"Poping when the stack is empty."
		);
	}

	#[test]
	fn reading_memory() {
		let data = Data::new(MEMORY);
		assert_eq!(
			data.read_memory(2),
			Ok(MEMORY[2]),
			"Reading from unchanged memory."
		);
	}

	#[test]
	fn reading_invalid_memory() {
		let data = Data::new(MEMORY);
		assert_eq!(
			data.read_memory(5),
			Err("Reading from out of range address 5!".to_string()),
			"Reading from out of range memory."
		);
	}

	#[test]
	fn writing_and_reading_memory() {
		const TEST_VALUE: u16 = 42;
		let mut data = Data::new(MEMORY);
		data.write_memory(2, TEST_VALUE).unwrap();
		assert_eq!(
			data.read_memory(2),
			Ok(TEST_VALUE),
			"Reading from changed memory."
		);
	}

	#[test]
	fn writing_invalid_memory() {
		let mut data = Data::new(MEMORY);
		let result = data.write_memory(5, 42);
		assert_eq!(
			result,
			Err("Writing to out of range address 5!".to_string()),
			"Writing to memory out of range."
		);
	}

	#[test]
	fn length_memory() {
		let data = Data::new(MEMORY);
		assert_eq!(data.length_memory(), MEMORY.len(), "Memory length.");
	}
}
