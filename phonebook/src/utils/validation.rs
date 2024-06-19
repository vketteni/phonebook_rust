
fn is_numbers_and_spaces(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric() || c.is_whitespace())
}

pub fn validate_length(value: &str, max_length: usize) -> Result<(), String> {
    if value.len() > max_length {
        Err(format!("Value exceeds maximum length of {}", max_length))
    } else {
        Ok(())
    }
}

pub fn validate_phone_number(pn: &str) -> Result<(), String> {
    if !is_numbers_and_spaces(pn.trim()) {
        Err("Can only contain numbers and spaces.".to_string())
    } else {
        Ok(())
    }
}

pub fn validate_email(email: &str) -> Result<(), String> {
    let email: &str = email.trim();
    if email.is_empty() {
        return Err("Cannot be empty.".to_string());
    }
	if email.matches('@').count() != 1 {
		return Err("Not a valid address.".to_string());
	}
    let at_pos = email.find('@').unwrap();
	let local_part: &str = &email[..at_pos];
	let domain_part: &str = &email[(at_pos + 1)..];

	if !validate_parts(local_part) {
		return Err("Not a valid address.".to_string());
	}
	if !validate_parts(domain_part) {
		return Err("Not a valid address.".to_string());
	}
	Ok(())
}

fn validate_parts(part: &str) -> bool {
    part.split('.').all(|substr| {
        substr
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    })
}
