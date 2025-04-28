# Harper C Example

This directory contains a simple C example demonstrating how to use the Harper Rust library from C code.

## Building

1. First, build the Rust library:
```bash
cargo build
```

2. Then compile the C example:
```bash
cc src/florb.c -L target/debug -lharper_c -o florb
```

## Running

Run the example:
```bash
./florb
```

## Example Code

The example (`src/florb.c`) demonstrates basic usage of the Harper library:

1. Creating a document from text
2. Getting the document's text content
3. Getting token information
4. Proper memory management

## API Reference

The C API is defined in `src/harper.h`:

- `harper_version()`: Get the library version number
- `harper_create_document(text)`: Create a new document from text
- `harper_free_document(doc)`: Free a document
- `harper_get_document_text(doc)`: Get the full text of a document
- `harper_get_token_count(doc)`: Get the number of tokens in a document
- `harper_get_token_text(doc, index)`: Get the text of a specific token

## Memory Management

- Documents created with `harper_create_document` must be freed with `harper_free_document`
- Strings returned by `harper_get_document_text` and `harper_get_token_text` must be freed with `free()`

## Error Handling

- Functions that return pointers return `NULL` on error
- Functions that return integers return `-1` on error
- Always check return values for errors 