pub fn authentificate_user(save_login: bool) {
    let user_password: String = fetch_password_from_cli();
    println!("The password is {}", user_password);
}

fn fetch_password_from_cli() -> String {
    let password = rpassword::read_password().unwrap();
    // TODO
    // Do some password validation!
    password
}
