use crate::core::authentification;
use crate::repository::services;
use crate::ui::lang::langs::fetch_text;

pub async fn login_user(username: &str) -> bool {
    let user_input_password: String =
        rpassword::prompt_password(fetch_text("password_prompt")).unwrap();
    let master_password_hash: String = services::fetch_user_password_hash(username).await.unwrap();
    authentification::verify_master_password(&master_password_hash, &user_input_password)
}
