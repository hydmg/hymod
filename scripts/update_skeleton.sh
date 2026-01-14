#!/bin/bash
set -e

# Directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# Root of the repo
REPO_ROOT="$SCRIPT_DIR/.."
# Target directory
ASSETS_DIR="$REPO_ROOT/assets"
# Target file
TARGET_FILE="$ASSETS_DIR/skeleton.zip"

mkdir -p "$ASSETS_DIR"

echo "Downloading skeleton..."
curl -L -o "$TARGET_FILE" https://github.com/hydmg/hymod-skeleton-mod/archive/refs/heads/template.zip

echo "Downloaded to $TARGET_FILE"
