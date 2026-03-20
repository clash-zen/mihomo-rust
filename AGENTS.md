# Rules for AI assistants

## 1. Language

- **Committed content** (anything under version control): **English only** — documentation, code comments, commit messages, changelogs, in-repo release notes, human-readable templates.
- **User-facing interaction** (chat, support, product copy not checked into the repo): use the **language agreed with stakeholders** or **the language the user writes in**. That does **not** override the English rule for committed material.

## 2. Code style

- **Rust**: Run **`rustfmt`**. Use idiomatic naming (`snake_case`, `PascalCase`, etc.). Keep `unsafe` at FFI boundaries or other documented invariants; add brief, accurate notes where non-obvious.
- **Go**: Run **`gofmt`**. Keep any exported C ABI small, stable, and documented.
- **Shell**: Prefer **`bash`** with `set -euo pipefail` when non-interactive; aim for idempotent behavior; user-visible messages in **English** (per §1 for committed scripts).

## 3. This repository — instructions for agents only

- **Respect the OpenSSL-style split** (`openssl-sys` / `openssl` pattern):
  - **`mihomo-sys`**: only linking (`build.rs`), `extern "C"`, thin `unsafe`. **Do not** add ergonomic APIs here.
  - **`mihomo`**: only safe public API; hide FFI; enforce invariants (e.g. single in-process start/stop). **Do not** expose raw pointers or C types publicly.
  - **Extending the ABI:** `//export` in `builder/lib.go` → matching declarations in `mihomo-sys` → optional wrapper in `mihomo`.
- **`builder/`:** keep Go **c-shared** exports minimal; use return codes plus a **`last_error`**-style path for errors (match `lib.go`).
- **Bumping upstream:** use `./scripts/bump-mihomo.sh` (or `go get` / `go mod tidy` under `builder/`) — do not add a local fork path to the mainline `go.mod` unless the owner explicitly orders it.
