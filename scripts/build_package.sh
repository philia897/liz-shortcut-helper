#!/bin/bash

# Check if the package_name parameter is provided
if [ -z "$1" ]; then
    echo "Error: No package_dir provided."
    echo "Usage: ./scripts/build_package.sh package_dir"
    exit 1
fi

# Create the package directory
PACKAGE_DIR="$1"
mkdir -p "$PACKAGE_DIR"

# Copy the binaries to the package directory
cp ./target/release/liz "$PACKAGE_DIR/"
cp ./target/release/bluebird "$PACKAGE_DIR/"

# Copy the shell scripts, ignoring build_package.sh
find ./scripts -name "*.sh" ! -name "build_package.sh" -exec cp {} "$PACKAGE_DIR/" \;

# Copy config data dir
cp -r ./data "$PACKAGE_DIR/"

# Create a compressed tarball of the release package
tar -czvf "$PACKAGE_DIR.tar.gz" "$PACKAGE_DIR"

# Optional: Cleanup the package directory after compression
# rm -rf "$PACKAGE_DIR"

echo "Release package created: $PACKAGE_DIR.tar.gz"
