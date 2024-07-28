#!/bin/bash

# Define variables
SDK_NAME=e2esdk
TARGETS=("aarch64-apple-ios" "x86_64-apple-ios")
BUILD_DIR=target/release
INSTALL_DIR=~/ios_sdk/

# Ensure the install directory exists
mkdir -p "$INSTALL_DIR"

# Deploy the Rust SDK for all iOS targets
for TARGET in "${TARGETS[@]}"; do
    # cargo build --target $TARGET --release
    cp "$BUILD_DIR/$TARGET/lib${SDK_NAME}.a" "$INSTALL_DIR/"
done

# Copy the header file
cp include/e2e_sdk.h "$INSTALL_DIR/"

echo "SDK installation complete for iOS."