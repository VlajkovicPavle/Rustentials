use crate::models::user;
use crate::repository::db::fetch_db_instances;
use sqlx::query;
use sqlx::query_scalar;
include!("./queries/insert_credentials.rs");
include!("./queries/insert_user_data.rs");
include!("./queries/get_user_password_hash.rs");

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
            println!("Failed to fetch database instance {} ", error);
        }
    };
}

pub async fn insert_user(user_data: &user::User) -> bool {
    match fetch_db_instances().await {
        Ok(instances) => {
            let result = query(&fetch_insert_user_data_query())
                .bind(&user_data.username)
                .bind(&user_data.password_hash)
                .execute(&instances)
                .await;
            instances.close().await;
            match result {
                Ok(_) => true,
                Err(error) => {
                    println!("Failed insert_user {}", error);
                    false
                }
            }
        }
        Err(error) => {
            println!("Failed to fetch database instance {} ", error);
            false
        }
    }
}

pub async fn fetch_user_password_hash(username: &str) -> Option<String> {
    match fetch_db_instances().await {
        Ok(instances) => {
            let result = query_scalar::<_, String>(&(get_user_password_hash()))
                .bind(username)
                .fetch_one(&instances)
                .await;
            instances.close().await;
            match result {
                Ok(row) => Some(row),
                Err(err) => {
                    println!("Failed to fetch password hash {}", err);
                    None
                }
            }
        }
        Err(error) => {
            println!("Failed to fetch database instance {}", error);
            None
        }
    }
}
