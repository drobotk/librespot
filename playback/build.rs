fn main() {
    cc::Build::new()
        .cpp(true)
		.std("c++20")
        .file("src/playplay.cpp")
        .compile("playplayimpl");

    println!("cargo:rustc-link-lib=static=playplayimpl");
}
