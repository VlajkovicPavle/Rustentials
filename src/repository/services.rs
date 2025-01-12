use crate::repository::db::fetch_db_instances;
use sqlx::query;
include!("./queries/insert_credentials.rs");

pub async fn insert_credentials() {
    let service_name = "facebook";
    let username = "pavle";
    let password = "password";
    match fetch_db_instances().await {
        Ok(instances) => {
            let result = query(&fetch_insert_credentials_query())
                .bind(service_name)
                .bind(username)
                .bind(password)
                .execute(&instances)
                .await;
            instances.close().await;
            println!("{:?}", result);
        }
        Err(error) => {
            panic!("Failed to fetch database instance! {}", error)
        }
    };
}
