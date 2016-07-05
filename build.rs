extern crate cmake;

use cmake::Config;

fn main() {
    let freetype = Config::new("libfreetype")
        .define("FREETYPE_NO_DIST", "true")
        .define("WITH_ZLIB", "OFF")
        .define("WITH_BZIP2", "OFF")
        .define("WITH_PNG", "OFF")
        .define("WITH_HARFBUZZ", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/build", freetype.display());
    println!("cargo:rustc-link-lib=static=freetype");
}
