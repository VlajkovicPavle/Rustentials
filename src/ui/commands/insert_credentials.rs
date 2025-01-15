use crate::core::authentification::encrypt_password;
use crate::models::credential::Credential;
use crate::models::user::User;
use crate::repository::services;
use crate::ui::lang::langs::fetch_text;
use std::io;

pub async fn insert_credentials(current_user: &User) -> bool {
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

    let master_key = current_user.master_key.expect("Failed to fetch master key");
    let encrypted_password_arr = encrypt_password(&master_key, &service_password);
    let credentials = Credential {
        username: service_username,
        service_name: service_label,
        encrypted_password: encrypted_password_arr,
    };
    if services::insert_credentials(&credentials, current_user).await {
        println!("{}", fetch_text("credentials_added"));
        true
    } else {
        println!("{}", fetch_text("credentials_failure_to_add"));
        false
    }
}
