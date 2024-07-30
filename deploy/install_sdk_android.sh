#!/bin/bash

# Check if the ANDROID_NDK path is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 /path/to/android-ndk"
    exit 1
fi

# Define variables
SDK_NAME=e2esdk
ANDROID_NDK="$1"
TARGET_ARCHS=("armv7-linux-androideabi" "aarch64-linux-android" "i686-linux-android" "x86_64-linux-android")
BUILD_DIR=target/release
INSTALL_DIR="${ANDROID_NDK}"/app/src/main/jniLibs/armeabi-v7a/

# Ensure the install directory exists
mkdir -p "$INSTALL_DIR"

# Build the Rust SDK for all Android targets
for TARGET in "${TARGET_ARCHS[@]}"; do
    # cargo ndk --target $TARGET --release
    cp "$BUILD_DIR/${TARGET}/lib${SDK_NAME}.so" "$INSTALL_DIR/"
done

# Copy the header file
cp include/e2esdk.h "$INSTALL_DIR/"

echo "SDK installation complete for Android."