#![feature(env)]

extern crate "pkg-config" as pkg_config;

use std::env::var;
use pkg_config::find_library;

fn main() {
    if var("CARGO_FEATURE_NOPKGCFG").is_ok() || find_library("freetype2").is_err() {
        if cfg!(windows) {
            println!("cargo:rustc-flags=-l freetype-6:dylib");
        } else {
            println!("cargo:rustc-flags=-l freetype:dylib");
        }
    }
}
