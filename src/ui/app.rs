use super::commands::insert_credentials::insert_credentials;
use crate::models::user::User;
use crate::ui::commands::fetch_credentials::fetch_credentials;
use crate::ui::lang::langs::fetch_text;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, SetTitle};
use std::io;
use std::io::{stdout, Write};

fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::FromCursorUp)).expect("Failed to clear terminal");
    execute!(stdout(), MoveTo(0, 0)).expect("Failed to move cursor to top");
}

fn set_terminal_title() {
    execute!(stdout(), SetTitle("Rustentials")).expect("Failed to set terminal title");
}

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
    clear_terminal();
    match command_number {
        1 => insert_credentials(current_user).await,
        2 => fetch_credentials(current_user).await,
        _ => false,
    }
}

pub async fn run_app(curent_user: &User) {
    set_terminal_title();
    clear_terminal();
    println!("\n============================");
    println!("{}", fetch_text("greet"));
    println!("============================");
    loop {
        println!("\n{}", fetch_text("menu_title"));
        println!("----------------------------");
        println!("1) {}", fetch_text("insert_credential"));
        println!("2) {}", fetch_text("fetch_credentials"));
        println!("3) {}", fetch_text("terminate_app"));
        if !choose_command(curent_user).await {
            break;
        }
    }
}
