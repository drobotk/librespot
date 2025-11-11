fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/playplay.cpp")
        .compile("playplayimpl");

    println!("cargo:rustc-link-lib=static=playplayimpl");
}