use std::collections::HashMap;
use std::str::FromStr;

pub struct HeaderCollection(HashMap<String, String>);

impl HeaderCollection {
	pub fn new() -> Self {
		HeaderCollection(HashMap::new())
	}

	pub fn add(&mut self, line: &str) -> Result<(), String> {
		let mut a = line.trim().split(": ");
		let name = a
			.next()
			.filter(|n| !n.is_empty())
			.ok_or_else(|| format!("No header name found in:\n\t{}", line))?;
		if self.0.contains_key(name) {
			Err(format!(
				"Header collection already contains key \"{}\"!",
				name,
			))
		} else {
			self.0.insert(
				name.into(),
				a.next()
					.ok_or_else(|| format!("No header value found in:\n\t{}", line))?
					.into(),
			);
			Ok(())
		}
	}

	pub fn get<T: FromStr>(&self, name: &str) -> Option<Result<T, <T as FromStr>::Err>> {
		self.0.get(name).map(|v| v.parse::<T>())
	}

	pub fn clear(&mut self) {
		self.0.clear()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add_and_get() {
		let mut hc = HeaderCollection::new();
		hc.add("Content-Length: 119\r\n").unwrap();
		let value = hc.get("Content-Length");
		assert_eq!(
			value,
			Some(Ok(119)),
			"Should be able to add a header, and then get its value.",
		);
	}

	#[test]
	fn add_without_name() {
		let mut hc = HeaderCollection::new();
		let result = hc.add(": 119\r\n");
		assert_eq!(
			result,
			Err("No header name found in:\n\t: 119\r\n".to_string()),
			"Should not be able to add a header without a name.",
		);
	}

	#[test]
	fn add_without_value() {
		let mut hc = HeaderCollection::new();
		let result = hc.add("Content-Length: \r\n");
		assert_eq!(
			result,
			Err("No header value found in:\n\tContent-Length: \r\n".to_string()),
			"Should not be able to add a header without a value.",
		);
	}

	#[test]
	fn add_twice() {
		let mut hc = HeaderCollection::new();
		hc.add("Content-Length: 119\r\n").unwrap();
		let second_result = hc.add("Content-Length: 197\r\n");
		assert_eq!(
			second_result,
			Err("Header collection already contains key \"Content-Length\"!".to_string()),
			"Should error when trying to add the same header twice.",
		);
	}

	#[test]
	fn clear() {
		let mut hc = HeaderCollection::new();
		hc.add("Content-Length: 119\r\n").unwrap();
		hc.clear();
		assert_eq!(hc.get::<usize>("Content-Length"), None);
		hc.add("Content-Length: 119\r\n").unwrap();
	}
}
