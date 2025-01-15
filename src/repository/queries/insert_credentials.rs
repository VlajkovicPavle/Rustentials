fn fetch_insert_credentials_query() -> String {
    String::from(
        r#"
            INSERT INTO credentials (user_id, username, password, label)
            VALUES (
                (SELECT id FROM users WHERE username = ?),  ?,  ?, ? 
            )
        "#,
    )
}
