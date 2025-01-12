mod core;
mod repository;
mod ui;

include!("./core/authentification.rs");

use core::authentification;

use ui::cli::run_cli;

// #[async_std::main]
// async fn main() { repository::services::insert_credentials().await;
// }
//
fn main() {
    authentification::generate_master_password_hash("test!");
}
