#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
    VERSION=$(grep '^version' "$REPO_ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)"/\1/')
fi

OS=$(uname | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    linux)  EXT="tar.gz" ;;
    darwin) OS="macos"; EXT="zip" ;;
    *)      echo "Unsupported OS: $OS" >&2; exit 1 ;;
esac

ARCHIVE_NAME="gurdo-${VERSION}-${OS}-${ARCH}.${EXT}"
DIST_DIR="$REPO_ROOT/dist"
ARCHIVE_PATH="$DIST_DIR/$ARCHIVE_NAME"

echo "Building gurdo $VERSION for $OS/$ARCH..."
cd "$REPO_ROOT"
cargo build --release

mkdir -p "$DIST_DIR"

BINARY="$REPO_ROOT/target/release/gurdo"
OFL="$REPO_ROOT/assets/fonts/OFL.txt"

echo "Packaging $ARCHIVE_NAME..."
if [ "$EXT" = "tar.gz" ]; then
    tar -czf "$ARCHIVE_PATH" -C "$REPO_ROOT/target/release" gurdo -C "$REPO_ROOT/assets/fonts" OFL.txt
else
    zip -j "$ARCHIVE_PATH" "$BINARY" "$OFL"
fi

echo "Archive: $ARCHIVE_PATH"
echo "Contents:"
if [ "$EXT" = "tar.gz" ]; then
    tar -tzf "$ARCHIVE_PATH"
else
    unzip -l "$ARCHIVE_PATH" | awk 'NR>3 && /gurdo|OFL/ {print $NF}'
fi
