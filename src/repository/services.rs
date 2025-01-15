use crate::models::credential::Credential;
use crate::models::user::User;
use crate::repository::db::fetch_db_instances;
use sqlx::query;
use sqlx::query_scalar;
include!("./queries/insert_credentials.rs");
include!("./queries/insert_user_data.rs");
include!("./queries/get_user_password_hash.rs");

pub async fn insert_credentials(credential: &Credential, current_user: &User) -> bool {
    match fetch_db_instances().await {
        Ok(instances) => {
            let result = query(&fetch_insert_credentials_query())
                .bind(&current_user.username)
                .bind(&credential.username)
                .bind(&credential.encrypted_password)
                .bind(&credential.service_name)
                .execute(&instances)
                .await;
            instances.close().await;
            match result {
                Ok(_) => true,
                Err(error) => {
                    println!("Failed to insert_credentials {}", error);
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

pub async fn insert_user(user_data: &User) -> bool {
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
