use regex::bytes::Regex;
const MINIMUM_PASSWORD_LENGTH: usize = 8;

pub fn validate_password(password: &str) -> bool {
    let length_check = password.len() >= MINIMUM_PASSWORD_LENGTH;
    let uppercase_check = Regex::new(r"[A-Z]").unwrap();
    let lowercase_check = Regex::new(r"[a-z]").unwrap();
    let digit_check = Regex::new(r"[0-9]").unwrap();
    let special = Regex::new(r"[!@#$%^&*(),.?]").unwrap();

    length_check
        && uppercase_check.is_match(password.as_bytes())
        && lowercase_check.is_match(password.as_bytes())
        && digit_check.is_match(password.as_bytes())
        && special.is_match(password.as_bytes())
}

pub fn encrypt_password(password: &str) {}
