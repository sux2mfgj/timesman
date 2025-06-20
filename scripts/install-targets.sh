#!/bin/bash
# Install Rust cross-compilation targets for TimesMan
# Run this script once to set up cross-compilation environment

set -e

echo "🦀 Installing Rust cross-compilation targets for TimesMan..."

# Linux targets
echo "📦 Installing Linux targets..."
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Windows targets  
echo "🪟 Installing Windows targets..."
rustup target add x86_64-pc-windows-gnu
rustup target add aarch64-pc-windows-msvc

# macOS targets
echo "🍎 Installing macOS targets..."
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

echo "✅ All targets installed successfully!"

# Check if cross-compilation tools are available
echo ""
echo "🔧 Checking cross-compilation toolchains..."

# Check for Linux cross-compilers
if command -v x86_64-linux-gnu-gcc &> /dev/null; then
    echo "✅ Linux x86_64 cross-compiler found"
else
    echo "⚠️  Linux x86_64 cross-compiler not found. Install with:"
    echo "   Ubuntu/Debian: sudo apt install gcc-x86-64-linux-gnu"
    echo "   macOS: brew install FiloSottile/musl-cross/musl-cross"
fi

if command -v aarch64-linux-gnu-gcc &> /dev/null; then
    echo "✅ Linux ARM64 cross-compiler found"
else
    echo "⚠️  Linux ARM64 cross-compiler not found. Install with:"
    echo "   Ubuntu/Debian: sudo apt install gcc-aarch64-linux-gnu"
    echo "   macOS: brew install aarch64-unknown-linux-gnu"
fi

# Check for Windows cross-compilers
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "✅ Windows cross-compiler found"
else
    echo "⚠️  Windows cross-compiler not found. Install with:"
    echo "   Ubuntu/Debian: sudo apt install gcc-mingw-w64"
    echo "   macOS: brew install mingw-w64"
fi

echo ""
echo "🚀 Cross-compilation setup complete!"
echo "   Run './scripts/build-release.sh' to build for all platforms"