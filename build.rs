#[cfg(feature = "link-freetype")]
extern crate pkg_config;

fn main() {
    #[cfg(feature = "link-freetype")]
    match pkg_config::find_library("freetype2") {
        Ok(_) => return,
        Err(_) => {
            println!("cargo:rustc-link-lib=dylib=freetype");
        }
    }
}
