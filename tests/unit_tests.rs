#[cfg(test)]
use rustentials::core::authentification::{
    generate_master_password_hash, validate_password, verify_master_password,
};
use rustentials::models::user::User;
use rustentials::repository::db::{create_database, fetch_db_instances};
use rustentials::repository::services::{fetch_user_password_hash, insert_user};

// Authentification tests

#[test]
fn password_validator_test() {
    let mut test_password = "";
    assert!(!validate_password(test_password));
    test_password = "abcd";
    assert!(!validate_password(test_password));
    test_password = "Abcdefgg";
    assert!(!validate_password(test_password));
    test_password = "Abcd1213ad";
    assert!(!validate_password(test_password));
    test_password = "P@ssw0rd";
    assert!(validate_password(test_password));
}

#[test]
fn master_password_verify() {
    let mut master_password = "P@ssw0rd";
    let master_password_hash = generate_master_password_hash(master_password);
    assert!(verify_master_password(
        &master_password_hash,
        master_password
    ));
    master_password = "abc";
    assert!(!verify_master_password(
        &master_password_hash,
        master_password
    ));
    master_password = "";
    assert!(!verify_master_password(
        &master_password_hash,
        master_password
    ));
}

// Database querries tests
#[async_std::test]
async fn test_database_creation() {
    // Successful creation of database
    assert!(create_database("sqlite://test.db").await);
    // No creation if it exists
    assert!(!create_database("sqlite://test.db").await);
}

#[async_std::test]
async fn test_fetching_app_db_instance() {
    assert!(fetch_db_instances().await.is_ok());
}

#[async_std::test]
async fn test_inserting_user() {
    let test_user: User = User {
        username: String::from("pavle"),
        password_hash: String::from("asdasdd"),
    };
    assert!(insert_user(&test_user).await);
}

#[async_std::test]
async fn test_fething_user_password() {
    let test_user: User = User {
        username: String::from("luka"),
        password_hash: String::from("asdasdd"),
    };
    assert!(insert_user(&test_user).await);
    assert!(fetch_user_password_hash(&test_user.username)
        .await
        .is_some());
}
