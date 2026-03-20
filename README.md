# Embeds the [mihomo] proxy kernel in Rust

**Why this shape:** Go keeps parity with upstream features and release cadence; Rust callers get a normal crate API without maintaining a full native port of the core.

## Prerequisites

- **Go**
- **Rust** toolchain

## Format, lint, and test

```bash
cargo fmt --all
gofmt -w builder/lib.go   # when Go is installed
cargo fmt --all --check
./scripts/build-libs.sh   # produces libs/<os>-<arch>/; needs Go and module download on first run
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Upstream is a normal Go module dependency (no local `replace`, no `mihomo-src` tree). After changing `builder/go.mod`, run `(cd builder && go mod tidy)` with network access if `go build` complains about `go.sum`.

## Build shared library

```bash
./scripts/build-libs.sh
```

Or cross-target:

```bash
TARGETS="darwin-arm64 linux-amd64 windows-amd64" ./scripts/build-libs.sh
```

## Build Rust crates

```bash
cargo build -p mihomo
```

If the library lives elsewhere:

```bash
MIHOMO_LIB_DIR=/path/to/dir cargo build -p mihomo
```

## Example

```bash
cargo run -p mihomo --example in_process
```

## Bump upstream mihomo (Go module)

```bash
./scripts/bump-mihomo.sh              # @latest + go mod tidy
./scripts/bump-mihomo.sh --tag v1.19.21
./scripts/bump-mihomo.sh --no-tidy    # only go get, skip tidy
```

Or manually:

```bash
(cd builder && go get github.com/metacubex/mihomo@v1.19.21 && go mod tidy)
```

Then commit `builder/go.mod` and `builder/go.sum`:

```bash
git add builder/go.mod builder/go.sum
git commit -m "chore: bump mihomo to v1.19.21"
```

## License

**GPL-3.0** — same as upstream [mihomo]

---

- [mihomo]: https://github.com/MetaCubeX/mihomo
