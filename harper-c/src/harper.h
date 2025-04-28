// harper.h - Header file for the Rust library

#ifndef HARPER_H // Include guard to prevent multiple inclusions
#define HARPER_H

#include <stdint.h>

#ifdef __cplusplus // Check if we're compiling with a C++ compiler
extern "C" { // If so, use C linkage for the following functions
#endif

// Opaque type for the document
typedef struct Document Document;

// Get the version number
int32_t harper_version(void);

// Create a new document from plain English text
// Returns NULL on error
Document* harper_create_document(const char* text);

// Free a document created by harper_create_document
void harper_free_document(Document* doc);

// Get the full text content of the document
// Returns a newly allocated string that must be freed by the caller using free()
// Returns NULL on error
char* harper_get_document_text(const Document* doc);

// Get the number of tokens in the document
// Returns -1 on error
int32_t harper_get_token_count(const Document* doc);

// Get the text of a specific token in the document
// Returns a newly allocated string that must be freed by the caller using free()
// Returns NULL on error
char* harper_get_token_text(const Document* doc, int32_t index);

#ifdef __cplusplus
} // End of extern "C"
#endif

#endif // End of include guard
