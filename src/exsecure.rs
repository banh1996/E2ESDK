use std::io::{self, Error, ErrorKind, Read, Write};
use std::fs::{self, File};
use std::path::Path;

use ctr::cipher::{KeyIvInit, StreamCipher};
use aes::cipher::generic_array::GenericArray;
use sha2::{Sha256, Digest};
use rand::Rng;

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

// Function to encrypt data with AES-128 CTR mode, adding 4 random bytes at the end
fn encrypt_aes128_ctr(data_to_encrypt: &[u8], iv_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, io::Error> {
    if iv_bytes.len() != 16 {
        return Err(Error::new(ErrorKind::InvalidInput, "wrong IV length"));
    }
    if key.len() != 16 {
        return Err(Error::new(ErrorKind::InvalidInput, "wrong key length"));
    }

    // Generate 4 random bytes
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 4];
    rng.fill(&mut random_bytes);

    // Append the random bytes to the end of the data
    let mut data_with_random = Vec::from(data_to_encrypt);
    data_with_random.extend_from_slice(&random_bytes);

    // Perform AES-128 CTR encryption
    let mut encrypted_data = data_with_random.clone();
    let mut cipher = Aes128Ctr64LE::new(GenericArray::from_slice(key), GenericArray::from_slice(iv_bytes));
    cipher.apply_keystream(&mut encrypted_data);

    Ok(encrypted_data)
}

// Function to decrypt data with AES-128 CTR mode, removing the last 4 bytes
fn decrypt_aes128_ctr(encrypted_data: &[u8], iv_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, io::Error> {
    if iv_bytes.len() != 16 {
        return Err(Error::new(ErrorKind::InvalidInput, "wrong IV length"));
    }
    if key.len() != 16 {
        return Err(Error::new(ErrorKind::InvalidInput, "wrong key length"));
    }

    // Perform AES-128 CTR decryption
    let mut decrypted_data = encrypted_data.to_vec();
    let mut cipher = Aes128Ctr64LE::new(GenericArray::from_slice(key), GenericArray::from_slice(iv_bytes));
    cipher.apply_keystream(&mut decrypted_data);

    // Remove the last 4 bytes (which are the random bytes)
    if decrypted_data.len() < 4 {
        return Err(Error::new(ErrorKind::InvalidInput, "Data too short"));
    }
    decrypted_data.truncate(decrypted_data.len() - 4);

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

//this function will encrypt file and overwrite the existed file
pub fn encrypt_file(file_path: &Path, password: &str) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let key = hash_aes_key(password);
    encrypt_file_and_save(&file_path, &key)?;
    Ok(())
}

//this function will decrypt file and return the plaintext, not overwrite existed file
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