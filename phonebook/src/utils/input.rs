use std::io::{self, Write};
use crate::utils::validation::validate_length;

pub fn read_input() -> String {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input. Bad input.");
    input.trim().to_string()
}

pub fn prompt_input(prompt: &str, max_length: usize) -> String {
    loop {
        print!("{}", prompt);
        let input = read_input();
        match validate_length(&input, max_length) {
            Ok(()) => return input,
            Err(err) => println!("Error: {}.\nPlease try again.", err),
        }
    }
}
