pub struct User {
    pub username: String,
    pub password_hash: String,
    pub master_key: Option<[u8; 32]>,
}
