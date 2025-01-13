use crate::ui::commands::{login, register};
use crate::ui::lang::langs::fetch_text;
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Debug, Parser)]
#[command(version = "1.0.0", about = fetch_text("cli_about"), long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = fetch_text("register_about"))]
    Register {
        #[arg(short = 'u', long = fetch_text("register_username_long"), help = fetch_text("register_username_help"))]
        username: String,
    },
    #[command(about = fetch_text("login_about"))]
    Login {
        #[arg(short = 'u', long = fetch_text("login_user"))]
        username: String,
    },
}

pub async fn run_cli() {
    let arguments = Cli::parse();
    match arguments.command {
        Commands::Register { username } => {
            println!("{}", fetch_text("register_password_requirements"));
            if !register::register_user(&username).await {
                println!("{}", fetch_text("register_fail"));
                return;
            }
            println!("{}", fetch_text("register_ok"))
        }
        Commands::Login { username } => {
            if login::login_user(&username).await {
                println!("{}", fetch_text("login_ok"));
            } else {
                println!("{}", fetch_text("login_fail"))
            }
        }
    }
}
