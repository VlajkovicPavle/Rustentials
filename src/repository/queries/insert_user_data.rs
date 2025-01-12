pub fn fetch_insert_user_data_query() -> String {
    String::from(
        r#"
            INSERT INTO users (username, password_hash)  
            VALUES (? ,?) 
        "#,
    )
}
