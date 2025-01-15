#[cfg(test)]
use rustentials::core::authentification::{
    decrypt_password, encrypt_password, generate_crypto_key, generate_master_password_hash,
    validate_password, verify_master_password,
};
use rustentials::models::credential::Credential;
use rustentials::models::user::User;
use rustentials::repository::db::{create_database, fetch_db_instances};
use rustentials::repository::services::{
    fetch_all_credential_labels, fetch_credentials, fetch_user_password_hash, insert_credentials,
    insert_user,
};

// Authentification tests
// ======================
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

#[test]
fn password_encryption_test() {
    let crypto_key = generate_crypto_key("P@ssw0rd");
    let test_password = "test_password";
    let encrypted_password = encrypt_password(&crypto_key, test_password);
    let decrypted_password = decrypt_password(&crypto_key, &encrypted_password);
    assert_eq!(
        test_password,
        &String::from_utf8(decrypted_password).unwrap()
    );
}

// Database querries tests
// ==========================
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
        username: String::from("milos"),
        password_hash: String::from("asdasdd"),
        master_key: None,
    };
    assert!(insert_user(&test_user).await);
}

#[async_std::test]
async fn test_fething_user_password() {
    let test_user: User = User {
        username: String::from("jakov"),
        password_hash: String::from("asdasdd"),
        master_key: None,
    };
    assert!(insert_user(&test_user).await);
    assert!(fetch_user_password_hash(&test_user.username)
        .await
        .is_some());
    assert!(fetch_user_password_hash("non_existent_user")
        .await
        .is_none());
}

#[async_std::test]
async fn test_inserting_credentials() {
    let master_user_password = "P@ssw0rd";
    let master_user_key = generate_crypto_key(master_user_password);
    let test_user: User = User {
        username: String::from("ognjen"),
        password_hash: String::from(master_user_password),
        master_key: Some(master_user_key),
    };
    let service_password = "12356";
    let encrypted_service_password =
        encrypt_password(&test_user.master_key.unwrap(), service_password);
    assert!(insert_user(&test_user).await);
    assert!(fetch_user_password_hash(&test_user.username)
        .await
        .is_some());
    let test_credentials = Credential {
        username: String::from("test_username"),
        encrypted_password: encrypted_service_password,
        label: String::from("test_service_name"),
    };
    assert!(insert_credentials(&test_credentials, &test_user).await);
}

#[async_std::test]
async fn test_fetching_credential() {
    let master_user_password = "P@ssw0rd";
    let master_user_key = generate_crypto_key(master_user_password);
    let test_user: User = User {
        username: String::from("katarina"),
        password_hash: String::from(master_user_password),
        master_key: Some(master_user_key),
    };
    let service_password = "12356";
    let encrypted_service_password =
        encrypt_password(&test_user.master_key.unwrap(), service_password);
    assert!(insert_user(&test_user).await);
    assert!(fetch_user_password_hash(&test_user.username)
        .await
        .is_some());
    let test_credentials = Credential {
        username: String::from("test_katarina"),
        encrypted_password: encrypted_service_password,
        label: String::from("test_katarina_service"),
    };
    assert!(insert_credentials(&test_credentials, &test_user).await);
    assert!(fetch_credentials("test_katarina_service", &test_user)
        .await
        .is_some());
}

#[async_std::test]
async fn test_fetching_all_labels() {
    let master_user_password = "P@ssw0rd";
    let master_user_key = generate_crypto_key(master_user_password);
    let test_user: User = User {
        username: String::from("katarina1"),
        password_hash: String::from(master_user_password),
        master_key: Some(master_user_key),
    };
    let service_password = "12356";
    let encrypted_service_password =
        encrypt_password(&test_user.master_key.unwrap(), service_password);
    assert!(insert_user(&test_user).await);
    assert!(fetch_user_password_hash(&test_user.username)
        .await
        .is_some());
    let test_credentials = Credential {
        username: String::from("test_katarina"),
        encrypted_password: encrypted_service_password.clone(),
        label: String::from("test_katarina_service"),
    };
    assert!(insert_credentials(&test_credentials, &test_user).await);
    let test_credentials1 = Credential {
        username: String::from("test_katarina1"),
        encrypted_password: encrypted_service_password,
        label: String::from("test_katarina_service1"),
    };
    assert!(insert_credentials(&test_credentials1, &test_user).await);
    assert_eq!(
        fetch_all_credential_labels(&test_user)
            .await
            .expect("Failed fetching all credentials")
            .len(),
        2
    );
}
