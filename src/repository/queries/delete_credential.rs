fn get_delete_credentials_query() -> String {
    String::from(
        r#"
            DELETE FROM credentials  
            WHERE user_id = (
                             SELECT id 
                             FROM users 
                             WHERE username = ?
                            )
            AND label = ?
        "#,
    )
}
