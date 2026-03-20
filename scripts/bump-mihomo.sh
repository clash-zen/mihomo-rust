#!/usr/bin/env bash
# Bump github.com/metacubex/mihomo in builder/ via the Go module proxy (no local mihomo-src).
# Requires: Go on PATH and network access to the module proxy.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILDER="${ROOT}/builder"
MOD="github.com/metacubex/mihomo"

usage() {
  cat <<'EOF'
Usage:
  bump-mihomo.sh                 # Default: go get @latest, then go mod tidy
  bump-mihomo.sh --latest        # Same as default
  bump-mihomo.sh --tag TAG       # Pin to a release tag (e.g. v1.19.21)
  bump-mihomo.sh TAG             # Same as --tag (positional)

Options:
  --tag TAG      Version or tag passed to go get (e.g. v1.19.21)
  --latest       Use @latest (default when no tag given)
  --no-tidy      Skip go mod tidy after go get
  -h, --help     Show this help

Updates builder/go.mod and builder/go.sum. Commit those files after review.

Examples:
  ./scripts/bump-mihomo.sh
  ./scripts/bump-mihomo.sh --tag v1.19.21
  ./scripts/bump-mihomo.sh v1.19.21
EOF
}

want_latest=0
version_ref=""
no_tidy=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    --latest)
      want_latest=1
      shift
      ;;
    --tag)
      if [[ $# -lt 2 ]]; then
        echo "error: --tag requires an argument" >&2
        exit 1
      fi
      version_ref="$2"
      shift 2
      ;;
    --no-tidy)
      no_tidy=1
      shift
      ;;
    -*)
      echo "error: unknown option: $1" >&2
      usage >&2
      exit 1
      ;;
    *)
      if [[ -n "$version_ref" ]]; then
        echo "error: only one version allowed (already have: $version_ref)" >&2
        exit 1
      fi
      version_ref="$1"
      shift
      ;;
  esac
done

if [[ "$want_latest" -eq 1 && -n "$version_ref" ]]; then
  echo "error: cannot use --latest together with an explicit tag" >&2
  exit 1
fi

if ! command -v go >/dev/null 2>&1; then
  echo "error: go not found in PATH" >&2
  exit 1
fi

if [[ ! -f "${BUILDER}/go.mod" ]]; then
  echo "error: ${BUILDER}/go.mod not found" >&2
  exit 1
fi

spec=""
if [[ -n "$version_ref" ]]; then
  spec="${MOD}@${version_ref}"
elif [[ "$want_latest" -eq 1 || -z "$version_ref" ]]; then
  spec="${MOD}@latest"
fi

echo "Running: (cd builder && go get ${spec})"
(
  cd "$BUILDER"
  go get "$spec"
)

if [[ "$no_tidy" -eq 0 ]]; then
  echo "Running: (cd builder && go mod tidy)"
  (cd "$BUILDER" && go mod tidy)
fi

echo "Done. Current module version:"
(cd "$BUILDER" && go list -m "$MOD")
