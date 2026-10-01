use co3::ffi;
use std::ffi::{CStr, c_int};

ffi! {
    #![unsafe(extern("C"))]

    #[symbol_name = "hello"]
    pub fn hello_raw(name: Option<&CStr>, #[unpack(_, _)] out_buffer: &mut [u8]) -> c_int;
}
