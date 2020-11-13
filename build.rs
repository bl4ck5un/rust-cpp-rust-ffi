extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/library.cpp")
        .cpp(true)
        .compile("libcpp.a");
}