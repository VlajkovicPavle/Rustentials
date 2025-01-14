use crate::models::credential::Credential;
use crate::repository::services;
use crate::ui::lang::langs::fetch_text;
use std::io;

pub async fn insert_credentials(master_username: &str) -> bool {
    println!("\n{}", fetch_text("ask_for_username"));
    let mut service_username = String::new();
    io::stdin()
        .read_line(&mut service_username)
        .expect("Failed to fetch username");
    println!("{}", fetch_text("ask_for_label"));
    let mut service_label = String::new();
    io::stdin()
        .read_line(&mut service_label)
        .expect("Failed to fetch label");
    println!("{}", fetch_text("ask_for_password"));
    let mut service_password = String::new();
    io::stdin()
        .read_line(&mut service_password)
        .expect("Failed to fetch password");

    let credentials: Credential = Credential {
        username: service_username,
        encrypted_password: service_password,
        service_name: service_label,
    };
    if services::insert_credentials(&credentials, master_username).await {
        println!("{}", fetch_text("credentials_added"));
        true
    } else {
        println!("{}", fetch_text("credentials_failure_to_add"));
        false
    }
}
