fn fetch_schema_query() -> String {
    String::from(
        "CREATE TABLE credentials (
            id  INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL, service_name TEXT NOT NULL,
            username    TEXT NOT NULL,           
            password_hash   TEXT NOT NULL,           
            created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        );
         CREATE TABLE users (
            id  INTEGER PRIMARY KEY AUTOINCREMENT,
            username    TEXT NOT NULL UNIQUE,
            password_hash   TEXT NOT NULL,
            created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        ",
    )
}
