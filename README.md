# Embeds the [mihomo] proxy kernel in Rust

## Quick start

> TODO: Add 

## Development

```bash
cargo fmt --all && cargo fmt --all --check
gofmt -w mihomo-sys/builder/lib.go   # if Go installed
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

**Bump upstream mihomo:** `./scripts/bump-mihomo.sh` (or `--tag vX.Y.Z`).

## License

**GPL-3.0** — same as upstream [mihomo]

---

- [mihomo]: https://github.com/MetaCubeX/mihomo
