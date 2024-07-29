use std::io::{self, Error, ErrorKind, Read, Write};
use std::fs::{self, File};
use std::path::Path;

use ctr::cipher::{KeyIvInit, StreamCipher};
use aes::cipher::generic_array::GenericArray;
use sha2::{Sha256, Digest};

type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;

const IV: &[u8] = b"tranngochung1996"; // Initialization vector - 16 bytes

// Hash the password to create a 128-bit key
fn hash_aes_key(password: &str) -> [u8; 16] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 16];
    key.copy_from_slice(&result[0..16]);
    key
}

fn encrypt_aes128_ctr(data_to_encrypt: &[u8], iv_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, io::Error> {
    if iv_bytes.len() != 16 {
        return Err(Error::new(ErrorKind::InvalidInput, "wrong key length"));
    }

    let mut encrypted_data = data_to_encrypt.to_vec();
    let mut cipher = Aes128Ctr64LE::new(GenericArray::from_slice(&key), GenericArray::from_slice(&iv_bytes));
    cipher.apply_keystream(&mut encrypted_data);

    Ok(encrypted_data)
}

fn decrypt_aes128_ctr(encrypted_data: &[u8], iv_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, io::Error> {
    if iv_bytes.len() != 16 {
        return Err(Error::new(ErrorKind::InvalidInput, "wrong key length"));
    }

    let mut decrypted_data = encrypted_data.to_vec();
    let mut cipher = Aes128Ctr64LE::new(GenericArray::from_slice(&key), GenericArray::from_slice(&iv_bytes));
    cipher.apply_keystream(&mut decrypted_data);

    Ok(decrypted_data)
}

// Encrypt a file and overwrite it with encrypted data
fn encrypt_file_and_save(file_path: &Path, key: &[u8]) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let ciphertext = match encrypt_aes128_ctr(&buffer, &IV, key) {
        Ok(encrypted_data_record) => {encrypted_data_record}
        Err(err) => {
            eprintln!("Encryption error: {}", err);
            return Err(err);
        }
    };

    let mut encrypted_file = File::create(file_path)?;
    encrypted_file.write_all(&ciphertext)?;
    Ok(())
}

// Decrypt a file and overwrite it with plaintext data
fn decrypt_file_and_save(file_path: &Path, key: &[u8]) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let ciphertext = match decrypt_aes128_ctr(&buffer, &IV, key) {
        Ok(decrypted_data_record) => {decrypted_data_record}
        Err(err) => {
            eprintln!("Encryption error: {}", err);
            return Err(err);
        }
    };

    let mut decrypted_file = File::create(file_path)?;
    decrypted_file.write_all(&ciphertext)?;
    Ok(())
}


/*****************************************************************************************************************
 * Start public interface for securing folder
 *****************************************************************************************************************/
pub fn encrypt_folder(folder_path: &Path, password: &str) -> io::Result<()> {
    let key = hash_aes_key(password);
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            encrypt_file_and_save(&path, &key)?;
        }
    }
    Ok(())
}

pub fn decrypt_folder(folder_path: &Path, password: &str) -> io::Result<()> {
    let key = hash_aes_key(password);
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            decrypt_file_and_save(&path, &key)?;
        }
    }
    Ok(())
}

pub fn decrypt_file(file_path: &Path, password: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let key = hash_aes_key(password);
    match decrypt_aes128_ctr(&buffer, &IV, &key) {
        Ok(decrypted_data_record) => Ok(decrypted_data_record),
        Err(err) => {
            eprintln!("Encryption error: {}", err);
            return Err(err);
        }
    }
}
/*****************************************************************************************************************
 * End public interface for securing folder
 *****************************************************************************************************************/