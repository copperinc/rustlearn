// Build dependencies.

// Bring in a dependency on an externally maintained `gcc` package which manages
// invoking the C compiler.
#[cfg(feature = "svm")]
extern crate cc;

fn main() {
    #[cfg(feature = "svm")]
    {
        cc::Build::new()
            .cpp(true) // Switch to C++ library compilation.
            .file("dependencies/libsvm/svm.cpp")
            .compile("libsvm.a");
        println!("cargo:rustc-link-lib=svm");
    }
}
