freetype-sys
============

Low level bindings for the FreeType font library

## For windows users

In order to easily set-up freetype:

 - Download precompiled versions of `libfreetype-6.dll` and `zlib1.dll`
   (see [the downloads page](http://www.freetype.org/download.html))

 - In the root of your project or in one of the parent directories,
   create a `.cargo` directory. This directory should contain a `config`
   file that contains the following snippet:

```toml
[target.i686-pc-windows-gnu.freetype]
rustc-flags = "-L C:\\Path\\To\\Freetype -l freetype-6:dylib"
root = "C:\\Path\\To\\Freetype"

[target.x86_64-pc-windows-gnu.freetype]
rustc-flags = "-L C:\\Path\\To\\Freetype -l freetype-6:dylib"
root = "C:\\Path\\To\\Freetype"

[target.i686-pc-windows-gnu.z]
rustc-flags = "-L C:\\Path\\To\\Zlib -l zlib1:dylib"
root = "C:\\Path\\To\\Zlib"

[target.x86_64-pc-windows-gnu.z]
rustc-flags = "-L C:\\Path\\To\\Zlib -l zlib1:dylib"
root = "C:\\Path\\To\\Zlib"
```

For more informations, check [the official Cargo documentation](http://doc.crates.io/build-script.html#overriding-build-scripts).
