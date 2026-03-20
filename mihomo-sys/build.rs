//! **Vendored-style** native build (same idea as `openssl-sys` + `openssl-src`):
//! - Go sources live under **`builder/`** in the crate; nothing is fetched at link time except **`go mod`**.
//! - Each Cargo build compiles **`c-shared`** for the **Rust [`TARGET`]** (not the host machine when cross-compiling).
//! - Output: **`$OUT_DIR/dylib/<goos>-<goarch>/`**, same layout on every platform.
//!
//! **Cross-compilation:** `build.rs` sets **`GOOS`/`GOARCH`** from [`CARGO_CFG_*`] for the **target** triple. With **`CGO_ENABLED=1`**, Go needs a C toolchain for that target — set **`CC`** / **`CXX`** / **`AR`** (and sometimes **`GOARM`**, etc.) the same way you would for CGO. Those variables are inherited by the `go` subprocess.
//!
//! [`TARGET`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
//! [`CARGO_CFG_*`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn rerun_if_build_env() {
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=MIHOMO_LIB_DIR");
    for k in [
        "CC",
        "CXX",
        "AR",
        "CFLAGS",
        "CGO_CFLAGS",
        "CGO_LDFLAGS",
        "GOARM",
    ] {
        println!("cargo:rerun-if-env-changed={k}");
    }
}

fn rust_os_to_goos(rust_os: &str) -> &'static str {
    match rust_os {
        "macos" => "darwin",
        "linux" => "linux",
        "windows" => "windows",
        other => panic!(
            "mihomo-sys: unsupported CARGO_CFG_TARGET_OS={other} (wired for macOS/Linux/Windows); use MIHOMO_LIB_DIR for TARGET={}",
            env::var("TARGET").unwrap_or_else(|_| "?".into())
        ),
    }
}

fn rust_arch_to_goarch(rust_arch: &str, target: &str) -> &'static str {
    match rust_arch {
        "aarch64" => "arm64",
        "x86_64" => "amd64",
        "arm" | "armv7" => "arm",
        "riscv64" => "riscv64",
        "i586" | "i686" => "386",
        other => panic!(
            "mihomo-sys: unsupported CARGO_CFG_TARGET_ARCH={other} (TARGET={target}); \
             use a supported arch or supply a prebuilt via MIHOMO_LIB_DIR"
        ),
    }
}

fn dylib_name(goos: &str) -> &'static str {
    match goos {
        "darwin" => "libmihomo.dylib",
        "linux" => "libmihomo.so",
        "windows" => "mihomo.dll",
        other => panic!("unsupported goos: {other}"),
    }
}

fn prebuilt_names(goos: &str) -> &'static [&'static str] {
    match goos {
        "windows" => &[
            "mihomo.dll",
            "mihomo.dll.a",
            "libmihomo.dll.a",
            "libmihomo.dll.lib",
        ],
        "darwin" => &["libmihomo.dylib", "libmihomo.so"],
        "linux" => &["libmihomo.so"],
        other => panic!("unsupported goos: {other}"),
    }
}

fn go_build(builder: &Path, out: &Path, goos: &str, goarch: &str) -> Result<(), String> {
    if !builder.is_dir() {
        return Err(format!("builder not found: {}", builder.display()));
    }
    let st = Command::new("go")
        .current_dir(builder)
        .env("CGO_ENABLED", "1")
        .env("GOOS", goos)
        .env("GOARCH", goarch)
        .args([
            "build",
            "-trimpath",
            "-buildmode=c-shared",
            "-ldflags=-s -w",
            "-o",
        ])
        .arg(out)
        .arg(".")
        .status()
        .map_err(|e| format!("`go`: {e}"))?;
    if !st.success() {
        return Err(format!("`go build` failed: {st}"));
    }
    Ok(())
}

fn install_name_rpath(dylib: &Path) -> Result<(), String> {
    if !dylib.is_file() {
        return Ok(());
    }
    let p = dylib.to_str().ok_or_else(|| non_utf8(dylib))?;
    let st = Command::new("install_name_tool")
        .args(["-id", "@rpath/libmihomo.dylib", p])
        .status()
        .map_err(|e| format!("`install_name_tool`: {e}"))?;
    if !st.success() {
        return Err(format!("`install_name_tool` failed: {st}"));
    }
    Ok(())
}

fn non_utf8(p: &Path) -> String {
    format!("non-UTF8 path: {}", p.display())
}

fn prebuilt_ok(dir: &Path, names: &[&str]) -> bool {
    names.iter().any(|n| dir.join(n).is_file())
}

fn main() {
    if env::var_os("DOCS_RS").is_some() {
        println!("cargo:rustc-cfg=docsrs");
        return;
    }

    rerun_if_build_env();
    println!("cargo:rerun-if-changed=build.rs");
    for f in ["builder/lib.go", "builder/go.mod", "builder/go.sum"] {
        println!("cargo:rerun-if-changed={f}");
    }

    let target = env::var("TARGET").expect("TARGET must be set by Cargo");
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let builder = manifest.join("builder");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let rust_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let rust_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let goos = rust_os_to_goos(rust_os.as_str());
    let ga = rust_arch_to_goarch(&rust_arch, &target);
    let triple = format!("{goos}-{ga}");
    let name = dylib_name(goos);
    let names = prebuilt_names(goos);

    let mihomo_lib = env::var("MIHOMO_LIB_DIR").ok();
    let lib_dir = match &mihomo_lib {
        Some(dir) => PathBuf::from(dir),
        None => out_dir.join("dylib").join(&triple),
    };

    if mihomo_lib.is_none() {
        let out = lib_dir.join(name);
        fs_create_dir_all(&lib_dir);
        go_build(&builder, &out, goos, ga).unwrap_or_else(|e| {
            panic!(
                "mihomo-sys: TARGET={target} — {e}\n\
                 For cross-compiles, set CC/CXX for CGO (see docs/BUILD.md), or use MIHOMO_LIB_DIR with a prebuilt {name}."
            );
        });
        if !out.is_file() {
            panic!(
                "mihomo-sys: TARGET={target}: expected {} after go build",
                out.display()
            );
        }
    } else if !prebuilt_ok(&lib_dir, names) {
        panic!(
            "mihomo-sys: TARGET={target}: no mihomo library in {} (expected one of: {})",
            lib_dir.display(),
            names.join(", "),
        );
    }

    if goos == "darwin" {
        install_name_rpath(&lib_dir.join("libmihomo.dylib"))
            .unwrap_or_else(|e| panic!("mihomo-sys: TARGET={target}: {e}"));
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=mihomo");
    if matches!(goos, "darwin" | "linux") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
    }
    println!("cargo:lib_dir={}", lib_dir.display());

    if env::var("MIHOMO_VERBOSE").ok().as_deref() == Some("1") {
        eprintln!(
            "mihomo-sys: TARGET={target} GOOS={goos} GOARCH={ga} lib_dir={}",
            lib_dir.display()
        );
    }
}

fn fs_create_dir_all(dir: &Path) {
    std::fs::create_dir_all(dir)
        .unwrap_or_else(|e| panic!("create_dir_all {}: {e}", dir.display()));
}
