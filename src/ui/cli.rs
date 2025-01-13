use crate::ui::commands::register;
use clap::{builder::Str, Parser, Subcommand};
use rpassword::prompt_password;

/// Simple program to greet a person
#[derive(Debug, Parser)]
#[command(version = "1.0.0", about = "Password manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Initiate register")]
    Register {
        #[arg(short = 'u', long = "username", help = "Desired username")]
        username: String,
    },
    #[command(about = "Initiate login")]
    Login {
        #[arg(short = 'u', long = "username")]
        username: String,
        #[arg(short = 's', long = "save", help = "Save the session")]
        save: bool,
    },
    #[command(about = "Save credentials")]
    Save {
        #[arg(short = 'l', long = "label", help = "Credential label")]
        label: String,
        #[arg(short = 'u', long = "username", help = "Credential username")]
        username: String,
    },
    #[command(about = "Yank credentials by label")]
    Yank {
        #[arg(
            short = 'l',
            long = "label",
            help = "Label that identifies credentials"
        )]
        label: String,
        #[arg(
            short = 'c',
            long = "clipboard",
            help = "Save credentials to clipboard"
        )]
        clipboard: bool,
    },
    #[command(about = "List all credential labels")]
    List {},
}

pub async fn run_cli() {
    let arguments = Cli::parse();
    match arguments.command {
        Commands::Register { username } => {
            println!(
                "   Password requirements: 
                    Length >= 8 
                    One uppercase letter  
                    One lowercase letter  
                    One special character: !@#$%^&*(),.? 
                 "
            );
            if !register::register_user(&username).await {
                println!("Failed to register!");
                return;
            }
            println!("Successfully registered!")
        }
        Commands::Login { username, save } => {}
        Commands::Yank { label, clipboard } => {}
        Commands::List {} => {}
        Commands::Save { label, username } => {}
    }
}
