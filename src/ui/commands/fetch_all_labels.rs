use crate::ui::lang::langs::fetch_text;
use crate::{models::user::User, repository::services::fetch_all_credential_labels};

pub async fn fetch_all_labels(current_user: &User) -> bool {
    match fetch_all_credential_labels(current_user).await {
        Some(labels) => {
            println!("{}", fetch_text("fetch_labels_title"));
            for label in labels {
                println!("* {}", label);
            }
            true
        }
        None => {
            println!("Failed to fetch labels!");
            false
        }
    }
}
