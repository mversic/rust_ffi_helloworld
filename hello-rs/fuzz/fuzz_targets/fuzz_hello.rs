#![no_main]

use hello_rs::hello;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // The fuzzer provides raw bytes. Our function's public API expects a `&str`,
    // which must be valid UTF-8.
    // We'll try to convert the bytes to a UTF-8 string. If it's not
    // valid UTF-8, we simply skip this input, as it falls outside the
    // function's contract. The fuzzer will quickly generate another input.
    if let Ok(s) = std::str::from_utf8(data) {
        // We call the function with the fuzzer-generated string.
        // We don't need to check the result (Ok or Err). The goal of the
        // fuzzer is to find inputs that cause a panic, crash, or hang.
        let _ = hello(Some(s));
    }
});
