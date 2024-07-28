use super::E2eRSA2K;
use crate::e2e_implementation::E2eCyber;
use std::path::Path;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn e2e_create_instance() -> *mut E2eRSA2K {
    Box::into_raw(Box::new(E2eRSA2K::new()))
}

#[no_mangle]
pub extern "C" fn e2e_destroy_instance(ptr: *mut E2eRSA2K) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(ptr)); // drop the Box
        }
    }
}

#[no_mangle]
pub extern "C" fn e2e_generate_pairkey(
    ptr: *mut E2eRSA2K,
    priv_key_path: *const c_char,
    pub_key_path: *const c_char
) -> bool {
    let instance = unsafe { &mut *ptr };
    let priv_key_path = unsafe { CStr::from_ptr(priv_key_path).to_str().unwrap() };
    let pub_key_path = unsafe { CStr::from_ptr(pub_key_path).to_str().unwrap() };
    let path_priv = Path::new(priv_key_path);
    let path_pub = Path::new(pub_key_path);
    instance.generate_pairkey(path_priv, path_pub).is_ok()
}

#[no_mangle]
pub extern "C" fn e2e_init(
    ptr: *mut E2eRSA2K,
    priv_key_path: *const c_char,
    pub_key_path: *const c_char
) -> bool {
    let instance = unsafe { &mut *ptr };
    let priv_key_path = unsafe { CStr::from_ptr(priv_key_path).to_str().unwrap() };
    let pub_key_path = unsafe { CStr::from_ptr(pub_key_path).to_str().unwrap() };
    let path_priv = Path::new(priv_key_path);
    let path_pub = Path::new(pub_key_path);
    instance.init(path_priv, path_pub).is_ok()
}

#[no_mangle]
pub extern "C" fn e2e_encrypt(
    ptr: *const E2eRSA2K,
    message: *const u8,
    message_len: usize,
    encrypted_message: *mut u8,
    encrypted_message_len: *mut usize
) -> bool {
    let instance = unsafe { &*ptr };
    let message_slice = unsafe { std::slice::from_raw_parts(message, message_len) };
    match instance.encrypt(message_slice) {
        Ok(encrypted) => {
            unsafe {
                std::ptr::copy_nonoverlapping(encrypted.as_ptr(), encrypted_message, encrypted.len());
                *encrypted_message_len = encrypted.len();
            }
            true
        }
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn e2e_decrypt(
    ptr: *const E2eRSA2K,
    encrypted_message: *const u8,
    encrypted_message_len: usize,
    decrypted_message: *mut u8,
    decrypted_message_len: *mut usize
) -> bool {
    let instance = unsafe { &*ptr };
    let encrypted_message_slice = unsafe { std::slice::from_raw_parts(encrypted_message, encrypted_message_len) };
    match instance.decrypt(encrypted_message_slice) {
        Ok(dec_msg) => {
            unsafe {
                std::ptr::copy_nonoverlapping(dec_msg.as_ptr(), decrypted_message, dec_msg.len());
                *decrypted_message_len = dec_msg.len();
            }
            true
        },
        Err(_) => false,
    }
}