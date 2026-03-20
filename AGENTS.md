# Rules for AI assistants

## 1. Language

- **Committed content:** **English only** (docs, comments, commits, changelogs, templates).
- **Chat with stakeholders:** their language; does not override the rule above for commits.

## 2. Code style

- **Rust:** **`rustfmt`**. **`unsafe`** only at FFI; brief notes if non-obvious.
- **Go:** **`gofmt`**. Small, stable C ABI.
- **Shell:** **`bash`**, **`set -euo pipefail`** for scripts; messages in English.

## 3. Repository

- **Crates:** **`mihomo-sys`** = FFI + link; **`mihomo`** = safe API. ABI from **`mihomo-sys/builder/lib.go`** (`//export`).
- **Native lib:** vendored Go in **`builder/`** → **`$OUT_DIR/dylib/<goos>-<goarch>/`** for **`TARGET`**. Override: **`MIHOMO_LIB_DIR`**. Cross: CGO **`CC`/`CXX`**. Details: **`docs/BUILD.md`**.
