freetype-sys [![Build Status](https://github.com/PistonDevelopers/freetype-sys/actions/workflows/ci.yml/badge.svg)](https://github.com/PistonDevelopers/freetype-sys/actions/workflows/ci.yml)
============

Low level bindings for the FreeType font library.

# Statically linking against FreeType

If the `bundled` feature is enabled, `freetype-sys` will build and link a static copy of FreeType. This requires a C compiler. The included version of FreeType is 2.13.2.

```
[dependencies]
freetype-sys = { version = "0.21", features = ["bundled"] }
```

## For Windows users

### -pc-windows-gnu
In order to easily setup FreeType just get MSYS2 and install either the `mingw-w64-x86_64-freetype` or `mingw-w64-i686-freetype` package and then use Rust from within the correct MinGW shell of MSYS2.

More information on setting up MSYS2 for Rust can be found in [the Rust installing from source document](https://github.com/rust-lang/rust/blob/master/INSTALL.md#building-on-windows).

### -pc-windows-msvc
Prebuilt libraries for FreeType are available [here](https://github.com/PistonDevelopers/binaries).

Then in the root of your project, in one of the parent directories, or in your home directory, create a .cargo directory. This directory should contain a `config` file that contains the following snippet:

```toml
[target.i686-pc-windows-msvc.freetype]
rustc-link-search = ["C:\\Path\\To\\binaries\\i686"]
rustc-link-lib = ["freetype"]

[target.x86_64-pc-windows-msvc.freetype]
rustc-link-search = ["C:\\Path\\To\\binaries\\x86_64"]
rustc-link-lib = ["freetype"]
```

For more information, check [the official Cargo documentation](https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts).
