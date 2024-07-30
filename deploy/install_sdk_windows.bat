@echo off
setlocal

REM Define source directories
set SDK_NAME=e2esdk
set BUILD_DIR=target\release
set SOURCE_DIR=include

REM Define installation directories
set SDK_INSTALL_DIR=C:\Program Files\E2ESDK
set INCLUDE_DIR=%SDK_INSTALL_DIR%\include
set LIB_DIR=%SDK_INSTALL_DIR%\lib

REM Create installation directories
if not exist "%INCLUDE_DIR%" mkdir "%INCLUDE_DIR%"
if not exist "%LIB_DIR%" mkdir "%LIB_DIR%"

REM Copy the header files
echo Copying header files...
xcopy /Y /I "%SOURCE_DIR%\*.h" "%INCLUDE_DIR%"

REM Copy the shared library DLL to the install directory
echo Copying shared library...
copy "%BUILD_DIR%\%SDK_NAME%.dll" "%INSTALL_DIR%"

REM Optionally, add SDK to PATH, you need to set PATH permanently
echo Adding SDK to system PATH...
setx PATH "%PATH%;%LIB_DIR%"

echo E2E SDK installation complete.
endlocal
pause