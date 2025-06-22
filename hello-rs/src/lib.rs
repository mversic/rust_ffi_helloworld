use std::ffi::CString;

pub fn hello(name: &str) {
    // Convert the Rust &str to a C-compatible null-terminated string
    let c_name = CString::new(name).expect("CString::new failed due to interior null byte");

    // Call the FFI function (which is unsafe because it's a C call)
    // hello_sys::hello is expected to take a *const std::os::raw::c_char
    unsafe {
        hello_sys::hello(c_name.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gag::BufferRedirect;
    use std::io::Read;

    #[test]
    fn say_hello() {
        // Create a buffer to capture stdout
        let mut buf = BufferRedirect::stdout().expect("Failed to redirect stdout");

        hello("Rust");

        // Read the captured output into a string
        let mut output = String::new();
        buf.read_to_string(&mut output)
            .expect("Failed to read from stdout buffer");

        // Assert that the output is what we expect
        assert_eq!(output, "hello Rust\n");
        // `buf` goes out of scope here, and stdout is restored.
    }
}
