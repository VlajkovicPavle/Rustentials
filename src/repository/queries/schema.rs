fn fetch_schema() -> String {
    String::from(
        "CREATE TABLE passwords (
        id  INTEGERER PRIMARY KEY AUTOINCREMENT,
        service_name    TEXT NOT NULL,       
        username    TEXT NOT NULL,           
        password    TEXT NOT NULL,           
        url     TEXT,                         
        notes   TEXT,                       
        created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP
    );
    CREATE TABLE settings (
        id  INTEGER PRIMARY KEY CHECK (id = 1), 
        master_key_hash     TEXT NOT NULL,        
        created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
    );",
    )
}
