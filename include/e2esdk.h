#ifndef E2ESDK_H
#define E2ESDK_H

#ifdef __cplusplus
extern "C" {
#endif

// Create a new E2eRSA2K instance
void* e2e_new();

// Free the E2eRSA2K instance
void e2e_free(void* ptr);

// Initialize the E2eRSA2K instance
int e2e_init(void* ptr, const char* priv_key_path, const char* pub_key_path);

// Generate a key pair
int e2e_generate_pairkey(void* ptr, const char* priv_key_path, const char* pub_key_path);

// Encrypt a message
int e2e_encrypt(void* ptr, const unsigned char* message, size_t message_len, unsigned char* encrypted_message, size_t* encrypted_message_len);

// Decrypt a message
int e2e_decrypt(void* ptr, const unsigned char* encrypted_message, size_t encrypted_message_len, unsigned char* decrypted_message, size_t* decrypted_message_len);

#ifdef __cplusplus
}
#endif

#endif // E2E_SDK_H
