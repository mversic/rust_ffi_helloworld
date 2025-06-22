use std::ffi::{CStr, CString};

pub fn hello(name: &str) -> String {
    // Convert the Rust &str to a C-compatible null-terminated string
    let c_name = CString::new(name).expect("CString::new failed due to interior null byte");

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
        c_str.to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn say_hello() {
        let msg = hello("Rust");

        // Assert that the output is what we expect
        assert_eq!(msg, "hello Rust\n");
        // `buf` goes out of scope here, and stdout is restored.
    }
}
