#!/bin/bash

# setup.sh - Helper script to configure the Hytale Mod Skeleton

set -e

echo "Welcome to the Hytale Mod Skeleton setup script!"
echo "This script will help you configure your new mod."
echo ""

# Function to prompt for input with a default value
prompt() {
    local prompt_text="$1"
    local default_val="$2"
    local var_name="$3"
    
    if [ -n "$default_val" ]; then
        read -p "$prompt_text [$default_val]: " input
        eval $var_name="\${input:-$default_val}"
    else
        read -p "$prompt_text: " input
        eval $var_name="\$input"
    fi
}

# --- Prompt for Values ---

prompt "Enter Mod Name (e.g. MyAwesomeMod)" "MyAwesomeMod" MOD_NAME
prompt "Enter Mod ID (no spaces, e.g. myawesomemod)" "$(echo "$MOD_NAME" | tr '[:upper:]' '[:lower:]' | tr -d ' ')" MOD_ID
prompt "Enter Group ID (e.g. com.myname)" "com.example" GROUP_ID
prompt "Enter Version" "1.0.0" VERSION
prompt "Enter Description" "A cool Hytale mod." DESCRIPTION
prompt "Enter Author" "$(whoami)" AUTHOR

# Construct Main Class Name
# Usually Group ID + . + Mod Name (Sanitized) + .Main, or just Group ID + .Main
# Let's assume standard conventions: <GROUP_ID>.<mod_id>.Main or just <GROUP_ID>.Main if user prefers.
# Let's ask for the main package.

DEFAULT_PACKAGE="$GROUP_ID.$(echo "$MOD_ID" | tr -d '-')"
prompt "Enter Main Package" "$DEFAULT_PACKAGE" PACKAGE

prompt "Enter Main Class Name" "Main" MAIN_CLASS_SIMPLE

MAIN_CLASS="$PACKAGE.$MAIN_CLASS_SIMPLE"

# Prompt for Device Username for local.properties
prompt "Enter Device Username (for Hytale path)" "$(whoami)" DEVICE_USERNAME

echo ""
echo "------------------------------------------------"
echo "Configuration summary:"
echo "Mod Name:    $MOD_NAME"
echo "Mod ID:      $MOD_ID"
echo "Group ID:    $GROUP_ID"
echo "Version:     $VERSION"
echo "Description: $DESCRIPTION"
echo "Author:      $AUTHOR"
echo "Package:     $PACKAGE"
echo "Main Class:  $MAIN_CLASS"
echo "Username:    $DEVICE_USERNAME"
echo "------------------------------------------------"
echo ""

read -p "Look good? (y/n) " confirm
if [[ $confirm != [yY] && $confirm != [yY][eE][sS] ]]; then
    echo "Aborted."
    exit 1
fi

echo "Applying changes..."

# --- Perform Replacements ---

# Helper for cross-platform compatible sed -i
sedi() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "$@"
    else
        sed -i "$@"
    fi
}

# 1. gradle.properties
sedi "s/<GROUP_ID>/$GROUP_ID/g" gradle.properties
sedi "s/<MOD_NAME>/$MOD_NAME/g" gradle.properties
sedi "s/<VERSION>/$VERSION/g" gradle.properties

# 2. settings.gradle
sedi "s/<MOD_ID>/$MOD_ID/g" settings.gradle

# 3. build.gradle
sedi "s/<MAIN_CLASS>/$MAIN_CLASS/g" build.gradle

# 4. manifest.json
sedi "s/<GROUP_ID>/$GROUP_ID/g" src/main/resources/manifest.json
sedi "s/<MOD_NAME>/$MOD_NAME/g" src/main/resources/manifest.json
sedi "s/<VERSION>/$VERSION/g" src/main/resources/manifest.json
sedi "s/<DESCRIPTION>/$DESCRIPTION/g" src/main/resources/manifest.json
sedi "s/<AUTHOR>/$Author/g" src/main/resources/manifest.json
# Note: Case sensitive replacement for Author in manifest might benefit from strict matching if variable name case differs,
# but here we use the variable $AUTHOR we captured.
# Wait, I used $Author in the sed command but the variable is AUTHOR.
sedi "s/<AUTHOR>/$AUTHOR/g" src/main/resources/manifest.json
sedi "s/<MAIN_CLASS>/$MAIN_CLASS/g" src/main/resources/manifest.json

# 5. Main.java and Directory Structure

# Current Main.java path
CURRENT_MAIN_JAVA="src/main/java/com/example/skeleton/Main.java"

# New Directory Structure
# Convert package dots to slashes
PACKAGE_PATH=$(echo "$PACKAGE" | tr '.' '/')
NEW_DIR="src/main/java/$PACKAGE_PATH"
NEW_MAIN_JAVA="$NEW_DIR/$MAIN_CLASS_SIMPLE.java"

echo "Moving source files..."
mkdir -p "$NEW_DIR"

if [ -f "$CURRENT_MAIN_JAVA" ]; then
    mv "$CURRENT_MAIN_JAVA" "$NEW_MAIN_JAVA"
    
    # Update Package Declaration in the new file
    sedi "s/package <PACKAGE>;/package $PACKAGE;/g" "$NEW_MAIN_JAVA"
    
    # Rename class if needed (simple regex, assumes public class Main)
    if [ "$MAIN_CLASS_SIMPLE" != "Main" ]; then
        sedi "s/public class Main/public class $MAIN_CLASS_SIMPLE/g" "$NEW_MAIN_JAVA"
        sedi "s/public Main()/public $MAIN_CLASS_SIMPLE()/g" "$NEW_MAIN_JAVA"
    fi
    
    # Cleanup old empty directories
    # Try to remove com/example/skeleton, then com/example, then com if empty
    rmdir src/main/java/com/example/skeleton 2>/dev/null || true
    rmdir src/main/java/com/example 2>/dev/null || true
    rmdir src/main/java/com 2>/dev/null || true
else
    echo "Warning: Could not find $CURRENT_MAIN_JAVA. Skipping Java file update."
fi

# 6. Setup local.properties
echo "Setting up local.properties..."
OS_TYPE=$(uname)
if [[ "$OS_TYPE" == "Darwin" ]]; then
    cp local.properties.mac local.properties
elif [[ "$OS_TYPE" == "Linux" ]]; then
    cp local.properties.linux local.properties
elif [[ "$OS_TYPE" == "CYGWIN"* || "$OS_TYPE" == "MINGW"* ]]; then
    cp local.properties.windows local.properties
    echo "Windows detected. Please verify local.properties paths manually if needed."
else
    # Fallback/Default
    echo "Could not detect OS. Using Mac template as default."
    cp local.properties.mac local.properties
fi

sedi "s/<USERNAME>/$DEVICE_USERNAME/g" local.properties

echo "Setup complete! You can now run './gradlew build' to build your mod."
