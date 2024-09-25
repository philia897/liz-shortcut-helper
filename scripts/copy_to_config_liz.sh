#!/bin/bash
set -e

# Define the target directory
TARGET_DIR="$HOME/.config/liz"

# Check if the target directory exists, create it if not
if [ ! -d "$TARGET_DIR" ]; then
    echo "Directory $TARGET_DIR does not exist. Creating it..."
    mkdir -p "$TARGET_DIR"
fi

# Copy files from ./data/* to ~/.config/liz
echo "Copying files from ./data/* to $TARGET_DIR..."
cp -r ./data/* "$TARGET_DIR"

echo "Files copied successfully!"