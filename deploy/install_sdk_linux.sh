#!/bin/bash

# Define variables
SDK_NAME=e2esdk
BUILD_DIR=target/release
INSTALL_DIR=/usr/local/lib
HEADER_DIR=/usr/local/include

# Ensure the install directory exists
mkdir -p "$INSTALL_DIR"
mkdir -p "$HEADER_DIR"

# Build the Rust SDK
# cargo build --release

# Copy the shared library and header file
cp "$BUILD_DIR/lib${SDK_NAME}.so" "$INSTALL_DIR/"
cp include/e2esdk.h "$HEADER_DIR/"

echo "SDK installation complete for Linux."