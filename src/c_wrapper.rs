use crate::e2e_implementation::{E2eCyber, E2eRSA2K};
use crate::exsecure;
use std::path::Path;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

#[no_mangle]
pub extern "C" fn hello_from_rust() { //testing function
    println!("Hello from Rust!");
}

#[no_mangle]
pub extern "C" fn e2e_new() -> *mut E2eRSA2K {
    Box::into_raw(Box::new(E2eRSA2K::new()))
}

#[no_mangle]
pub extern "C" fn e2e_free(ptr: *mut E2eRSA2K) {
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
pub extern "C" fn e2e_generate_pairkey_withexsecure(
    ptr: *mut E2eRSA2K,
    priv_key_path: *const c_char,
    pub_key_path: *const c_char,
    password: *const c_char
) -> bool {
    let instance = unsafe { &mut *ptr };
    let priv_key_path = unsafe { CStr::from_ptr(priv_key_path).to_str().unwrap() };
    let pub_key_path = unsafe { CStr::from_ptr(pub_key_path).to_str().unwrap() };
    let path_priv = Path::new(priv_key_path);
    let path_pub = Path::new(pub_key_path);
    let password = unsafe { CStr::from_ptr(password).to_str().unwrap() };
    instance.generate_pairkey_withexsecure(path_priv, path_pub, password).is_ok()
}

#[no_mangle]
pub extern "C" fn e2e_init_withexsecure(
    ptr: *mut E2eRSA2K,
    priv_key_path: *const c_char,
    pub_key_path: *const c_char,
    password: *const c_char
) -> bool {
    let instance = unsafe { &mut *ptr };
    let priv_key_path = unsafe { CStr::from_ptr(priv_key_path).to_str().unwrap() };
    let pub_key_path = unsafe { CStr::from_ptr(pub_key_path).to_str().unwrap() };
    let path_priv = Path::new(priv_key_path);
    let path_pub = Path::new(pub_key_path);
    let password = unsafe { CStr::from_ptr(password).to_str().unwrap() };
    instance.init_withexsecure(path_priv, path_pub, password).is_ok()
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
        Err(_) => false, // Return false on failure, TODO: mapping more error code
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
        Err(_) => false, // Return false on failure, TODO: mapping more error code
    }
}

#[no_mangle]
pub extern "C" fn e2e_encrypt_folder(folder_path: *const c_char, password: *const c_char) -> bool {
    let folder_path = unsafe { CStr::from_ptr(folder_path).to_str().unwrap() };
    let password = unsafe { CStr::from_ptr(password).to_str().unwrap() };

    let result = exsecure::encrypt_folder(Path::new(folder_path), password);

    println!("path {:?} result e2e_encrypt_folder {:?}", folder_path, result);

    match result {
        Ok(_) => true,
        Err(_) => false, // Return false on failure, TODO: mapping more error code
    }
}

#[no_mangle]
pub extern "C" fn e2e_decrypt_folder(folder_path: *const c_char, password: *const c_char) -> bool {
    let folder_path = unsafe { CStr::from_ptr(folder_path).to_str().unwrap() };
    let password = unsafe { CStr::from_ptr(password).to_str().unwrap() };

    let result = exsecure::decrypt_folder(Path::new(folder_path), password);

    println!("path {:?} result e2e_decrypt_folder {:?}", folder_path, result);

    match result {
        Ok(_) => true,
        Err(_) => false, // Return false on failure, TODO: mapping more error code
    }
}

#[no_mangle]
pub extern "C" fn e2e_decrypt_file(file_path: *const c_char, password: *const c_char, outbuf: *mut c_char) -> i64 {
    let file_path = unsafe { CStr::from_ptr(file_path).to_str().unwrap() };
    let password = unsafe { CStr::from_ptr(password).to_str().unwrap() };
    let result = exsecure::decrypt_file(Path::new(file_path), password);
    println!("path {:?} result e2e_decrypt_file {:?}", file_path, result);
    match result {
        Ok(decrypted_data) => {
            // Convert decrypted data to a CString
            let c_string = CString::new(decrypted_data).expect("CString::new failed");

            // Copy the decrypted data to the provided buffer
            unsafe {
                let outbuf_len = c_string.as_bytes().len();
                std::ptr::copy_nonoverlapping(c_string.as_ptr(), outbuf as *mut c_char, outbuf_len);
                //outbuf.add(outbuf_len);
                outbuf_len as i64
            }
        },
        Err(_) => {
            -1 // Return -1 on failure, TODO: mapping more error code
        },
    }
}

#[no_mangle]
pub extern "C" fn e2e_free_data(data: *mut c_void) {
    if data.is_null() { return; }
    unsafe {
        let _ = Box::from_raw(data as *mut Vec<u8>);
    }
}