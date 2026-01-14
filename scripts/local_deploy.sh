#!/bin/bash
set -e

# 1. Check OS
OS="$(uname -s)"
if [ "$OS" != "Darwin" ]; then
    echo "Error: This script is only for macOS. Detected: $OS"
    exit 1
fi

echo "Detected macOS. Proceeding with installation..."

# 2. Update Skeleton
echo "Updating skeleton assets..."
./scripts/update_skeleton.sh

# 3. Build Release
echo "Building hymod in release mode..."
cargo build --release

# 4. Install to /usr/local/bin
TARGET_DIR="/usr/local/bin"
BINARY_PATH="target/release/hymod"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

echo "Installing hymod to $TARGET_DIR requires sudo..."

# Remove existing binary if it exists
if [ -f "$TARGET_DIR/hymod" ]; then
    echo "Removing existing hymod binary..."
    sudo rm "$TARGET_DIR/hymod"
fi

sudo cp "$BINARY_PATH" "$TARGET_DIR/hymod"

echo "Success! You can now run 'hymod' from anywhere."
