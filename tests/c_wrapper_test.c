#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "e2esdk.h"

int test_e2e() {
    E2eRSA2K* instance = e2e_new();
    if (!instance) {
        printf("Failed to create E2eRSA2K instance\n");
        return -1;
    }

    const char* priv_key_path = "private.pem";
    const char* pub_key_path = "public.pem";

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

    const char* message = "C wrapper test is passed";
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

int main() {
    //hello_from_rust();
    if (test_e2e()) {
        return -1;
    }

    return 0;
}
