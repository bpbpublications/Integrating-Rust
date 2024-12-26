fn main() {
    // Get current directory of the build script     
    let current_dir = std::env::current_dir().unwrap();

    //  Tell Cargo to link with the library
    println!("cargo:rustc-link-lib=dylib=extern_example");

   // Tell Cargo where to find the library.
   println!(“cargo:rustc-link-search=native={}/src/", current_dir.display());
}