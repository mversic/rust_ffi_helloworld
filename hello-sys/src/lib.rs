// This brings the generated FFI declarations into our crate.
// The code will be located at $OUT_DIR/bindings.rs
// We must disable some lints that are commonly triggered by C code.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
