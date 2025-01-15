use crate::models::user::User;
use crate::ui::lang::langs::fetch_text;
use std::io::{self, Write};

use super::commands::insert_credentials::insert_credentials;

fn fetch_command_number_from_cli() -> u16 {
    let mut user_input = String::new();
    print!("{}", fetch_text("ask_for_number_of_action"));
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");
    let number = user_input.trim().parse::<u16>().unwrap_or_default();
    number
}
async fn choose_command(current_user: &User) -> bool {
    let command_number = fetch_command_number_from_cli();
    match command_number {
        1 => insert_credentials(current_user).await,
        2 => false,
        _ => false,
    }
}

pub async fn run_app(curent_user: &User) {
    println!("\n============================");
    println!("{}", fetch_text("greet"));
    println!("============================");
    loop {
        println!("\n{}", fetch_text("menu_title"));
        println!("----------------------------");
        println!("1) {}", fetch_text("insert_credential"));
        println!("2) {}", fetch_text("terminate_app"));
        if !choose_command(curent_user).await {
            break;
        }
    }
}
