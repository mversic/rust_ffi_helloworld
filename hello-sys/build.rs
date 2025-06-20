use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // --- Part 1: Define Header and Include Paths ---

    // The path to the source repository
    let hello_source_path = PathBuf::from("../"); // Adjust this path if needed

    // --- Part 2: Configure bindgen ---

    let bindings = bindgen::Builder::default()
        // The header file we want to generate bindings for.
        // We use a "wrapper" header for better control.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Add the necessary include paths for hello's internal headers.
        .clang_arg(format!("-I{}", hello_source_path.join("src").display()))
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
}
