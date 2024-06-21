use super::constants;
use super::input;
use super::contact::Contact;
use super::utils::input::prompt_input;

pub struct PhoneBook {
    conn: rusqlite::Connection,
}

impl PhoneBook {
    pub fn new(conn: rusqlite::Connection) -> PhoneBook {
        PhoneBook { conn }
    }

    pub fn init_db(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS contact (
                id              INTEGER PRIMARY KEY,
                first_name      TEXT,
                last_name       TEXT,
                nickname        TEXT,
                phone_number    TEXT,
                email           TEXT,
                address         TEXT
        )",
            [],
        )?;
        Ok(())
    }

    pub fn add_contact(&self, contact: &Contact) -> rusqlite::Result<usize> {
        self.conn.execute(
            "INSERT INTO contact (first_name, last_name, nickname, phone_number, email, address)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                contact.first_name(),
                contact.last_name(),
                contact.nickname(),
                contact.phone_number(),
                contact.email(),
                contact.address()
            ],
        )
    }

    pub fn get_contacts(&self) -> rusqlite::Result<Vec<Contact>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, first_name, last_name, nickname, phone_number, email, address FROM contact",
        )?;
        let contact_iter = stmt.query_map([], |row| {
            Ok(Contact {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                nickname: row.get(3)?,
                phone_number: row.get(4)?,
                email: row.get(5)?,
                address: row.get(6)?,
            })
        })?;
        let mut contacts = Vec::new();
        for contact in contact_iter {
            contacts.push(contact?);
        }
        Ok(contacts)
    }

    pub fn add_contact_interactively(&self) -> Result<usize, String> {
        let mut contact: Contact = Contact::new();

        let first_name = prompt_input("Enter first name: ", constants::MAX_FIRST_NAME_LENGTH);
        let last_name = prompt_input("Enter last_name: ", constants::MAX_LAST_NAME_LENGTH);
        let nickname = prompt_input("Enter nickname: ", constants::MAX_NICKNAME_LENGTH);
        let phone_number = prompt_input("Enter phone_number: ", constants::MAX_PHONE_NUMBER_LENGTH);
        let email = prompt_input("Enter email: ", constants::MAX_EMAIL_LENGTH);
        let address = prompt_input("Enter address: ", constants::MAX_ADDRESS_LENGTH);

        contact.set_first_name(first_name);
        contact.set_last_name(last_name);
        contact.set_nickname(nickname);
        contact.set_phone_number(phone_number);
        contact.set_email(email);
        contact.set_address(address);

        self.add_contact(&contact).map_err(|err| err.to_string())
    }

    fn print_page(&self, contacts: &[Contact], page: usize, page_size: usize) {
        let start = page * page_size;
        let end = (start + page_size).min(contacts.len());

        println!("Displaying contacts {} to {}:", start + 1, end);
        for contact in &contacts[start..end] {
            println!(
                "ID: {}\nFirst Name: {}\nLast Name: {}\nNickname: {}\nPhone Number: {}\nEmail: {}\nAddress: {}\n",
                contact.id(), 
                contact.first_name(), 
                contact.last_name(), 
                contact.nickname(), 
                contact.phone_number(), 
                contact.email(), 
                contact.address(),
            );
        }
        println!("Page: {}/{}", start+1, (contacts.len() + (page_size - 1))/ page_size);
    }

    pub fn search_contacts_interactively(&self, contacts: &Vec<Contact>) {
        let mut current_page: usize = 0;
        let page_size = constants::PHONEBOOK_PAGE_SIZE;
        
        loop {
            self.print_page(contacts, current_page, page_size);

            println!("Options:");
            println!("n - next page");
            println!("p - previous page");
            println!("s - search specific user");
            println!("q - quit search");
        
            match input::prompt_input(
                "Enter your choice: ",
                constants::MAX_MENU_OPTION_LENGTH,
            ).as_str() {
                "n" => { current_page = (current_page + 1).max(((contacts.len() + (page_size - 1))/ page_size) - 1); },
                "p" => { current_page = (current_page - 1).min(0); },
                "s" => { println!("Specific search not implemented yet.."); },
                "q" => {
                    println!("Exiting..");
                    break;
                },
                _ => println!("Invalid choice, please try again."),
            }
        }
    }
}
