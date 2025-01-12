fn fetch_insert_credentials_query() -> String {
    String::from(
        r#"
        INSERT INTO credentials (service_name, username, password)
        VALUES (?, ?, ?)
        "#,
    )
}
