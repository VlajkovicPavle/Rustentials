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
    /// Initiate login | Add -s if you want to save login
    Login {
        #[arg(short = 's')]
        save: bool,
    },
}

pub fn run_cli() {
    let arguments = Cli::parse();
    match arguments.command {
        Commands::Login { save } => {
            authentificate_user(save);
        }
    }
}
