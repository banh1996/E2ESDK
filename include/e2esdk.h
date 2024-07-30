#ifndef E2ESDK_H
#define E2ESDK_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdbool.h>

typedef struct E2eRSA2K E2eRSA2K;

void hello_from_rust(); //just for me to test C ABI working :)

// Create a new E2eRSA2K instance
E2eRSA2K* e2e_new();

// Free the E2eRSA2K instance
void e2e_free(E2eRSA2K* ptr);

// Initialize the E2eRSA2K instance
bool e2e_init(E2eRSA2K* ptr, const char* priv_key_path, const char* pub_key_path);

// Generate a key pair
bool e2e_generate_pairkey(E2eRSA2K* ptr, const char* priv_key_path, const char* pub_key_path);

// Encrypt a message
bool e2e_encrypt(E2eRSA2K* ptr, const unsigned char* message, size_t message_len, unsigned char* encrypted_message, size_t* encrypted_message_len);

// Decrypt a message
bool e2e_decrypt(E2eRSA2K* ptr, const unsigned char* encrypted_message, size_t encrypted_message_len, unsigned char* decrypted_message, size_t* decrypted_message_len);


/*************** Start Secure folder API **********************/
bool e2e_encrypt_folder(const char* folder_path, const char* password);
bool e2e_decrypt_folder(const char* folder_path, const char* password);
size_t e2e_decrypt_file(const char* file_path, const char* password, char *outbuf);
void e2e_free_data(void* data);
/*************** End Secure folder API **********************/

#ifdef __cplusplus
}
#endif

#endif // E2ESDK_H
