mod core;
mod models;
mod repository;
mod ui;

include!("./core/authentification.rs");

use ui::cli::run_cli;

#[async_std::main]
async fn main() {
    run_cli().await;
}
