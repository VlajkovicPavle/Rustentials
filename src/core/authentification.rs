use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use regex::bytes::Regex;
use simple_crypt::{decrypt, encrypt};
const MINIMUM_PASSWORD_LENGTH: usize = 8;

pub fn validate_password(password: &str) -> bool {
    let length_check = password.len() >= MINIMUM_PASSWORD_LENGTH;
    let uppercase_check = Regex::new(r"[A-Z]").unwrap();
    let lowercase_check = Regex::new(r"[a-z]").unwrap();
    let digit_check = Regex::new(r"[0-9]").unwrap();
    let special_check = Regex::new(r"[!@#$%^&*(),.?]").unwrap();
    length_check
        && uppercase_check.is_match(password.as_bytes())
        && lowercase_check.is_match(password.as_bytes())
        && digit_check.is_match(password.as_bytes())
        && special_check.is_match(password.as_bytes())
}

pub fn generate_master_password_hash(password: &str) -> String {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    // println!("{}", PasswordHash::new(&password_hash).unwrap());
    // assert!(Argon2::default().verify_password(password, &parsed_hash).is_ok());
    password_hash
}

pub fn verify_master_password(master_password_hash: &str, user_input_password: &str) -> bool {
    let argon2 = Argon2::default();
    let master_password_re_hashed = PasswordHash::new(master_password_hash).unwrap();
    argon2
        .verify_password(user_input_password.as_bytes(), &master_password_re_hashed)
        .is_ok()
}

pub fn generate_crypto_key(master_password: &str) -> [u8; 32] {
    let salt = SaltString::generate(OsRng);
    let mut output_key = [0u8; 32];
    let argon2 = Argon2::default();
    argon2
        .hash_password_into(
            master_password.as_bytes(),
            salt.as_str().as_bytes(),
            &mut output_key,
        )
        .expect("Failed to fetch crypto_key");
    output_key
}

pub fn encrypt_password(crypto_key: &[u8], password: &str) -> Vec<u8> {
    encrypt(password.as_bytes(), crypto_key).expect("Failed to encrypt password")
}

pub fn decrypt_password(crypto_key: &[u8], encrypted_data: &[u8]) -> Vec<u8> {
    decrypt(encrypted_data, crypto_key).expect("Failed to decrypt password")
}
