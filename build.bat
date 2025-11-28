@echo off
echo Vibe Rust Coder - Build Script
echo ================================

echo.
echo Cleaning previous build...
cargo clean

echo.
echo Building in release mode...
cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ================================
    echo Build successful!
    echo ================================
    echo.
    echo Run the application with:
    echo   cargo run --release
    echo.
    echo Or directly:
    echo   .\target\release\vibe_rust_coder.exe
    echo.
) else (
    echo.
    echo ================================
    echo Build failed!
    echo ================================
    echo.
    echo Try:
    echo   1. Close any running instances
    echo   2. Disable antivirus temporarily
    echo   3. Run this script again
    echo.
)

pause
