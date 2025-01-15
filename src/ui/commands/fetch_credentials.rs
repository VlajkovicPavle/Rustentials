use crate::core::authentification::decrypt_password;
use crate::models::user::User;
use crate::repository::services;
use crate::ui::lang::langs::fetch_text;
use std::io;

pub async fn fetch_credentials(current_user: &User) -> bool {
    println!("\n{}", fetch_text("ask_for_label"));
    let mut service_label = String::new();
    io::stdin()
        .read_line(&mut service_label)
        .expect("Failed to fetch label");
    service_label = service_label.replace('\n', "");
    match services::fetch_credentials(&service_label, current_user).await {
        Some(credentials) => {
            let decrypted_password = decrypt_password(
                &current_user.master_key.unwrap(),
                &credentials.encrypted_password,
            );
            match String::from_utf8(decrypted_password) {
                Ok(password) => {
                    println!("\n***************************");
                    println!("Username: {}", credentials.username);
                    println!("Passowrd : {}", password);
                    println!("Label: {}", credentials.label);
                    println!("***************************");
                    true
                }
                Err(error) => {
                    println!("Failed to decrypt password {}", error);
                    false
                }
            }
        }
        None => false,
    }
}
