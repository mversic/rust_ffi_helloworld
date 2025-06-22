use std::env;
use std::path::PathBuf;

fn main() {
    // --- Part 1: Define Header and Include Paths ---

    // The path to the source repository
    let hello_source_path = PathBuf::from("../src"); // Adjust this path if needed

    let hello_c_file = hello_source_path.join("hello.c");
    let wrapper_h_file = PathBuf::from("wrapper.h");
    let hello_h_file = hello_source_path.join("hello.h");

    println!("cargo:rerun-if-changed={}", wrapper_h_file.display());
    println!("cargo:rerun-if-changed={}", hello_c_file.display());
    println!("cargo:rerun-if-changed={}", hello_h_file.display());

    // --- Compile C code ---
    // This will compile hello.c and create a static library (e.g., libhello_c_lib.a)
    // The name "hello_c_lib" is arbitrary but should be consistent.
    cc::Build::new()
        .file(&hello_c_file)
        .include(&hello_source_path) // So hello.c can find hello.h if it needs to, and for bindgen's include context
        .compile("hello_c_lib");

    // --- Part 2: Configure bindgen ---

    let bindings = bindgen::Builder::default()
        // The header file we want to generate bindings for.
        // We use a "wrapper" header for better control.
        .header(wrapper_h_file.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Add the necessary include paths for hello's internal headers.
        .clang_arg(format!("-I{}", hello_source_path.display())) // To find hello.h from wrapper.h
        // --- Part 3: Filter What to Generate (IMPORTANT) ---
        // By default, bindgen generates everything. hello headers include tons of
        // system headers. We should only allowlist the functions/types we need.
        // This makes the generated code smaller and compilation faster.
        .allowlist_function("hello")
        // Generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // --- Part 4: Write the bindings to a file ---

    // Get the path to cargo's output directory.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // --- Part 5: Link the compiled C library ---
    println!("cargo:rustc-link-lib=static=hello_c_lib"); // Link the library we just compiled
}
