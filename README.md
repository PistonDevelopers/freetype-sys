freetype-sys [![Build Status](https://travis-ci.org/PistonDevelopers/freetype-sys.svg?branch=master)](https://travis-ci.org/PistonDevelopers/freetype-sys)
============

Low level bindings for the FreeType font library

## For windows users

In order to easily set-up freetype:

 - Download precompiled versions of `libfreetype6.dll` and `zlib1.dll`
   ([32bits](http://sourceforge.net/projects/gnuwin32/files/freetype/2.3.5-1/freetype-2.3.5-1-setup.exe/download) or [64bits](http://lvserver.ugent.be/gtk-win64/gtk3-runtime/gtk3-runtime-3.14.13-2015-07-03-ts-win64.exe))

 - In the root of your project, in one of the parent directories, or in your home directory,
   create a `.cargo` directory. This directory should contain a `config`
   file that contains the following snippet:

```toml
[target.i686-pc-windows-gnu.freetype]
rustc-link-search = ["C:\\Path\\To\\32bits\\Freetype"]
rustc-link-lib = ["freetype6"]

[target.x86_64-pc-windows-gnu.freetype]
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
