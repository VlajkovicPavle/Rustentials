use rustentials::core::authentification::*;
#[cfg(test)]
use rustentials::core::authentification::{
    generate_master_password_hash, validate_password, verify_master_password,
};

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
