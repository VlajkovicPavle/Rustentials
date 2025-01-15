pub struct Credential {
    pub username: String,
    pub encrypted_password: Vec<u8>,
    pub label: String,
}
