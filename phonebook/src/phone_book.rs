use super::contact::Contact;
use super::utils::input::get_input;

pub struct PhoneBook {
    contacts: Vec<Contact>,
}

impl PhoneBook {
    pub fn new() -> PhoneBook {
        PhoneBook {
            contacts: Vec::new(),
        }
    }


    pub fn prompt_new_contact(&mut self) {

    }

    pub fn search_contacts(&self) {
        for contact in &self.contacts {
            println!("{} {}", contact.first_name(), contact.last_name());
        }
    }
}

