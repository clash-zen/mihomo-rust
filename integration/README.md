# Integration

Crates here depend **only** on **`mihomo`** (downstream-shaped smoke tests).

**`it_works`** — start core + GET `http://127.0.0.1:39090/version` (ports **`37890`** / **`39090`** to reduce clashes with a local Clash/mihomo). **`mihomo-sys/build.rs`** sets **`rpath`** to the directory containing **`libmihomo`** (under **`$OUT_DIR/dylib/...`** from the **`mihomo-sys`** build).

```bash
cargo run -p it_works
```

More: [README](../README.md#prerequisites), [Building](../README.md#building-the-shared-library).
