// florb.c - C program that uses the Rust library

#include <stdio.h> // Standard I/O library for printf
#include <stdlib.h>
#include "harper.h" // Include the header file for the Rust library

int main() {
    const char* text = "Hello, world!";

    // Create a document
    Document* doc = harper_create_document(text);
    if (doc == NULL) {
        printf("Failed to create document\n");
        return 1;
    }

    // Get and print document text
    char* doc_text = harper_get_document_text(doc);
    if (doc_text != NULL) {
        printf("Document text: %s\n", doc_text);
        free(doc_text);
    }

    // Get and print token count
    int32_t token_count = harper_get_token_count(doc);
    printf("Token count: %d\n", token_count);

    // Print each token
    for (int32_t i = 0; i < token_count; i++) {
        char* token_text = harper_get_token_text(doc, i);
        if (token_text != NULL) {
            printf("Token %d: %s\n", i, token_text);
            free(token_text);
        }
    }

    // Free the document
    harper_free_document(doc);

    return 0;
}
