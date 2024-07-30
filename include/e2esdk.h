#ifndef E2ESDK_H
#define E2ESDK_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdbool.h>

typedef struct E2eRSA2K E2eRSA2K;

void hello_from_rust(); //just for me to test C ABI working :)

/*****************************************************************************************************************
 *  e2e_new function
 *  brief        Create a new E2eRSA2K instance
 *  details      -
 *  \param[in]   ptr: point to E2eRSA2K ptr
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  TRUE
 *  \return      Error code if any
 ****************************************************************************************************************/
E2eRSA2K* e2e_new();

/*****************************************************************************************************************
 *  e2e_free function
 *  brief        Free your E2eRSA2K object that created
 *  details      -
 *  \param[in]   ptr: point to E2eRSA2K ptr
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  TRUE
 *  \return      Error code if any
 ****************************************************************************************************************/
void e2e_free(E2eRSA2K* ptr);

/*****************************************************************************************************************
 *  init function
 *  brief        Init necessary configuration for SDK
 *  details      If you already owned private/public key, free to call this function
 *               Otherwise, call generate_pairkey function to generate pair-key firstly
 *  \param[in]   priv_key_path: path to existed private key, expect PEM format
 *  \param[in]   pub_key_path: path to existed public key, expect PEM format
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return      Error code if any
 ****************************************************************************************************************/
bool e2e_init(E2eRSA2K* ptr, const char* priv_key_path, const char* pub_key_path);

/*****************************************************************************************************************
 *  generate_pairkey function
 *  brief        Generate private/public key to input paths
 *  details      Call this function if you do not have a pair key for setting
 *  \param[in]   priv_key_path: path to private key need to be stored, expect PEM format
 *  \param[in]   pub_key_path: path to public key need to be stored, expect PEM format
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return      Error code if any
 ****************************************************************************************************************/
bool e2e_generate_pairkey(E2eRSA2K* ptr, const char* priv_key_path, const char* pub_key_path);

/*****************************************************************************************************************
 *  init_withexsecure function
 *  brief        Init necessary configuration for SDK with secured secret keys
 *  details      If you already owned private/public key, free to call this function
 *               Otherwise, call generate_pairkey_withexsecure function to generate secured pair-key firstly
 *  \param[in]   priv_key_path: path to secured private key, expect PEM format
 *  \param[in]   pub_key_path: path to secured public key, expect PEM format
 *  \param[in]   password: password to secure secret key
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return      Error code if any
 ****************************************************************************************************************/
bool e2e_init_withexsecure(E2eRSA2K* ptr, const char* priv_key_path, const char* pub_key_path, const char* password);

/*****************************************************************************************************************
 *  generate_pairkey_withexsecure function
 *  brief        Generate private/public key to input paths with secured secret keys
 *  details      Call this function if you do not have a pair key for setting
 *  \param[in]   priv_key_path: path to private key need to be stored, expect PEM format
 *  \param[in]   pub_key_path: path to public key need to be stored, expect PEM format
 *  \param[in]   password: password to secure secret key
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return      Error code if any
 ****************************************************************************************************************/
bool e2e_generate_pairkey_withexsecure(E2eRSA2K* ptr, const char* priv_key_path, const char* pub_key_path, const char* password);

/*****************************************************************************************************************
 *  encrypt function
 *  brief        Encrypt message with provisioned public-key
 *  details      Call this function to encrypt message
 *  \param[in]   message: message to encrypt
 *  \param[out]  -
 *  \precondition: Call e2e_init or e2e_init_withexsecure before using this function
 *  \reentrant:  FALSE
 *  \return      Encrypted array
 *               Error code if any
 ****************************************************************************************************************/
bool e2e_encrypt(E2eRSA2K* ptr, const unsigned char* message, size_t message_len, unsigned char* encrypted_message, size_t* encrypted_message_len);

/*****************************************************************************************************************
 *  decrypt function
 *  brief        Decrypt message with provisioned private-key
 *  details      Call this function to decrypt message
 *  \param[in]   message: message to decrypt
 *  \param[out]  -
 *  \precondition: Call e2e_init or e2e_init_withexsecure before using this function
 *  \reentrant:  FALSE
 *  \return      Decrypted array
 *               Error code if any
 ****************************************************************************************************************/
bool e2e_decrypt(E2eRSA2K* ptr, const unsigned char* encrypted_message, size_t encrypted_message_len, unsigned char* decrypted_message, size_t* decrypted_message_len);

/*************** Start Secure folder API *************************************************************
 * List all API to protect you secret folder (contain secret info such as keys, cert)
 * The developer's too lazy for writing more detail :( 
 *****************************************************************************************************/
bool e2e_encrypt_folder(const char* folder_path, const char* password);
bool e2e_decrypt_folder(const char* folder_path, const char* password);
size_t e2e_decrypt_file(const char* file_path, const char* password, char *outbuf);
void e2e_free_data(void* data);
/*************** End Secure folder API ***************************************************************/

#ifdef __cplusplus
}
#endif

#endif // E2ESDK_H
