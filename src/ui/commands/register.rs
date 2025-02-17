use crate::core::authentification;
use crate::models::user;
use crate::repository::services;
use crate::ui::lang::langs::fetch_text;

pub async fn register_user(username: &str) -> bool {
    let password = rpassword::prompt_password(fetch_text("password_prompt")).unwrap();
    if authentification::validate_password(&password) {
        let password_second_time =
            rpassword::prompt_password(fetch_text("password_confirmation")).unwrap();
        if password != password_second_time {
            return false;
        }
        let password_hash = authentification::generate_master_password_hash(&password);
        let user_data: user::User = user::User {
            username: username.to_string(),
            password_hash: password_hash.to_string(),
            master_key: None,
        };
        return services::insert_user(&user_data).await;
    }
    false
}
