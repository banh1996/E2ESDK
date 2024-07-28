@echo off
setlocal

:: Define variables
set SDK_NAME=e2esdk
set BUILD_DIR=target\release
set INSTALL_DIR=C:\Program Files\MySDK

:: Ensure the install directory exists
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

:: Build the Rust SDK
@REM cargo build --release

:: Copy the DLL to the install directory
copy "%BUILD_DIR%\%SDK_NAME%.dll" "%INSTALL_DIR%"

:: Copy the header file to the install directory
copy "include\e2e_sdk.h" "%INSTALL_DIR%"

echo SDK installation complete for Windows.
endlocal