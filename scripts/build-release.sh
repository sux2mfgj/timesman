#!/bin/bash
# Cross-platform release build script for TimesMan
# Builds all binaries for all supported platforms

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get version from Cargo.toml
VERSION=$(grep "^version" timesman-server/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
echo -e "${BLUE}📦 Building TimesMan v$VERSION for all platforms${NC}"

# Create build directory
BUILD_DIR="target/releases"
mkdir -p $BUILD_DIR

# Define targets and their display names
declare -A TARGETS=(
    ["x86_64-unknown-linux-gnu"]="Linux x86_64"
    ["aarch64-unknown-linux-gnu"]="Linux ARM64"
    ["x86_64-pc-windows-gnu"]="Windows x86_64"
    ["x86_64-apple-darwin"]="macOS Intel"
    ["aarch64-apple-darwin"]="macOS Apple Silicon"
)

# Define which binaries to build
BINARIES=("timesman-server" "timesman-app" "timesman-tools")

# Function to build for a specific target
build_target() {
    local target=$1
    local display_name=$2
    
    echo -e "${YELLOW}🔨 Building for $display_name ($target)${NC}"
    
    # Create target-specific directory
    local target_dir="$BUILD_DIR/$target"
    mkdir -p "$target_dir"
    
    # Build each binary for this target
    for binary in "${BINARIES[@]}"; do
        echo -e "  📋 Building $binary..."
        
        # Special handling for timesman-app on non-GUI platforms
        if [[ "$binary" == "timesman-app" && "$target" == *"linux"* ]]; then
            echo -e "    ⚠️  GUI app may require additional system dependencies on Linux"
        fi
        
        # Build the binary
        if cargo build --release --target "$target" --bin "$binary" 2>/dev/null; then
            echo -e "    ✅ $binary built successfully"
            
            # Copy binary to target directory with appropriate extension
            local binary_name="$binary"
            if [[ "$target" == *"windows"* ]]; then
                binary_name="$binary.exe"
            fi
            
            cp "target/$target/release/$binary_name" "$target_dir/"
        else
            echo -e "    ${RED}❌ Failed to build $binary for $target${NC}"
            # Continue with other binaries instead of failing completely
        fi
    done
    
    # Copy additional files
    echo -e "  📄 Copying additional files..."
    cp README.md "$target_dir/" 2>/dev/null || true
    cp LICENSE "$target_dir/" 2>/dev/null || true
    cp -r docs "$target_dir/" 2>/dev/null || true
    cp timesman-server/config.toml "$target_dir/config.example.toml" 2>/dev/null || true
    
    # Create a basic README for the release
    cat > "$target_dir/README.txt" << EOF
TimesMan v$VERSION - $display_name

This package contains:
- timesman-server: Authentication-enabled gRPC/HTTP server
- timesman-app: GUI application for time tracking
- timesman-tools: CLI tools and TUI interface

Quick Start:
1. Copy config.example.toml to config.toml and edit as needed
2. Run ./timesman-server --config config.toml
3. Use ./timesman-app for GUI or ./timesman-tools for CLI

Documentation: See docs/ folder for complete guides
Authentication: See docs/AUTHENTICATION.md for setup instructions

For support, visit: https://github.com/your-repo/timesman
EOF
    
    echo -e "  ${GREEN}✅ $display_name build complete${NC}"
}

# Check if required targets are installed
echo -e "${BLUE}🔍 Checking installed targets...${NC}"
missing_targets=()
for target in "${!TARGETS[@]}"; do
    if ! rustup target list --installed | grep -q "$target"; then
        missing_targets+=("$target")
    fi
done

if [ ${#missing_targets[@]} -ne 0 ]; then
    echo -e "${RED}❌ Missing targets: ${missing_targets[*]}${NC}"
    echo -e "${YELLOW}Run './scripts/install-targets.sh' to install missing targets${NC}"
    exit 1
fi

# Clean previous builds
echo -e "${BLUE}🧹 Cleaning previous builds...${NC}"
cargo clean
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Build for each target
echo -e "${BLUE}🚀 Starting cross-platform builds...${NC}"
for target in "${!TARGETS[@]}"; do
    build_target "$target" "${TARGETS[$target]}"
    echo ""
done

# Create archives for distribution
echo -e "${BLUE}📦 Creating distribution archives...${NC}"
cd "$BUILD_DIR"

for target in "${!TARGETS[@]}"; do
    if [ -d "$target" ]; then
        echo -e "  📦 Creating archive for ${TARGETS[$target]}..."
        
        if [[ "$target" == *"windows"* ]]; then
            # Create ZIP for Windows
            zip -r "timesman-v$VERSION-$target.zip" "$target/"
        else
            # Create tar.gz for Unix-like systems
            tar -czf "timesman-v$VERSION-$target.tar.gz" "$target/"
        fi
        
        echo -e "    ${GREEN}✅ Archive created${NC}"
    fi
done

cd - > /dev/null

# Summary
echo -e "${GREEN}🎉 Cross-platform build complete!${NC}"
echo -e "${BLUE}📊 Build Summary:${NC}"
echo -e "   Version: $VERSION"
echo -e "   Build directory: $BUILD_DIR"
echo -e "   Platforms built: ${#TARGETS[@]}"

echo ""
echo -e "${BLUE}📦 Distribution files:${NC}"
ls -la "$BUILD_DIR"/*.{tar.gz,zip} 2>/dev/null || echo "   No archives created"

echo ""
echo -e "${YELLOW}🚀 Next steps:${NC}"
echo -e "   • Test binaries on target platforms"
echo -e "   • Run './scripts/package.sh' to create installers"
echo -e "   • Upload to GitHub releases or package repositories"