use crate::ui::lang::langs::fetch_text;
use std::io::{self, Write};

fn fetch_command_number_from_cli() -> u16 {
    let mut user_input = String::new();
    print!("Input number of action: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");
    let number = user_input.trim().parse::<u16>().unwrap_or_default();
    number
}

fn choose_command() -> bool {
    let command_number = fetch_command_number_from_cli();
    match command_number {
        1 => true,
        2 => false,
        _ => false,
    }
}

pub fn run_app() {
    println!("\n============================");
    println!("{}\n", fetch_text("greet"));
    loop {
        println!("\n{}", fetch_text("menu_title"));
        println!("----------------------------");
        println!("1) {}", fetch_text("insert_credential"));
        println!("2) Quit the app");
        if !choose_command() {
            break;
        }
    }
}
