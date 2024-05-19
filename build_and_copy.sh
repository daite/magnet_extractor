#!/bin/sh

# Build the Rust project in release mode
cargo build --release

# Define the binary name and the target directory
BINARY_NAME="magnet_extractor"
TARGET_DIR="/usr/local/bin"

# Copy the binary to the target directory
cp "target/release/$BINARY_NAME" "$TARGET_DIR"

echo "Binary has been built and copied to $TARGET_DIR"
