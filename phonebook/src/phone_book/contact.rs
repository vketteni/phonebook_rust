pub struct Contact {
    first_name: String,
    last_name: String,
    nickname: String,
    phone_number: String,
    email: String,
    address: String,
}

impl Contact {
    pub fn new(
		first_name: &str, 
		last_name: &str, 
		nickname: &str, 
		phone_number: &str, 
		email: &str, 
		address: &str,
	) -> Self {
		Self {
			first_name:     first_name.to_string(),
			last_name:     last_name.to_string(),
			nickname:     nickname.to_string(),
			phone_number:     phone_number.to_string(),
			email:     email.to_string(),
			address:     address.to_string()
		}
    }

	pub fn first_name(&self) -> &str {
		&self.first_name
	}

	pub fn last_name(&self) -> &str {
		&self.last_name
	}

	pub fn nickname(&self) -> &str {
		&self.nickname
	}

	pub fn phone_number(&self) -> &str {
		&self.phone_number
	}
	pub fn email(&self) -> &str {
		&self.email
	}

	pub fn address(&self) -> &str {
		&self.address
	}

	pub fn set_first_name(&mut self, first_name: String) {
		self.first_name = first_name;
	}

	pub fn set_last_name(&mut self, last_name: String) {
		self.last_name = last_name;
	}
	
	pub fn set_nickname(&mut self, nickname: String) {
		self.nickname = nickname;
	}

	pub fn set_phone_number(&mut self, phone_number: String) {
		self.phone_number = phone_number;
	}

	pub fn set_address(&mut self, address: String) {
		self.address = address;
	}

	pub fn set_email(&mut self, email: String) {
		self.email = email;
	}
}
