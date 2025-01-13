pub fn get_user_password_hash() -> String {
    String::from(
        r#"
             SELECT password_hash FROM users 
             WHERE username = (?)
        "#,
    )
}
