#!/bin/bash
set -e


# Path to Cargo.toml
CARGO_TOML="Cargo.toml"
SRC_DIR="src" 
# Note: We probably want to track all source files in workspace, but starting with basics.
# The user's project structure has multiple crates. 
# We should probably hash the entire 'cli', 'core', 'features' directories if possible, 
# or just "." excluding target/ .git/ etc.
# Let's try to hash relevant directories: cli core features
DIRECTORIES="cli core features"

# We store the hash in target/ to keep the root clean and ensure it cleans with cargo clean
HASH_FILE="target/.source_hash"

if [ ! -d "target" ]; then
    mkdir -p target
fi

if [ ! -f "$CARGO_TOML" ]; then
    echo "Error: $CARGO_TOML not found in current directory."
    exit 1
fi

# Calculate current hash of source files
# We use find to list files, sort them (for stability), and hash content.
# Only looking at .rs and .toml files
CURRENT_HASH=$(find $DIRECTORIES -type f \( -name "*.rs" -o -name "*.toml" \) -not -path "*/target/*" | sort | xargs cat | shasum | awk '{print $1}')

if [ -f "$HASH_FILE" ]; then
    STORED_HASH=$(cat "$HASH_FILE")
else
    STORED_HASH=""
fi

if [ "$CURRENT_HASH" == "$STORED_HASH" ]; then
    # No changes in source, do not bump
    # echo "Source unchanged. Skipping version bump."
    exit 0
fi

# Source changed, proceed to bump

# Extract current version
# Assumes the first line starting with version = "..." is the package version
CURRENT_VERSION=$(grep -m 1 '^version = "[0-9]\+\.[0-9]\+\.[0-9]\+"' "$CARGO_TOML" | sed -E 's/version = "([0-9]+\.[0-9]+\.[0-9]+)"/\1/')

if [ -z "$CURRENT_VERSION" ]; then
    echo "Error: Could not find version in $CARGO_TOML"
    exit 1
fi

# Split version into components
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"

# Logic: Increment patch. If 20, reset to 0 and increment minor.
# If minor is 20, reset to 0 and increment major.
PATCH=$((PATCH + 1))

if [ "$PATCH" -ge 20 ]; then
    PATCH=0
    MINOR=$((MINOR + 1))
    
    if [ "$MINOR" -ge 20 ]; then
        MINOR=0
        MAJOR=$((MAJOR + 1))
    fi
fi

NEW_VERSION="$MAJOR.$MINOR.$PATCH"

echo "Bumping version: $CURRENT_VERSION -> $NEW_VERSION"

# Update Cargo.toml
if [[ "$OSTYPE" == "darwin"* ]]; then
    sed -i '' "1,/^version = \"$CURRENT_VERSION\"/s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
else
    sed -i "1,/^version = \"$CURRENT_VERSION\"/s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
fi

# Update Hash File
echo "$CURRENT_HASH" > "$HASH_FILE"
