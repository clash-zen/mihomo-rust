# Build flow

This crate follows the same **vendored** idea as Rust’s **`openssl-sys`** + **`openssl-src`**: **sources ship with the crate**, the build script **compiles them for the Rust target**, and **artifacts live under Cargo’s `OUT_DIR`** — not under a global install prefix.

| | **`openssl-sys` vendored** | **`mihomo-sys` (this crate)** |
|---|------------------------------|--------------------------------|
| Sources | `openssl-src` (C) | `mihomo-sys/builder/` (Go, `c-shared`) |
| Who the build is for | **`$TARGET`** | **`$TARGET`** (see below) |
| Output | under **`OUT_DIR`** / build tree | **`$OUT_DIR/dylib/<goos>-<goarch>/`** |
| Override | `OPENSSL_DIR`, etc. | **`MIHOMO_LIB_DIR`** (prebuilt for that target) |

## Host vs `TARGET`

- **Host:** machine running **`cargo`** (your laptop, CI runner).
- **`TARGET`:** the [Rust target triple](https://doc.rust-lang.org/nightly/rustc/platform-support.html) for the artifact being built (`aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, …).

The build script runs **on the host** but must produce a library for **`TARGET`**.  
Cargo sets **`TARGET`**, **`CARGO_CFG_TARGET_OS`**, **`CARGO_CFG_TARGET_ARCH`**, etc. for that triple — **not** for the host when cross-compiling.

## Pipeline

1. **`cargo build` / `cargo test`** pulls in **`mihomo-sys`**.
2. If **`DOCS_RS`** is set ([docs.rs](#docsrs)), **`build.rs`** exits after **`cfg(docsrs)`** — no Go, no link.
3. Otherwise **`build.rs`** resolves **`GOOS` / `GOARCH`** from **`CARGO_CFG_TARGET_*`** (i.e. **`TARGET`**).
4. If **`MIHOMO_LIB_DIR`** is **unset**:
   - Runs **`go build -trimpath -buildmode=c-shared -ldflags=-s -w`** from **`mihomo-sys/builder/`** with **`CGO_ENABLED=1`**, **`GOOS`**, **`GOARCH`**.
   - Writes **`libmihomo.so`** / **`libmihomo.dylib`** / **`mihomo.dll`** under **`$OUT_DIR/dylib/<goos>-<goarch>/`**.
5. **macOS target:** **`install_name_tool -id @rpath/libmihomo.dylib`** on the dylib so **dyld** matches the **`-rpath`** passed to the linker.
6. Emits **`cargo:rustc-link-search`**, **`dylib=mihomo`**, and **`-Wl,-rpath`** on Linux/macOS.

If **`MIHOMO_LIB_DIR`** is **set**, step 4 does **not** run **`go build`**; the directory must already contain a **`libmihomo`** (or **`mihomo.dll`**) for **that `TARGET`**.

## Cross-compilation

`GOOS` / `GOARCH` follow **`TARGET`**.  
With **`CGO_ENABLED=1`**, Go needs a **C cross-compiler** for the destination — same idea as **`CC`/`CXX`** for **`openssl-src`**.

- Set **`CC`**, **`CXX`**, **`AR`** (and sometimes **`GOARM`**, **`CGO_CFLAGS`**, …) so the **`go`** toolchain can invoke the right toolchain for the **target** OS/arch.
- **Rust** does not pass these for you; **`build.rs`** inherits the process environment into the **`go`** subprocess.
- **`build.rs`** prints **`cargo:rerun-if-env-changed=TARGET`** and **`rerun-if-env-changed`** for **`CC`**, **`CXX`**, **`AR`**, **`GOARM`**, etc., so changing the toolchain forces a rebuild.

If CGO cross-compilation is too heavy, **build the shared library on the target (or in a matching container)** and point **`MIHOMO_LIB_DIR`** at that directory when building Rust for that **`TARGET`**.

**Supported OS (bundled Go build):** **`linux`**, **`darwin`**, **`windows`** (see **`build.rs`**). Other OS triples need **`MIHOMO_LIB_DIR`**.

## Output

```text
$OUT_DIR/dylib/<goos>-<goarch>/<filename>
```

| Rust `TARGET` (examples) | Directory | File |
|--------------------------|-----------|------|
| `aarch64-apple-darwin` | `darwin-arm64` | `libmihomo.dylib` |
| `x86_64-unknown-linux-gnu` | `linux-amd64` | `libmihomo.so` |
| `aarch64-pc-windows-msvc` | `windows-arm64` | `mihomo.dll` |

Under **`target/`**: **`target/<profile>/build/mihomo-sys-*/out/dylib/...`**

## `MIHOMO_LIB_DIR`

Prebuilt tree for the **current `TARGET`** (no `go build`):

```bash
MIHOMO_LIB_DIR=/path/to/dir cargo build -p mihomo --target <triple>
```

## docs.rs

**`DOCS_RS`**: no **`go build`**, no real link; stubs only.

## Commands

```bash
cargo build -p mihomo
```

```bash
cargo build -p mihomo --target aarch64-unknown-linux-gnu
```

```bash
MIHOMO_LIB_DIR=/path/to/dir cargo build -p mihomo --target aarch64-unknown-linux-gnu
```

If **`go mod`** fails: **`(cd mihomo-sys/builder && go mod tidy)`**.

## See also

- **[CI.md](CI.md)** — GitHub Actions: **`ci.yml`** (push/PR) and **`release.yml`** (tag **`v*`**).
