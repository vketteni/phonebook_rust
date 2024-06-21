use crate::utils::validation::validate_length;
pub struct Contact {
	pub id:				Option<i32>,
    pub first_name:		Option<String>,
    pub last_name:		Option<String>,
    pub nickname:		Option<String>,
    pub phone_number:	Option<String>,
    pub email:			Option<String>,
    pub address:		Option<String>,
}

impl Contact {
    pub fn new() -> Contact {
		Contact {
			id:				None,
			first_name:		None,
			last_name:		None,
			nickname:		None,
			phone_number:	None,
			email:			None,
			address:		None
		}
    }

	pub fn id(&self) -> String {
		match self.id {
			Some(id) => return id.to_string(),
			None =>	return String::from(""),
		}
	}

	pub fn first_name(&self) -> &str {
		self.first_name.as_deref().unwrap_or("")
	}

	pub fn last_name(&self) -> &str {
		&self.last_name.as_deref().unwrap_or("")
	}

	pub fn nickname(&self) -> &str {
		&self.nickname.as_deref().unwrap_or("")
	}

	pub fn phone_number(&self) -> &str {
		&self.phone_number.as_deref().unwrap_or("")
	}
	pub fn email(&self) -> &str {
		&self.email.as_deref().unwrap_or("")
	}

	pub fn address(&self) -> &str {
		&self.address.as_deref().unwrap_or("")
	}

	pub fn set_first_name(&mut self, first_name: String) {
		self.first_name = Some(first_name)
	}

	pub fn set_last_name(&mut self, last_name: String) {
		self.last_name = Some(last_name)
	}
	
	pub fn set_nickname(&mut self, nickname: String) {
		self.nickname = Some(nickname)
	}

	pub fn set_phone_number(&mut self, phone_number: String) {
		self.phone_number = Some(phone_number)
	}

	pub fn set_address(&mut self, address: String) {
		self.address = Some(address)
	}

	pub fn set_email(&mut self, email: String) {
		self.email = Some(email)
	}
}
