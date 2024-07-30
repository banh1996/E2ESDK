#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "e2esdk.h"

#define BUFFER_SIZE 512

//test 0
static int test_e2e_basicfunc(const char *out_dir) {
    E2eRSA2K* instance = e2e_new();
    if (!instance) {
        printf("Failed to create E2eRSA2K instance\n");
        return -1;
    }

    char priv_key_path[BUFFER_SIZE];
    char pub_key_path[BUFFER_SIZE];
    memset(priv_key_path, 0, BUFFER_SIZE);
    memset(pub_key_path, 0, BUFFER_SIZE);

    // Create the full path string
    snprintf(priv_key_path, sizeof(priv_key_path), "%s/private.pem", out_dir);
    snprintf(pub_key_path, sizeof(pub_key_path), "%s/public.pem", out_dir);

    if (!e2e_generate_pairkey(instance, priv_key_path, pub_key_path)) {
        printf("Failed to generate key pair\n");
        e2e_free(instance);
        return -1;
    }

    if (!e2e_init(instance, priv_key_path, pub_key_path)) {
        printf("Failed to initialize with key files\n");
        e2e_free(instance);
        remove(priv_key_path);
        remove(pub_key_path);
        return -1;
    }

    const char* message = "C wrapper test_e2e_basicfunc is passed";
    unsigned char encrypted_message[256];
    size_t encrypted_message_len = 256;

    if (!e2e_encrypt(instance, (const unsigned char*)message, strlen(message), encrypted_message, &encrypted_message_len)) {
        printf("Failed to encrypt message\n");
        e2e_free(instance);
        remove(priv_key_path);
        remove(pub_key_path);
        return -1;
    }

    unsigned char decrypted_message[256];
    size_t decrypted_message_len = 256;

    if (!e2e_decrypt(instance, encrypted_message, encrypted_message_len, decrypted_message, &decrypted_message_len)) {
        printf("Failed to decrypt message\n");
        e2e_free(instance);
        remove(priv_key_path);
        remove(pub_key_path);
        return -1;
    }

    decrypted_message[decrypted_message_len] = '\0';
    if (memcmp(decrypted_message, message, strlen(message)) == 0) {
        printf("Decrypted message successfully: %s\n", decrypted_message);
        e2e_free(instance);
        remove(priv_key_path);
        remove(pub_key_path);
        return 0;
    }

    e2e_free(instance);
    remove(priv_key_path);
    remove(pub_key_path);
    return -1;
}

//test 1
static int test_e2e_exsecure(const char *out_dir) {
    //const char* folder_path = "test_folder";
    const char* password = "testpassword";
    char folder_path[BUFFER_SIZE];
    memset(folder_path, 0, BUFFER_SIZE);

    // Create the full path string
    snprintf(folder_path, sizeof(folder_path), "%s/test_folder", out_dir);

    printf("encrypt folder %s\n", folder_path);

    // Encrypt the folder
    if (e2e_encrypt_folder(folder_path, password) == false) {
        printf("Failed to encrypt folder\n");
        return -1;
    }

    // Decrypt the folder
    if (e2e_decrypt_folder(folder_path, password) == false) {
        printf("Failed to decrypt folder\n");
        return -1;
    }

    //test decrypt file
    // firstly, encrypt all files in folder again
    if (e2e_encrypt_folder(folder_path, password) == false) {
        printf("Failed to encrypt folder\n");
        return -1;
    }
    // Decrypt a file and print content
    char file_path[BUFFER_SIZE];
    char decrypted_data[BUFFER_SIZE];
    memset(folder_path, 0, BUFFER_SIZE);
    memset(decrypted_data, 0, BUFFER_SIZE);
    snprintf(file_path, sizeof(file_path), "%s/test_folder/test1.txt", out_dir);

    if (e2e_decrypt_file(file_path, password, decrypted_data) == -1) {
        printf("Failed to decrypt file\n");
        return -1;
    }

    // Print decrypted content
    printf("Decrypted data file successfully: %s\n", (char*)decrypted_data);

    return 0;
}

int main(int argc, char *argv[]) {
    if (argc != 3) {
        fprintf(stderr, "Usage: %s <argument>\n", argv[0]);
        return -1;
    }

    //get out_dir
    char *out_dir = argv[1];

    // Convert the argument to an integer
    int number = atoi(argv[2]);

    //hello_from_rust();

    if (number == 0) {
        if (test_e2e_basicfunc(out_dir)) {
            printf("test_e2e_basicfunc failed\n");
            return -1;
        }
    } else if (number == 1) {
        if (test_e2e_exsecure(out_dir)) {
            printf("test_e2e_exsecure failed\n");
            return -1;
        }
    } else {
        printf("argument not expected\n");
        return -1;
    }

    return 0;
}
