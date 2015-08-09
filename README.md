freetype-sys [![Build Status](https://travis-ci.org/PistonDevelopers/freetype-sys.svg?branch=master)](https://travis-ci.org/PistonDevelopers/freetype-sys)
============

Low level bindings for the FreeType font library

## For windows users

In order to easily set-up freetype:

 - Download precompiled versions of `libfreetype-6.dll` and `zlib1.dll`
   ([32bits](http://www.gtk.org/download/win32.php) or [64bits](http://www.gtk.org/download/win64.php))

 - In the root of your project, in one of the parent directories, or in your home directory,
   create a `.cargo` directory. This directory should contain a `config`
   file that contains the following snippet:

```toml
[target.i686-pc-windows-gnu.freetype]
rustc-link-search = ["C:\\Path\\To\\32bits\\Freetype"]
rustc-link-lib = ["freetype-6"]
root = "C:\\Path\\To\\Freetype"

[target.x86_64-pc-windows-gnu.freetype]
rustc-link-search = ["C:\\Path\\To\\64bits\\Freetype"]
rustc-link-lib = ["freetype-6"]
root = "C:\\Path\\To\\Freetype"

[target.i686-pc-windows-gnu.z]
rustc-link-search = ["C:\\Path\\To\\32bits\\Zlib"]
rustc-link-lib = ["zlib1"]
root = "C:\\Path\\To\\Zlib"

[target.x86_64-pc-windows-gnu.z]
rustc-link-search = ["C:\\Path\\To\\64bits\\Zlib"]
rustc-link-lib = ["zlib1"]
root = "C:\\Path\\To\\Zlib"
```

For more informations, check [the official Cargo documentation](http://doc.crates.io/build-script.html#overriding-build-scripts).

## If you use Msys2
Another way with Msys2. Install package:
```
pacman -S mingw-w64-x86_64-freetype
```

Also you need to set path for linker:
```
export LIBRARY_PATH=/C/msys64/mingw64/bin/
```

And don't forget to add library to search path for your runtime.
