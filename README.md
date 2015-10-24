freetype-sys [![Build Status](https://travis-ci.org/PistonDevelopers/freetype-sys.svg?branch=master)](https://travis-ci.org/PistonDevelopers/freetype-sys)
============

Low level bindings for the FreeType font library

## For windows users

In order to easily set-up freetype:

 - Download precompiled versions of `libfreetype6.dll` and `zlib1.dll`
   ([32bits](http://www.gtk.org/download/win32.php) or [64bits](http://www.gtk.org/download/win64.php))

 - In the root of your project, in one of the parent directories, or in your home directory,
   create a `.cargo` directory. This directory should contain a `config`
   file that contains the following snippet:

```toml
[target.i686-pc-windows-gnu.freetype6]
rustc-link-search = ["C:\\Path\\To\\32bits\\Freetype"]
rustc-link-lib = ["freetype6"]

[target.x86_64-pc-windows-gnu.freetype6]
rustc-link-search = ["C:\\Path\\To\\64bits\\Freetype"]
rustc-link-lib = ["freetype6"]

[target.i686-pc-windows-gnu.z]
rustc-link-search = ["C:\\Path\\To\\32bits\\Zlib"]
rustc-link-lib = ["zlib1"]

[target.x86_64-pc-windows-gnu.z]
rustc-link-search = ["C:\\Path\\To\\64bits\\Zlib"]
rustc-link-lib = ["zlib1"]
```

For more informations, check [the official Cargo documentation](http://doc.crates.io/build-script.html#overriding-build-scripts).
