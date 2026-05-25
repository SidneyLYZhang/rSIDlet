#!/usr/bin/env bash
set -euo pipefail

# --- Configuration ---
OWNER="SidneyLYZhang"
REPO="rSIDlet"
NAME="rsidlet"
BINARY="sidlet"
INSTALL_DIR="${RSIDLET_INSTALL_DIR:-$HOME/.local/bin}"

# --- Parse optional version ---
VERSION="${1:-}"

# --- Detect OS and arch ---
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64|amd64)   ARCH="x86_64" ;;
    aarch64|arm64)  ARCH="aarch64" ;;
    *)
        echo "Error: Unsupported architecture: $ARCH" >&2
        exit 1
        ;;
esac

case "$OS" in
    linux)
        case "$ARCH" in
            x86_64)  TARGET="ubuntu" ;;
            aarch64) TARGET="aarch64" ;;
        esac
        EXT="tar.xz"
        ;;
    darwin)
        TARGET="macos"
        EXT="tar.xz"
        ;;
    *)
        echo "Error: Unsupported OS: $OS" >&2
        exit 1
        ;;
esac

# --- Resolve version ---
if [ -z "$VERSION" ]; then
    echo "Fetching latest release tag..."
    VERSION=$(curl -sSf "https://api.github.com/repos/${OWNER}/${REPO}/releases/latest" \
        | grep -o '"tag_name": *"[^"]*"' \
        | head -1 \
        | sed 's/.*"tag_name": *"\(.*\)"/\1/')
    if [ -z "$VERSION" ]; then
        echo "Error: Failed to fetch latest version from GitHub API." >&2
        exit 1
    fi
fi
VERSION="${VERSION#v}"

echo "Installing ${NAME} v${VERSION} (${OS}-${ARCH} → ${TARGET})..."

# --- Download ---
ARCHIVE="${NAME}-${VERSION}-${TARGET}.${EXT}"
URL="https://github.com/${OWNER}/${REPO}/releases/download/v${VERSION}/${ARCHIVE}"

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

echo "Downloading ${URL}..."
curl -sSfL -o "${TMPDIR}/${ARCHIVE}" "$URL"

# --- Extract ---
echo "Extracting..."
case "$EXT" in
    tar.xz) tar -xJf "${TMPDIR}/${ARCHIVE}" -C "$TMPDIR" ;;
    zip)    unzip -q "${TMPDIR}/${ARCHIVE}" -d "$TMPDIR" ;;
esac

# --- Install ---
mkdir -p "$INSTALL_DIR"

cp "${TMPDIR}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
chmod +x "${INSTALL_DIR}/${BINARY}"

if [ -d "${TMPDIR}/fonts" ]; then
    rm -rf "${INSTALL_DIR}/fonts"
    cp -r "${TMPDIR}/fonts" "${INSTALL_DIR}/"
fi

echo "Installed to ${INSTALL_DIR}"

# --- PATH check ---
if ! echo "$PATH" | tr ':' '\n' | grep -qxF "$INSTALL_DIR"; then
    echo
    echo "NOTE: ${INSTALL_DIR} is not on your PATH."
    echo "Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
    echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    echo
    echo "Or set a custom install directory next time:"
    echo "  RSIDLET_INSTALL_DIR=/your/path bash install.sh"
fi

echo "Done! Run '${BINARY} --help' to get started."
