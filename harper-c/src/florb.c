// florb.c - C program that uses the Rust library

#include <stdio.h> // Standard I/O library for printf
#include <stdlib.h>
#include "harper.h" // Include the header file for the Rust library

int main(int argc, char* argv[]) {
    const char* text = argc > 1 ? argv[1] : "Helloo ,Wrld!";
    
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
    
    // Create a lint group
    LintGroup* lint_group = harper_create_lint_group();
    if (lint_group == NULL) {
        printf("Failed to create lint group\n");
        harper_free_document(doc);
        return 1;
    }
    
    // Get and print lints
    int32_t lint_count;
    Lint** lints = harper_get_lints(doc, lint_group, &lint_count);
    if (lints != NULL) {
        printf("%d lints:\n", lint_count);
        for (int32_t i = 0; i < lint_count; i++) {
            char* message = harper_get_lint_message(lints[i]);
            if (message != NULL) {
                const int32_t start = harper_get_lint_start(lints[i]);
                const int32_t end = harper_get_lint_end(lints[i]);
                printf("Lint %d: '%.*s' : %s\n", i, end - start, text + start, message);
                free(message);
            }
        }
        harper_free_lints(lints, lint_count);
    }
    
    // Clean up
    harper_free_lint_group(lint_group);
    harper_free_document(doc);
    
    return 0;
}
