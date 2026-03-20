#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILDER_DIR="${ROOT}/builder"
OUT_DIR="${ROOT}/libs"

if ! command -v go >/dev/null 2>&1; then
  echo "go executable not found in PATH. Install Go and retry." >&2
  exit 1
fi

build_one() {
  local target="$1" # <os>-<arch> like darwin-arm64 / linux-amd64 / windows-amd64
  local os="${target%%-*}"
  local arch="${target##*-}"

  local goarch
  case "$arch" in
    arm64) goarch="arm64" ;;
    amd64|x86_64) goarch="amd64" ;;
    *) echo "Unsupported arch: $arch" >&2; exit 1 ;;
  esac

  local dir="${OUT_DIR}/${os}-${arch}"
  mkdir -p "$dir"

  local out
  case "$os" in
    darwin) out="${dir}/libmihomo.dylib" ;;
    linux) out="${dir}/libmihomo.so" ;;
    windows) out="${dir}/mihomo.dll" ;;
    *) echo "Unsupported os: $os" >&2; exit 1 ;;
  esac

  echo "Building $out ..."
  (
    export CGO_ENABLED=1
    export GOOS="$os"
    export GOARCH="$goarch"
    # Cwd must be the Go module root (builder/).
    cd "$BUILDER_DIR"
    go build -buildmode=c-shared -o "$out" .
  )
}

if [[ -n "${TARGETS:-}" ]]; then
  # Space-separated list.
  for t in $TARGETS; do
    build_one "$t"
  done
  exit 0
fi

default_target="$(go env GOOS)-$(go env GOARCH)"
case "$default_target" in
  darwin-amd64|darwin-arm64|linux-amd64|windows-amd64)
    build_one "$default_target"
    ;;
  *)
    # Normalize GOARCH values (e.g., arm64 stays arm64; amd64 stays amd64).
    echo "Building current platform target: $default_target"
    build_one "$default_target"
    ;;
esac

