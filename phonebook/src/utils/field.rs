pub struct Field {
    value: Option<String>,
    length: usize,
    max_length: usize,
}

impl Field {
    pub fn new(value: Option<String>, max_length: usize) -> Field {
        let length = value.as_ref().map_or(0, |v| v.len());
        Field {
            value,
            length,
            max_length,
        }
    }

	pub fn set_value(&mut self, value: String) -> Result<(), String> {
		if value.len() > self.max_length {
			Err(format!("Value exceeds maximum length of {}", self.max_length))
		} else {
			self.length = value.len();
			self.value = Some(value);
			Ok(())
		}
	}

	pub fn get_value(&self) -> &str {
		self.value.as_deref().unwrap_or("")
	}
}
