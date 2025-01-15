fn get_credentials_query() -> String {
    String::from(
        r#"
            SELECT c.username, c.password, c.label 
            FROM credentials c
            JOIN users u ON c.user_id = u.id
            WHERE u.username = ? 
            AND c.label = ?;
      "#,
    )
}
