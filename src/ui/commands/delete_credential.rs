use crate::models::user::User;
use crate::repository;
use crate::ui::lang::langs::fetch_text;
use std::io;

pub async fn delete_credential(current_user: &User) -> bool {
    println!("\n{}", fetch_text("ask_for_label"));
    let mut service_label = String::new();
    io::stdin()
        .read_line(&mut service_label)
        .expect("Failed to fetch label");
    service_label = service_label.replace('\n', "");
    if repository::services::delete_credentials(&service_label, current_user).await {
        println!("{}", fetch_text("deletion_of_credential_ok"));
        return true;
    }
    println!("{}", fetch_text("deletion_of_credential_fail"));
    false
}
