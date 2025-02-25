fn main() {
    cc::Build::new()
        .cpp(true)  // Enable C++ compilation
        .include("include")
        .flag("-std=c++23")
        .file("src/driver/args.cc")  // Path to your .cc file
        .compile("args");  // Output static library
}
