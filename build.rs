extern crate pkg_config;

fn main() {
    match pkg_config::find_library("freetype2") {
        Ok(_) => return,
        Err(_) => {
            println!("cargo:rustc-link-lib=dylib=freetype");
        }
    }
}
