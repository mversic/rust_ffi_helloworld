fn main() {
    println!("cargo:rerun-if-changed=../src/hello.c");
    println!("cargo:rerun-if-changed=../src/hello.h");

    cc::Build::new()
        .file("../src/hello.c")
        .include("../src")
        .compile("hello_c_lib");
}
