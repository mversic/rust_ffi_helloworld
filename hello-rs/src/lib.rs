use anyhow::{Context, Result, anyhow};
use std::ffi::CString;
use std::os::raw::c_char;

pub fn hello(name: Option<&str>) -> Result<String> {
    // Convert the optional Rust &str to an optional C-compatible null-terminated string.
    // This will be `Some(CString)` if a name is provided, or `None` if the input is `None`.
    // `transpose()` converts `Option<Result<T, E>>` to `Result<Option<T>, E>`.
    let c_name_opt: Option<CString> = name
        .map(CString::new)
        .transpose()
        .context("CString::new failed due to interior null byte on parameter 'name'")?;

    // Get a pointer to the C string, or a null pointer if `name` was `None`.
    // The C function is designed to handle a null pointer gracefully.
    let name_ptr = match &c_name_opt {
        Some(s) => s.as_ptr(),
        None => std::ptr::null(),
    };

    // Create a buffer for the C function to write into.
    // We are responsible for allocating memory that the C code can use.
    let mut buffer: Vec<u8> = vec![0; 300];

    // Call the FFI function. The `unsafe` block is necessary because we are calling C code,
    // passing raw pointers, and trusting it to write safely into our buffer.
    unsafe {
        let written = hello_sys::hello(name_ptr, buffer.as_mut_ptr() as *mut c_char, buffer.len());

        if written < 0 {
            return Err(anyhow!(
                "C function 'hello' failed: the output buffer was too small."
            ));
        }

        // `written` is the number of bytes written (excluding the null terminator).
        let result_slice = &buffer[..written as usize];
        String::from_utf8(result_slice.to_vec())
            .context("Failed to convert C output to UTF-8 string")
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
        // Our Rust wrapper now passes NULL for None, and the C code defaults to "World".
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
