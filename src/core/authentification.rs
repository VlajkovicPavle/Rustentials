use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
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
}

pub fn generate_master_password_hash(password: &str) {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    // println!("{}", PasswordHash::new(&password_hash).unwrap());
    // assert!(Argon2::default().verify_password(password, &parsed_hash).is_ok());
}
