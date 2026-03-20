# mihomo-sys

FFI to [mihomo](https://github.com/MetaCubeX/mihomo) via Go **`c-shared`** (`builder/`).

**Build:** Go + CGO + network (first `go mod`). **Vendored-style** (like **`openssl-sys`** + **`openssl-src`**): compile **`builder/`** for the Rust **`TARGET`** into **`$OUT_DIR/dylib/<goos>-<goarch>/`**. **Cross:** set **`CC`/`CXX`** (CGO) or use **`MIHOMO_LIB_DIR`** with a prebuilt for that target. **Runtime:** dynamic **`libmihomo`**.

**Build flow and artifact layout:** [docs/BUILD.md](https://github.com/clash-zen/mihomo-rust/blob/main/docs/BUILD.md).

**GPL-3.0**
