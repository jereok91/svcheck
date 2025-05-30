#!/bin/bash

set -e

RELEASE="latest"
OS="$(uname -s)"
ARCH="$(uname -m)"

OLD_BIN_PATH="$HOME/.local/bin/svcheck"
if [ -f "$OLD_BIN_PATH" ]; then
    echo "⚠️  Found old version of $BIN_NAME at $OLD_BIN_PATH. Removing..."
    rm -f "$OLD_BIN_PATH" || {
        echo "❌ Failed to remove old version. Check permissions?"
        exit 1
    }
    echo "♻️  Old version removed."
fi

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

case "$OS" in
    Linux | linux)
        case "$ARCH" in
            x64) TARGET="x86_64-unknown-linux-gnu" ;;
            arm64 | aarch64) TARGET="aarch64-unknown-linux-gnu";;
            *) echo "Arquitectura no soportada: ${OS} ${ARCH}" && exit 1 ;;
        esac
        ;;
    Darwin | macos)
        case "$ARCH" in
            x64) TARGET="x86_64-apple-darwin" ;;
            arm64 | aarch64) TARGET="aarch64-apple-darwin" ;;
            *) echo "Arquitectura no soportada: ${OS} ${ARCH}" && exit 1 ;;
        esac
        ;;
    *)
        echo "Sistema operativo no soportado: ${OS} ${ARCH}"
        exit 1
        ;;
esac

BIN_NAME="svcheck"
INSTALL_DIR="$HOME/.local/bin/$BIN_NAME"
FILENAME="${TARGET}"

echo "📦 Installing $BIN_NAME to $INSTALL_DIR ..."

mkdir -p "$INSTALL_DIR" || {
        echo "❌ Failed create $INSTALL_DIR. Check permissions?"
        exit 1
    }


URL="https://github.com/jereok91/svcheck/raw/refs/heads/main/dist/${TARGET}/${FILENAME}.tar.gz"
echo "🔗 Descargando desde: $URL"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

curl -fsSL "$URL" -o "$TMP_DIR/svcheck.tar.gz" || {
    echo "❌ Error al descargar $BIN_NAME" >&2
    rm -rf "$TMP_DIR"
    exit 1
}

tar -xzf "$TMP_DIR/svcheck.tar.gz" -C "$TMP_DIR" || {
    echo "❌ Error al extraer el archivo" >&2
    rm -rf "$TMP_DIR"
    exit 1
}
mv "$TMP_DIR/$FILENAME/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME" || {
    echo "❌ Error al mover el binario" >&2
    rm -rf "$TMP_DIR"
    exit 1
}
chmod +x "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR"

echo "✅ svcheck installed at $INSTALL_DIR/$BIN_NAME"

# --- Añadir PATH si no está configurado ---
SHELL_CONFIG=""
if [ -f "$HOME/.zshrc" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -f "$HOME/.bashrc" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
elif [ -f "$HOME/.profile" ]; then
    SHELL_CONFIG="$HOME/.profile"
fi

PATH_STRING="export PATH=\"$INSTALL_DIR:\$PATH\""
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo "🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧🚧"
  echo ""
  echo "⚠️  $INSTALL_DIR no está en tu PATH."
  if [ -n "$SHELL_CONFIG" ]; then
    if grep -qF "$PATH_STRING" "$SHELL_CONFIG"; then
        echo "ℹ️  El PATH ya está configurado en $SHELL_CONFIG (pero no está cargado en este terminal)."
        echo "    Ejecuta 'source $SHELL_CONFIG' o reinicia tu terminal."
    else
        echo "🔧 Añadiendo PATH a $SHELL_CONFIG..."
        echo -e "\n# Añadido por el instalador de $BIN_NAME\n$PATH_STRING" >> "$SHELL_CONFIG"
        echo "✅ Hecho. Ejecuta 'source $SHELL_CONFIG' o reinicia tu terminal."
    fi
  else
    echo "❌ No se encontró ningún archivo de configuración del shell (.bashrc, .zshrc o .profile)."
    echo "Añade manualmente esta línea a tu configuración:"
    echo "Add this line to your shell config (e.g., ~/.bashrc, ~/.zshrc):"
    echo "$PATH_STRING"
  fi
fi


