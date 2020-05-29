extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.contains("android")
        && pkg_config::Config::new().atleast_version("18.5.12").find("freetype2").is_ok()
    {
        return
    }

    let mut config = Config::new("freetype2");
    if let Ok(s) = env::var("FREETYPE_CMAKE_GENERATOR") {
        config.generator(s);
    }
    let dst = config
        .define("WITH_BZip2", "OFF")
        .define("WITH_HarfBuzz", "OFF")
        .define("WITH_PNG", "OFF")
        .define("WITH_ZLIB", "OFF")
        .profile("Release")
        .build();
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=freetype");
    println!("cargo:outdir={}", out_dir);
}
