// ffi.rs - Foreign Function Interface for Rust to C

// Importing the necessary types from the standard library
use std::os::raw::{c_int, c_char}; // This allows us to use C-compatible types
use std::ffi::{CStr, CString}; // For handling C-compatible strings
use std::ptr;
use std::sync::Arc;

// Import something extremely basic and simple from Harper
use harper_core::{
    Document,
    linting::{Lint, LintGroup, Linter},
    spell::FstDictionary,
};

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

/// Creates a new lint group with curated rules.
/// Returns a pointer to the lint group, or null if there was an error.
/// The caller is responsible for freeing the lint group using harper_free_lint_group.
#[no_mangle]
pub extern "C" fn harper_create_lint_group() -> *mut LintGroup {
    let dictionary = FstDictionary::curated();
    let lint_group = LintGroup::new_curated(Arc::new(dictionary), harper_core::Dialect::American);
    Box::into_raw(Box::new(lint_group))
}

/// Frees a lint group created by harper_create_lint_group.
#[no_mangle]
pub extern "C" fn harper_free_lint_group(lint_group: *mut LintGroup) {
    if !lint_group.is_null() {
        unsafe {
            let _ = Box::from_raw(lint_group);
        }
    }
}

/// Gets all lints for a document using a lint group.
/// Returns a pointer to an array of Lint pointers, and sets count to the number of lints.
/// The caller is responsible for freeing both the array and each Lint using harper_free_lints.
/// Returns NULL if any pointer is NULL or if memory allocation fails.
#[no_mangle]
pub extern "C" fn harper_get_lints(
    doc: *const Document,
    lint_group: *mut LintGroup,
    count: *mut c_int,
) -> *mut *mut Lint {
    if doc.is_null() || lint_group.is_null() || count.is_null() {
        return ptr::null_mut();
    }

    let doc = unsafe { &*doc };
    let lint_group = unsafe { &mut *lint_group };
    
    let lints = lint_group.lint(doc);
    
    // Convert Vec<Lint> to Vec<Box<Lint>>
    let boxed_lints: Vec<Box<Lint>> = lints.into_iter().map(Box::new).collect();
    
    // Convert to raw pointers
    let mut raw_lints: Vec<*mut Lint> = boxed_lints.into_iter().map(Box::into_raw).collect();
    
    // Set the count
    unsafe {
        *count = raw_lints.len() as c_int;
    }
    
    // Return the array
    let result = raw_lints.as_mut_ptr();
    std::mem::forget(raw_lints); // Prevent deallocation
    result
}

/// Frees an array of lints created by harper_get_lints.
#[no_mangle]
pub extern "C" fn harper_free_lints(lints: *mut *mut Lint, count: c_int) {
    if lints.is_null() || count <= 0 {
        return;
    }

    unsafe {
        // Convert back to Vec
        let lints_vec = Vec::from_raw_parts(lints, count as usize, count as usize);
        
        // Free each lint
        for lint in lints_vec {
            if !lint.is_null() {
                let _ = Box::from_raw(lint);
            }
        }
    }
}

/// Gets the message for a lint.
/// Returns a newly allocated string that must be freed by the caller using free().
/// Returns NULL if the lint is NULL or if memory allocation fails.
#[no_mangle]
pub extern "C" fn harper_get_lint_message(lint: *const Lint) -> *mut c_char {
    if lint.is_null() {
        return ptr::null_mut();
    }

    let lint = unsafe { &*lint };
    let message = lint.message.to_string();
    
    match CString::new(message) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Gets the start position of a lint in the document.
/// Returns -1 if the lint is NULL.
#[no_mangle]
pub extern "C" fn harper_get_lint_start(lint: *const Lint) -> c_int {
    if lint.is_null() {
        return -1;
    }

    let lint = unsafe { &*lint };
    lint.span.start as c_int
}

/// Gets the end position of a lint in the document.
/// Returns -1 if the lint is NULL.
#[no_mangle]
pub extern "C" fn harper_get_lint_end(lint: *const Lint) -> c_int {
    if lint.is_null() {
        return -1;
    }

    let lint = unsafe { &*lint };
    lint.span.end as c_int
}
