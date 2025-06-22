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

    #[test]
    fn succeeds_when_output_fits_exactly() {
        // The internal buffer in `hello` is 300 bytes.
        // The output format is "Hello, %s\n".
        // "Hello, " is 7 bytes.
        // "\n" is 1 byte.
        // The null terminator is 1 byte.
        // Total overhead is 9 bytes.
        // A name of (300 - 9) = 291 bytes will result in a 299-byte string,
        // which fits perfectly into the 300-byte buffer with its null terminator.
        let long_name = "a".repeat(291);
        let result = hello(Some(&long_name));

        assert!(result.is_ok());
        let expected = format!("Hello, {}\n", long_name);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn fails_when_output_buffer_is_too_small() {
        // A name of 292 bytes will cause the C function to report an error because
        // the total output (7 + 292 + 1 = 300 bytes) plus the null terminator
        // requires a 301-byte buffer, but ours is 300.
        let long_name = "a".repeat(292);
        let result = hello(Some(&long_name));

        assert!(result.is_err(), "Expected an error, but got Ok");
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("output buffer was too small")
        );
    }

    #[test]
    fn handles_multibyte_utf8_characters_correctly() {
        // "😊" is a 4-byte UTF-8 character. This test ensures that byte lengths,
        // not character counts, are handled correctly across the FFI boundary.
        let name = "😊";
        let msg = hello(Some(name));
        assert!(msg.is_ok());
        assert_eq!(msg.unwrap(), "Hello, 😊\n");
    }

    #[test]
    fn handles_input_with_c_format_specifiers() {
        // This is a defensive test. The C code uses `snprintf("...", name)`, which is safe.
        // An unsafe implementation might do `snprintf(name, ...)` with a user-controlled
        // string, which would be a format string vulnerability. This test confirms
        // our implementation is not vulnerable and treats specifiers as literals.
        let name = "user %s%s%n";
        let msg = hello(Some(name));
        assert!(msg.is_ok());
        assert_eq!(msg.unwrap(), "Hello, user %s%s%n\n");
    }
}
