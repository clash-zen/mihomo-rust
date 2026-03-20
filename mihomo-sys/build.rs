use std::env;
use std::path::PathBuf;
use std::process::Command;

fn normalize_arch(target_arch: &str) -> &'static str {
    match target_arch {
        "aarch64" => "arm64",
        "x86_64" => "amd64",
        other => panic!("Unsupported target arch: {other}"),
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=MIHOMO_LIB_DIR");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_dir = manifest_dir
        .parent()
        .expect("mihomo-sys/Cargo.toml should have a parent directory");

    let target_os_raw = env::var("CARGO_CFG_TARGET_OS").unwrap();
    // Cargo uses `macos` while our build script uses `darwin`.
    let target_os = match target_os_raw.as_str() {
        "macos" => "darwin",
        other => other,
    };
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let arch = normalize_arch(&target_arch);

    let lib_dir = match env::var("MIHOMO_LIB_DIR") {
        Ok(v) => PathBuf::from(v),
        Err(_) => root_dir.join("libs").join(format!("{target_os}-{arch}")),
    };

    let candidate_files: &[&str] = if target_os == "windows" {
        &[
            "mihomo.dll",
            "mihomo.dll.a",
            "libmihomo.dll.a",
            "libmihomo.dll.lib",
        ]
    } else if target_os == "darwin" {
        &["libmihomo.dylib", "libmihomo.so"]
    } else if target_os == "linux" {
        &["libmihomo.so"]
    } else {
        panic!("Unsupported target OS: {target_os}");
    };

    let mut found = false;
    for f in candidate_files {
        if lib_dir.join(f).exists() {
            found = true;
            break;
        }
    }

    if !found {
        // Best-effort: try to build libs automatically on the user's machine.
        // (This environment may not have `go`, so it can still fail.)
        let target = format!("{target_os}-{arch}");
        let script_path = root_dir.join("scripts/build-libs.sh");
        if script_path.exists() {
            let status = Command::new(&script_path).env("TARGETS", &target).status();

            if let Ok(s) = status {
                if s.success() {
                    for f in candidate_files {
                        if lib_dir.join(f).exists() {
                            found = true;
                            break;
                        }
                    }
                }
            }
        }

        if !found {
            panic!(
                "Missing mihomo dynamic library in {}\n\
                 Build it with:\n\
                   TARGETS=\"{target}\" {script}\n\
                 Or set MIHOMO_LIB_DIR to the directory containing the library.",
                lib_dir.display(),
                target = target,
                script = script_path.display()
            );
        }
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    // The Go c-shared output is named libmihomo.* on unix and mihomo.dll on windows.
    // Using `dylib=mihomo` matches the platform linker conventions.
    println!("cargo:rustc-link-lib=dylib=mihomo");

    // Helpful for local debugging (not required).
    if env::var("MIHOMO_VERBOSE").ok().as_deref() == Some("1") {
        eprintln!("mihomo-sys: linking from {}", lib_dir.display());
    }
}
