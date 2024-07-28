use e2esdk::{E2eCyber, E2eRSA2K};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

// fn setup() -> E2eRSA2K {
//     let mut e2e = E2eRSA2K::new();
//     e2e.generate_pairkey(Path::new(TEST_PRIV_KEY_PATH), Path::new(TEST_PUB_KEY_PATH))
//         .expect("Failed to generate key pair");
//     e2e.init(Path::new(TEST_PRIV_KEY_PATH), Path::new(TEST_PUB_KEY_PATH))
//         .expect("Failed to initialize with key files");
//     e2e
// }

#[test]
fn test_generate_pairkey() {
    let priv_key_path = "private_key.pem";
    let pub_key_path = "public_key.pem";

    let mut e2e_rsa = E2eRSA2K::new();
    e2e_rsa.generate_pairkey(Path::new(priv_key_path), Path::new(pub_key_path)).unwrap();

    assert!(Path::new(priv_key_path).exists());
    assert!(Path::new(pub_key_path).exists());

    // Cleanup generated key files
    fs::remove_file(priv_key_path).unwrap();
    fs::remove_file(pub_key_path).unwrap();
}

#[test]
fn test_init_with_generated_keys() {
    let priv_key_path = "private_key.pem";
    let pub_key_path = "public_key.pem";

    // Generate the keys first
    let mut e2e_rsa = E2eRSA2K::new();
    e2e_rsa.generate_pairkey(Path::new(priv_key_path), Path::new(pub_key_path)).unwrap();

    // Create a new instance and initialize it with the generated keys
    let mut e2e_rsa_initialized = E2eRSA2K::new();
    let init_result = e2e_rsa_initialized.init(Path::new(priv_key_path), Path::new(pub_key_path));

    assert!(init_result.is_ok());
    assert_eq!(e2e_rsa_initialized.is_initialized(), true);

    // Cleanup generated key files
    fs::remove_file(priv_key_path).unwrap();
    fs::remove_file(pub_key_path).unwrap();
}

#[test]
fn test_init_with_invalid_keys() {
    let priv_key_path = "invalid_private_key.pem";
    let pub_key_path = "invalid_public_key.pem";

    // Create invalid key files
    {
        let mut priv_key_file = File::create(priv_key_path).unwrap();
        priv_key_file.write_all(b"invalid private key").unwrap();

        let mut pub_key_file = File::create(pub_key_path).unwrap();
        pub_key_file.write_all(b"invalid public key").unwrap();
    }

    let mut e2e_rsa = E2eRSA2K::new();
    let init_result = e2e_rsa.init(Path::new(priv_key_path), Path::new(pub_key_path));

    assert!(init_result.is_err());
    assert_eq!(e2e_rsa.is_initialized(), false);

    // Cleanup invalid key files
    fs::remove_file(priv_key_path).unwrap();
    fs::remove_file(pub_key_path).unwrap();
}

#[test]
fn test_encrypt_decrypt() {
    let test_message: &[u8] = b"Test message for RSA encryption";

    let priv_key_path = "private_key.pem";
    let pub_key_path = "public_key.pem";

    // Generate the keys first
    let mut e2e_rsa = E2eRSA2K::new();
    e2e_rsa.generate_pairkey(Path::new(priv_key_path), Path::new(pub_key_path)).unwrap();
    let init_result = e2e_rsa.init(Path::new(priv_key_path), Path::new(pub_key_path));

    assert!(init_result.is_ok());
    assert_eq!(e2e_rsa.is_initialized(), true);

    // Encrypt the message
    let encrypted_message = e2e_rsa.encrypt(test_message).expect("Encryption failed");

    // Decrypt the message
    let decrypted_message = e2e_rsa.decrypt(&encrypted_message).expect("Decryption failed");

    // Verify that the decrypted message is the same as the original message
    assert_eq!(decrypted_message, test_message);

    // Cleanup generated key files
    fs::remove_file(priv_key_path).unwrap();
    fs::remove_file(pub_key_path).unwrap();
}

#[test]
fn test_encrypt_not_initialized() {
    let e2e_rsa = E2eRSA2K::new();
    let test_message: &[u8] = b"Test message for RSA encryption";

    // Attempt to encrypt without initialization
    let result = e2e_rsa.encrypt(test_message);

    // Verify that the encryption fails because the instance is not initialized
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
}

#[test]
fn test_decrypt_not_initialized() {
    let test_message: &[u8] = b"Test message for RSA encryption";

    let priv_key_path = "private_key.pem";
    let pub_key_path = "public_key.pem";

    // Generate the keys first
    let mut e2e_rsa = E2eRSA2K::new();
    e2e_rsa.generate_pairkey(Path::new(priv_key_path), Path::new(pub_key_path)).unwrap();
    let init_result = e2e_rsa.init(Path::new(priv_key_path), Path::new(pub_key_path));

    assert!(init_result.is_ok());
    assert_eq!(e2e_rsa.is_initialized(), true);
    let encrypted_message = e2e_rsa.encrypt(test_message).expect("Encryption failed");

    //create new e2e
    let e2e_rsa_new = E2eRSA2K::new();
    // Attempt to decrypt without initialization
    let result = e2e_rsa_new.decrypt(&encrypted_message);

    // Verify that the decryption fails because the instance is not initialized
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);

    // Cleanup generated key files
    fs::remove_file(priv_key_path).unwrap();
    fs::remove_file(pub_key_path).unwrap();
}

// Clean up key files after tests
// #[test]
// fn cleanup() {
//     fs::remove_file(TEST_PRIV_KEY_PATH).ok();
//     fs::remove_file(TEST_PUB_KEY_PATH).ok();
// }