// ffi.rs - Foreign Function Interface for Rust to C

// Importing the necessary types from the standard library
use std::os::raw::{c_int, c_char}; // This allows us to use C-compatible types
use std::ffi::{CStr, CString}; // For handling C-compatible strings
use std::ptr;

// Import something extremely basic and simple from Harper
use harper_core::Document;

// The `no_mangle` attribute prevents Rust from changing the name of this function,
// making it easier for C to link to it.
/// Creates a new document from plain English text.
/// Returns a pointer to the document, or null if there was an error.
/// The caller is responsible for freeing the document using harper_free_document.
#[no_mangle]
pub extern "C" fn harper_create_document(text: *const c_char) -> *mut Document {
    if text.is_null() {
        return ptr::null_mut();
    }

    // Convert C string to Rust string
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    // Create the document
    let doc = Document::new_plain_english_curated(text_str);
    
    // Box the document and leak it to get a raw pointer
    Box::into_raw(Box::new(doc))
}

/// Frees a document created by harper_create_document.
#[no_mangle]
pub extern "C" fn harper_free_document(doc: *mut Document) {
    if !doc.is_null() {
        unsafe {
            // Convert the raw pointer back to a Box and let it drop
            let _ = Box::from_raw(doc);
        }
    }
}

/// Gets the full text content of the document.
/// Returns a newly allocated C string that must be freed by the caller using free().
/// Returns NULL if the document is NULL or if memory allocation fails.
#[no_mangle]
pub extern "C" fn harper_get_document_text(doc: *const Document) -> *mut c_char {
    if doc.is_null() {
        return ptr::null_mut();
    }

    let doc = unsafe { &*doc };
    let text = doc.get_full_string();
    
    match CString::new(text) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Gets the number of tokens in the document.
/// Returns -1 if the document is NULL.
#[no_mangle]
pub extern "C" fn harper_get_token_count(doc: *const Document) -> c_int {
    if doc.is_null() {
        return -1;
    }

    let doc = unsafe { &*doc };
    doc.get_tokens().len() as c_int
}

/// Gets the text of a specific token in the document.
/// Returns a newly allocated C string that must be freed by the caller using free().
/// Returns NULL if the document is NULL, the index is out of bounds, or if memory allocation fails.
#[no_mangle]
pub extern "C" fn harper_get_token_text(doc: *const Document, index: c_int) -> *mut c_char {
    if doc.is_null() || index < 0 {
        return ptr::null_mut();
    }

    let doc = unsafe { &*doc };
    let tokens = doc.get_tokens();
    
    if index as usize >= tokens.len() {
        return ptr::null_mut();
    }

    let token = &tokens[index as usize];
    let text = doc.get_span_content_str(&token.span);
    
    match CString::new(text) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}
