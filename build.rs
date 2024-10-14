use std::env;

fn add_sources(build: &mut cc::Build, root: &str, files: &[&str]) {
    let root = std::path::Path::new(root);
    build.files(files.iter().map(|src| {
        let mut p = root.join(src);
        p.set_extension("c");
        p
    }));

    build.include(root);
}

fn main() {
    if !cfg!(feature = "bundled") {
        let target = env::var("TARGET").unwrap();
        if !target.contains("android") && !target.contains("ohos") {
            pkg_config::Config::new()
                .atleast_version("24.3.18")
                .probe("freetype2")
                .unwrap();
        }
        return;
    }

    // From libpng/png.h:
    // "If pnglibconf.h is missing, you can copy scripts/pnglibconf.h.prebuilt to pnglibconf.h"
    std::fs::copy(
        "libpng/scripts/pnglibconf.h.prebuilt",
        "libpng/pnglibconf.h",
    )
    .unwrap();

    let mut build = cc::Build::new();

    build
        .warnings(false)
        .include(".")
        .include("freetype2/include")
        .include("libpng")
        .define("FT2_BUILD_LIBRARY", None)
        .define("FT_CONFIG_OPTION_USE_PNG", None);

    add_sources(
        &mut build,
        "freetype2/src",
        &[
            "autofit/autofit",
            "base/ftbase",
            "base/ftbbox",
            "base/ftbdf",
            "base/ftbitmap",
            "base/ftcid",
            "base/ftdebug",
            "base/ftfstype",
            "base/ftgasp",
            "base/ftglyph",
            "base/ftgxval",
            "base/ftinit",
            "base/ftmm",
            "base/ftotval",
            "base/ftpatent",
            "base/ftpfr",
            "base/ftstroke",
            "base/ftsynth",
            "base/ftsystem",
            "base/fttype1",
            "base/ftwinfnt",
            "bdf/bdf",
            "bzip2/ftbzip2",
            "cache/ftcache",
            "cff/cff",
            "cid/type1cid",
            "gzip/ftgzip",
            "lzw/ftlzw",
            "pcf/pcf",
            "pfr/pfr",
            "psaux/psaux",
            "pshinter/pshinter",
            "psnames/psnames",
            "raster/raster",
            "sdf/sdf",
            "svg/svg",
            "sfnt/sfnt",
            "smooth/smooth",
            "truetype/truetype",
            "type1/type1",
            "type42/type42",
            "winfonts/winfnt",
        ],
    );

    build.compile("freetype2");

    println!("cargo:rustc-link-lib=z");

    let mut build = cc::Build::new();
    build.include("libpng").include("libz-sys/src/zlib");
    build
        .file("libpng/png.c")
        .file("libpng/pngerror.c")
        .file("libpng/pngget.c")
        .file("libpng/pngmem.c")
        .file("libpng/pngpread.c")
        .file("libpng/pngread.c")
        .file("libpng/pngrio.c")
        .file("libpng/pngrtran.c")
        .file("libpng/pngrutil.c")
        .file("libpng/pngset.c")
        .file("libpng/pngtrans.c")
        .file("libpng/pngwio.c")
        .file("libpng/pngwrite.c")
        .file("libpng/pngwtran.c")
        .file("libpng/pngwutil.c");

    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if arch == "arm" || arch == "aarch64" {
        build
            .file("libpng/arm/arm_init.c")
            .file("libpng/arm/filter_neon_intrinsics.c")
            .file("libpng/arm/filter_neon.S")
            .file("libpng/arm/palette_neon_intrinsics.c");
    }

    build.compile("libpng.a");
}
