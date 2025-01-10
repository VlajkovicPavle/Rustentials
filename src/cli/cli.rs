use clap::parser;

#[command(name = "Rustentials")]
#[command(about = "Password manager", version = "1.0")]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn parse_input() {
    let cli = Cli::parse();
    println!("{}", cli.name);
}
