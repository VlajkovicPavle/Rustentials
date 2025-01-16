use once_cell::sync::Lazy;
use std::collections::HashMap;

static LANGUAGE: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut dictionary = HashMap::new();
    // Insert language here!
    // CLI
    // ====================================
    dictionary.insert("cli_about", "Password manager");
    // Register
    dictionary.insert("register_about", "Register user");
    dictionary.insert("register_username_help", "Desired username");
    dictionary.insert("register_username_long", "username");
    dictionary.insert(
        "register_password_requirements",
        "Password requirements:
                    Length >= 8 
                    One uppercase letter  
                    One lowercase letter  
                    One special character: !@#$%^&*(),.? 
                  ",
    );
    dictionary.insert("register_fail", "Failed to register!");
    dictionary.insert("register_ok", "Successfully registtered!");
    // Login
    dictionary.insert("login_about", "Initiate login");
    dictionary.insert("login_user_long", "username");
    dictionary.insert("login_ok", "Successfful login!");
    dictionary.insert("login_fail", "Failed login!");
    // Common
    dictionary.insert("password_prompt", "Your password: ");
    dictionary.insert("password_confirmation", "Re-enter password: ");

    // ====================================
    dictionary.insert("listCredentials", "List all credential labels");
    dictionary.insert("greet", "Welcome to rustentials");
    dictionary.insert("menu_title", "Available commands:");
    // Commands
    dictionary.insert("list_all", "List all credentials label");
    dictionary.insert("insert_credential", "Add credential");
    dictionary.insert("ask_for_username", "Provide service username: ");
    dictionary.insert("ask_for_label", "Provide service label: ");
    dictionary.insert("ask_for_password", "Provide service password:");
    dictionary.insert("credentials_added", "Successfully added credentials!");
    dictionary.insert("credentials_failure_to_add", "Failure to add credentials");
    dictionary.insert("ask_for_number_of_action", "Choose the number of action: ");
    dictionary.insert("terminate_app", "Quit the app");
    dictionary.insert("fetch_credentials", "Fetch credential");
    dictionary.insert("fetch_labels", "List all saved labels");
    dictionary.insert("fetch_labels_title", "List of all of the labels: ");
    dictionary.insert("delete_credential", "Remove credential");
    dictionary.insert(
        "deletion_of_credential_ok",
        "Successfful deleted credential!",
    );
    dictionary.insert(
        "deletion_of_credential_fail",
        "Failed to delete credential!",
    );

    dictionary
});

static FALLBACK_TEXT: &str = "Missing text...";

pub fn fetch_text(key: &str) -> &str {
    match LANGUAGE.get(key) {
        Some(text) => text,
        None => FALLBACK_TEXT,
    }
}
