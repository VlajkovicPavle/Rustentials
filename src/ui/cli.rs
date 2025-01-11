use crate::core::authentification::authentificate_user;
use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Debug, Parser)]
#[command(version = "1.0.0", about = "Password manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Initiate login")]
    Login {
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

pub fn run_cli() {
    let arguments = Cli::parse();
    match arguments.command {
        Commands::Login { save } => {
            let user_password = rpassword::prompt_password("Your password: ").unwrap();
            authentificate_user(save, &user_password);
        }
        Commands::Yank { label, clipboard } => {}
        Commands::List {} => {}
        Commands::Save { label, username } => {}
    }
}
