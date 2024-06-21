mod constants;
mod contact;
mod phone_book;
mod utils;

use utils::{cmdl_utils::clear_screen, input};
use phone_book::PhoneBook;
use rusqlite::Connection;


fn main() {
    let conn = Connection::open("phonebook.db").expect("Failed to open database.");
    PhoneBook::init_db(&conn).expect("Failed to initialize database.");

    let phone_book = PhoneBook::new(conn);

    clear_screen();
    loop {
        println!("Menu:");
        println!("1. ADD");
        println!("2. SEARCH");
        println!("3. EXIT");

        match input::prompt_input(
            "Enter your choice: ",
            constants::MAX_MENU_OPTION_LENGTH,
        ).as_str() {
            "1" => {
                if let Err(e) = phone_book.add_contact_interactively() {
                    println!("Failed to add contact: {}", e);
                }
            },
			"2" => {
                match phone_book.get_contacts() {
                    Ok(contacts) => phone_book.search_contacts_interactively(&contacts),
                    Err(e) => println!("Error failed to search contacts: {}.", e)
                }
			},
			"3" => {
				println!("Exiting..");
                break;
			},
			_ => println!("Invalid choice, please try again."),
        }
    }
}
