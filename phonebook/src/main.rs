mod phone_book;

fn main() {
    let mut phone_book = phone_book::PhoneBook::new();
    phone_book.add_contact("Vincent", "Kette", "Vinny", "0172154223", "vketteni@42berlin.de", "rummelstreet 69");
    phone_book.search_contacts();
}
