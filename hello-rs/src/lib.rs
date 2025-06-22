use anyhow::{Context, Result};
use std::ffi::{CStr, CString};

pub fn hello(name: Option<&str>) -> Result<String> {
    // Convert the Rust &str to a C-compatible null-terminated string
    // If `name` is None, default to "World". Otherwise, use the provided name.
    let c_name = CString::new(name.unwrap_or("World"))
        .context("CString::new failed due to interior null byte on parameter 'name'")?;

    // Call the FFI function. The `unsafe` block is necessary because we are
    // dereferencing a raw pointer returned from C, which Rust cannot verify for safety.
    unsafe {
        // `hello_sys::hello` returns a raw pointer `*const c_char`.
        let result_ptr = hello_sys::hello(c_name.as_ptr());

        // We wrap this raw pointer in a `CStr`. This is safe if the C function
        // guarantees a valid, null-terminated string for the lifetime of the call.
        // Our C function uses a static buffer, so this is okay, but be wary of
        // C functions that return pointers to memory you must free.
        let c_str = CStr::from_ptr(result_ptr);

        // Convert the C-style string into a Rust `String`.
        // `.to_string_lossy()` is a robust way to handle potential non-UTF-8 data.
        Ok(c_str.to_string_lossy().into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn say_hello() {
        let msg = hello(Some("Rust"));

        // Assert that the call was successful
        assert!(msg.is_ok());

        // Assert that the output is what we expect
        assert_eq!(msg.unwrap(), "Hello, Rust\n");
    }

    #[test]
    fn say_hello_to_world_on_empty_name() {
        // The C function treats an empty string as a case to default to "World".
        let msg = hello(Some(""));

        assert!(msg.is_ok());
        // Assert that an empty name defaults to "World"
        assert_eq!(msg.unwrap(), "Hello, World\n");
    }

    #[test]
    fn say_hello_to_world_on_none_name() {
        // Our Rust wrapper defaults a None to "World".
        let msg = hello(None);
        assert!(msg.is_ok());
        assert_eq!(msg.unwrap(), "Hello, World\n");
    }

    #[test]
    fn check_internal_null_byte_error() {
        // C strings are null-terminated, so they cannot contain null bytes internally.
        // CString::new will return an error if this is attempted.
        let msg = hello(Some("Ru\0st"));
        assert!(msg.is_err());
        assert!(msg.unwrap_err().to_string().contains("interior null byte"));
    }
}
