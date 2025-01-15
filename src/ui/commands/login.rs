use crate::core::authentification;
use crate::core::authentification::generate_crypto_key;
use crate::models::user::User;
use crate::repository::services;
use crate::ui::lang::langs::fetch_text;

pub async fn login_user(username: &str) -> Option<User> {
    let user_input_password: String =
        rpassword::prompt_password(fetch_text("password_prompt")).unwrap();
    let master_password_hash: String = services::fetch_user_password_hash(username).await.unwrap();
    if authentification::verify_master_password(&master_password_hash, &user_input_password) {
        let current_user = User {
            username: String::from(username),
            master_key: Some(generate_crypto_key(&master_password_hash)),
            password_hash: master_password_hash,
        };
        return Some(current_user);
    }
    None
}
