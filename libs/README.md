# libs

This directory holds shared libraries produced by `builder/` via:

```bash
go build -buildmode=c-shared
```

Build script: `scripts/build-libs.sh`

## Layout

- `libs/<os>-<arch>/libmihomo.so` (Linux)
- `libs/<os>-<arch>/libmihomo.dylib` (macOS)
- `libs/<os>-<arch>/mihomo.dll` (Windows)

The `mihomo-sys` crate links against the library for the current compilation target.
