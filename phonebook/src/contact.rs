use super::utils::field::Field;

pub struct Contact {
    first_name: Field,
    last_name: Field,
    nickname: Field,
    phone_number: Field,
    email: Field,
    address: Field,
}

impl Contact {
    pub fn new() -> Contact {
		Contact {
			first_name:		Field::new(None, 50),
			last_name:		Field::new(None, 50),
			nickname:		Field::new(None, 30),
			phone_number:	Field::new(None, 15),
			email:			Field::new(None, 100),
			address:		Field::new(None, 100)
		}
    }

	pub fn first_name(&self) -> &str {
		self.first_name.get_value()
	}

	pub fn last_name(&self) -> &str {
		&self.last_name.get_value()
	}

	pub fn nickname(&self) -> &str {
		&self.nickname.get_value()
	}

	pub fn phone_number(&self) -> &str {
		&self.phone_number.get_value()
	}
	pub fn email(&self) -> &str {
		&self.email.get_value()
	}

	pub fn address(&self) -> &str {
		&self.address.get_value()
	}

	pub fn set_first_name(&mut self, first_name: String) {
		self.first_name.set_value(first_name);
	}

	pub fn set_last_name(&mut self, last_name: String) {
		self.last_name.set_value(last_name);
	}
	
	pub fn set_nickname(&mut self, nickname: String) {
		self.nickname.set_value(nickname);
	}

	pub fn set_phone_number(&mut self, phone_number: String) {
		self.phone_number.set_value(phone_number);
	}

	pub fn set_address(&mut self, address: String) {
		self.address.set_value(address);
	}

	pub fn set_email(&mut self, email: String) {
		self.email.set_value(email);
	}
}
