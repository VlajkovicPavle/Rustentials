use crate::core::authentification;

pub fn register_user(username: &str) -> bool {
    let password = rpassword::prompt_password("Your password: ").unwrap();
    if authentification::validate_password(&password) {
        let password_second_time = rpassword::prompt_password("Re-enter password: ").unwrap();
        if password != password_second_time {
            return false;
        }
        return true;
        //TODO
        //All is good here!
    }
    false
}
