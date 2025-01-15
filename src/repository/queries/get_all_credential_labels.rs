pub fn get_all_credential_labels_query() -> String {
    String::from(
        r#"
            SELECT label FROM 
            credentials c join 
            users u on u.id = c.user_id 
            where u.username = ?
        "#,
    )
}
