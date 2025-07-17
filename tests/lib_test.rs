use e2esdk::{self, E2eCyber, E2eRSA2K};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempdir::TempDir;

struct KeyFileCleaner<'a> {
    priv_key_path: &'a str,
    pub_key_path: &'a str,
}

impl<'a> Drop for KeyFileCleaner<'a> {
    fn drop(&mut self) {
        fs::remove_file(self.priv_key_path).unwrap_or_else(|_| {
            println!("Not found private key file to remove: {}", self.priv_key_path);
        });
        fs::remove_file(self.pub_key_path).unwrap_or_else(|_| {
            println!("Not found public key file to remove: {}", self.pub_key_path);
        });
    }
}

#[test]
fn test_generate_pairkey() {
    let priv_key_path = "private_key.pem";
    let pub_key_path = "public_key.pem";
    let _cleaner = KeyFileCleaner { priv_key_path, pub_key_path };

    let mut e2e_rsa = E2eRSA2K::new();
    e2e_rsa.generate_pairkey(Path::new(priv_key_path), Path::new(pub_key_path)).unwrap();

    assert!(Path::new(priv_key_path).exists());
    assert!(Path::new(pub_key_path).exists());
}

#[test]
fn test_init_with_generated_keys() {
    let priv_key_path = "private_key.pem";
    let pub_key_path = "public_key.pem";
    let _cleaner = KeyFileCleaner { priv_key_path, pub_key_path };

    // Generate the keys first
    let mut e2e_rsa = E2eRSA2K::new();
    e2e_rsa.generate_pairkey(Path::new(priv_key_path), Path::new(pub_key_path)).unwrap();

    // Create a new instance and initialize it with the generated keys
    let mut e2e_rsa_initialized = E2eRSA2K::new();
    let init_result = e2e_rsa_initialized.init(Path::new(priv_key_path), Path::new(pub_key_path));

    assert!(init_result.is_ok());
    assert_eq!(e2e_rsa_initialized.is_initialized(), true);
}

#[test]
fn test_init_with_invalid_keys() {
    let priv_key_path = "invalid_private_key.pem";
    let pub_key_path = "invalid_public_key.pem";
    let _cleaner = KeyFileCleaner { priv_key_path, pub_key_path };

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
}

#[test]
fn test_encrypt_decrypt() {
    let test_message: &[u8] = b"Test message for RSA encryption";
    let priv_key_path = "private_key.pem";
    let pub_key_path = "public_key.pem";
    let _cleaner = KeyFileCleaner { priv_key_path, pub_key_path };

    let _cleaner = KeyFileCleaner { priv_key_path, pub_key_path };

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
    let _cleaner = KeyFileCleaner { priv_key_path, pub_key_path };

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
}


/*****************************************************************************************************************
 * Start testing for ex-secure
 *****************************************************************************************************************/
const PASSWORD: &str = "12345678aA@";

use std::io::{BufReader, Read};
use std::error::Error;

fn calculate_entropy(path: &Path) -> Result<f64, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    if buffer.is_empty() {
        return Ok(0.0);
    }

    let mut counts = [0usize; 256];
    for &byte in &buffer {
        counts[byte as usize] += 1;
    }

    let total = buffer.len() as f64;
    let mut entropy = 0.0;

    for &count in &counts {
        if count > 0 {
            let p = count as f64 / total;
            entropy -= p * p.log2();
        }
    }

    Ok(entropy)
}

/// Recursively calculate entropy for all files in a folder
fn calculate_entropy_for_folder<P: AsRef<Path>>(
    folder: P,
) -> Result<Vec<(PathBuf, f64)>, Box<dyn Error>> {
    let mut results = Vec::new();

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            match calculate_entropy(&path) {
                Ok(entropy) => results.push((path, entropy)),
                Err(err) => eprintln!("Failed to process {}: {}", path.display(), err),
            }
        } else if path.is_dir() {
            let nested = calculate_entropy_for_folder(&path)?;
            results.extend(nested);
        }
    }

    Ok(results)
}

fn create_test_file(dir: &TempDir, name: &str, content: &[u8]) -> PathBuf {
    let file_path = dir.path().join(name);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content).unwrap();
    file_path
}

// #[test]
// fn test_cal_entroy_folder() {
//     let raw_path = "filesprocessing";
//     println!("Entropy Before encrypting:");
//     match calculate_entropy_for_folder(raw_path) {
//         Ok(results) => {
//             for (raw_path, entropy) in results {
//                 println!("{:.4} bits - {}", entropy, raw_path.display());
//             }
//         },
//         _ => {

//         }
//     }
//     let files_path = Path::new(raw_path);
//     e2esdk::encrypt_folder(files_path, PASSWORD).unwrap();
//     println!("Entropy After encrypting:");
//     match calculate_entropy_for_folder(raw_path) {
//         Ok(results) => {
//             for (raw_path, entropy) in results {
//                 println!("{:.4} bits - {}", entropy, raw_path.display());
//             }
//         },
//         _ => {

//         }
//     }
// }


#[test]
fn test_encrypt_folder() {
    let dir = TempDir::new("test_encrypt_folder").unwrap();
    let file_path = create_test_file(&dir, "test.txt", b"test_encrypt_folder Hello world!");

    e2esdk::encrypt_folder(dir.path(), PASSWORD).unwrap();

    let encrypted_content = fs::read(&file_path).unwrap();
    assert_ne!(encrypted_content, b" test_encrypt_folderHello world!");
}

#[test]
fn test_decrypt_folder() {
    let dir = TempDir::new("test_decrypt_folder").unwrap();
    let file_path = create_test_file(&dir, "test.txt", b"test_decrypt_folder Hello world!");

    e2esdk::encrypt_folder(dir.path(), PASSWORD).unwrap();
    e2esdk::decrypt_folder(dir.path(), PASSWORD).unwrap();

    let decrypted_content = fs::read(&file_path).unwrap();
    assert_eq!(decrypted_content, b"test_decrypt_folder Hello world!");
}

#[test]
fn test_decrypt_file() {
    let dir = TempDir::new("test_decrypt_file").unwrap();
    let file_path = create_test_file(&dir, "test.txt", b"test_decrypt_file Hello world!");

    e2esdk::encrypt_folder(dir.path(), PASSWORD).unwrap();

    let decrypted_content = e2esdk::decrypt_file(&file_path, PASSWORD).unwrap();
    assert_eq!(decrypted_content, b"test_decrypt_file Hello world!");
}
/*****************************************************************************************************************
 * End testing for ex-secure
 *****************************************************************************************************************/