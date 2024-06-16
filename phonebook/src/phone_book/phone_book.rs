use super::contact::{self, Contact};

pub struct PhoneBook {
	contacts: Vec<Contact>,
}

impl PhoneBook {
	pub fn new() -> PhoneBook {
		PhoneBook {
			contacts: Vec::new(),
		}
	}

	pub fn add_contact(
		&mut self,
		first_name: &str,
		last_name: &str,
		nickname: &str,
		phone_number: &str,
		email: &str,
		address: &str,
	) {
		let contact: Contact = contact::Contact::new(first_name, last_name, nickname, phone_number, email, address);
		self.contacts.push(contact);
	}

	pub fn search_contacts(&self) {
		for contact in &self.contacts {
			println!("{} {}", contact.first_name(), contact.last_name());
		}
	}
}