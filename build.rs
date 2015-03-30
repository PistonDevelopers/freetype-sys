extern crate pkg_config;

fn main() {
    match pkg_config::find_library("freetype2") {
        Ok(_) => return,
        Err(_) => {
            if cfg!(windows) {
                println!("cargo:rustc-link-lib=dylib=freetype-6");
            } else {
                println!("cargo:rustc-link-lib=dylib=freetype");
            }
        }
    }
}
