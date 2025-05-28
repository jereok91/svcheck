#!/bin/bash

set -e

RELEASE="latest"
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin) OS="macos" ;;
  Linux) OS="linux" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64) ARCH="x64" ;;
  arm64 | aarch64) ARCH="arm64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case "${OS}" in
    Linux)
        case "${ARCH}" in
            x86_64 ) TARGET="x86_64-unknown-linux-gnu" ;;
            arm64 | aarch64) TARGET="aarch64-unknown-linux-gnu";;
            *) echo "Arquitectura no soportada: ${ARCH} ${ARCH}" && exit 1 ;;
        esac
        ;;
    Darwin)
        case "${ARCH}" in
            x86_64) TARGET="x86_64-apple-darwin" ;;
            arm64 | aarch64) TARGET="aarch64-apple-darwin" ;;
            *) echo "Arquitectura no soportada: ${ARCH} ${ARCH}" && exit 1 ;;
        esac
        ;;
    *)
        echo "Sistema operativo no soportado: ${OS}"
        exit 1
        ;;
esac

INSTALL_DIR="$HOME/.local/bin"
BIN_NAME="svcheck"
FILENAME="${TARGET}"

echo "Installing svcheck to $INSTALL_DIR ..."

mkdir -p "$INSTALL_DIR"

URL="https://github.com/jereok91/svcheck/raw/refs/heads/main/dist/${TARGET}/${filename}.tar.gz"

TMP_DIR=$(mktemp -d)
curl -fsSL "$URL" -o "$TMP_DIR/svcheck.tar.gz"

tar -xzf "$TMP_DIR/svcheck.tar.gz" -C "$TMP_DIR"
mv "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

echo "✅ svcheck installed at $INSTALL_DIR/$BIN_NAME"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo ""
  echo "⚠️  $INSTALL_DIR is not in your PATH."
  echo "Add this line to your shell config (e.g., ~/.bashrc, ~/.zshrc):"
  echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

rm -rf "$TMP_DIR"

