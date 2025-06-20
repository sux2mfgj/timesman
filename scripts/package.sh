#!/bin/bash
# Package generation script for TimesMan
# Creates native installer packages for each platform

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get version from Cargo.toml
VERSION=$(grep "^version" timesman-server/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
echo -e "${BLUE}📦 Creating installation packages for TimesMan v$VERSION${NC}"

BUILD_DIR="target/releases"
PACKAGE_DIR="target/packages"
mkdir -p "$PACKAGE_DIR"

# Check if builds exist
if [ ! -d "$BUILD_DIR" ]; then
    echo -e "${RED}❌ No builds found. Run './scripts/build-release.sh' first${NC}"
    exit 1
fi

# Function to create Linux packages
create_linux_packages() {
    echo -e "${YELLOW}🐧 Creating Linux packages...${NC}"
    
    # AppImage (Universal Linux)
    if [ -d "$BUILD_DIR/x86_64-unknown-linux-gnu" ]; then
        echo -e "  📦 Creating AppImage..."
        
        # Create AppDir structure
        APPDIR="$PACKAGE_DIR/TimesMan.AppDir"
        mkdir -p "$APPDIR/usr/bin"
        mkdir -p "$APPDIR/usr/share/applications"
        mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"
        mkdir -p "$APPDIR/usr/share/doc/timesman"
        
        # Copy binaries
        cp "$BUILD_DIR/x86_64-unknown-linux-gnu"/timesman-* "$APPDIR/usr/bin/"
        
        # Copy documentation
        cp -r docs "$APPDIR/usr/share/doc/timesman/" 2>/dev/null || true
        cp README.md "$APPDIR/usr/share/doc/timesman/" 2>/dev/null || true
        cp LICENSE "$APPDIR/usr/share/doc/timesman/" 2>/dev/null || true
        
        # Create desktop file
        cat > "$APPDIR/usr/share/applications/timesman.desktop" << EOF
[Desktop Entry]
Type=Application
Name=TimesMan
Comment=Time tracking application with authentication
Exec=timesman-app
Icon=timesman
Categories=Office;ProjectManagement;
StartupNotify=true
EOF
        
        # Create AppRun script
        cat > "$APPDIR/AppRun" << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin/:${PATH}"
exec "${HERE}/usr/bin/timesman-app" "$@"
EOF
        chmod +x "$APPDIR/AppRun"
        
        # Create .desktop file in root
        cp "$APPDIR/usr/share/applications/timesman.desktop" "$APPDIR/"
        
        # Create placeholder icon (in real scenario, you'd have an actual icon)
        touch "$APPDIR/timesman.png"
        cp "$APPDIR/timesman.png" "$APPDIR/usr/share/icons/hicolor/256x256/apps/"
        
        echo -e "    ${GREEN}✅ AppImage structure created${NC}"
        echo -e "    💡 Note: Use appimagetool to create final AppImage"
    fi
    
    # Debian package
    if [ -d "$BUILD_DIR/x86_64-unknown-linux-gnu" ]; then
        echo -e "  📦 Creating Debian package structure..."
        
        DEB_DIR="$PACKAGE_DIR/timesman-deb"
        mkdir -p "$DEB_DIR/DEBIAN"
        mkdir -p "$DEB_DIR/usr/bin"
        mkdir -p "$DEB_DIR/usr/share/doc/timesman"
        mkdir -p "$DEB_DIR/usr/share/applications"
        mkdir -p "$DEB_DIR/etc/timesman"
        
        # Copy binaries
        cp "$BUILD_DIR/x86_64-unknown-linux-gnu"/timesman-* "$DEB_DIR/usr/bin/"
        
        # Copy documentation
        cp -r docs "$DEB_DIR/usr/share/doc/timesman/" 2>/dev/null || true
        cp README.md "$DEB_DIR/usr/share/doc/timesman/" 2>/dev/null || true
        cp LICENSE "$DEB_DIR/usr/share/doc/timesman/copyright" 2>/dev/null || true
        
        # Copy config
        cp timesman-server/config.toml "$DEB_DIR/etc/timesman/config.toml"
        
        # Create control file
        cat > "$DEB_DIR/DEBIAN/control" << EOF
Package: timesman
Version: $VERSION
Section: utils
Priority: optional
Architecture: amd64
Depends: libc6 (>= 2.17), libssl3 (>= 3.0.0)
Maintainer: TimesMan Team <team@timesman.dev>
Description: Time tracking application with authentication
 TimesMan is a comprehensive time tracking solution with JWT authentication,
 gRPC/HTTP API, and multiple client interfaces including GUI, CLI, and TUI.
 .
 Features:
  - Secure JWT-based authentication
  - Role-based access control
  - gRPC and HTTP APIs
  - Cross-platform GUI application
  - Command-line and TUI interfaces
  - Multiple storage backends
EOF
        
        # Create postinst script
        cat > "$DEB_DIR/DEBIAN/postinst" << 'EOF'
#!/bin/bash
set -e

# Create timesman user for running the service
if ! id "timesman" &>/dev/null; then
    useradd --system --home /var/lib/timesman --shell /bin/false timesman
fi

# Create directories
mkdir -p /var/lib/timesman
mkdir -p /var/log/timesman
chown timesman:timesman /var/lib/timesman
chown timesman:timesman /var/log/timesman

# Set permissions
chmod 755 /usr/bin/timesman-*

echo "TimesMan installed successfully!"
echo "Configure /etc/timesman/config.toml and start the service:"
echo "  sudo systemctl enable timesman-server"
echo "  sudo systemctl start timesman-server"
EOF
        chmod 755 "$DEB_DIR/DEBIAN/postinst"
        
        echo -e "    ${GREEN}✅ Debian package structure created${NC}"
        echo -e "    💡 Note: Use 'dpkg-deb --build' to create final .deb"
    fi
}

# Function to create Windows packages
create_windows_packages() {
    echo -e "${YELLOW}🪟 Creating Windows packages...${NC}"
    
    if [ -d "$BUILD_DIR/x86_64-pc-windows-gnu" ]; then
        # NSIS installer script
        echo -e "  📦 Creating NSIS installer script..."
        
        cat > "$PACKAGE_DIR/timesman-installer.nsi" << EOF
; TimesMan NSIS Installer Script
!define APPNAME "TimesMan"
!define APPVERSION "$VERSION"
!define APPEXE "timesman-app.exe"

Name "\${APPNAME}"
OutFile "TimesMan-v\${APPVERSION}-Windows-x64-Installer.exe"
InstallDir "\$PROGRAMFILES64\\\${APPNAME}"
RequestExecutionLevel admin

Page directory
Page instfiles

Section "Install"
    SetOutPath \$INSTDIR
    
    ; Copy binaries
    File "$BUILD_DIR/x86_64-pc-windows-gnu/timesman-server.exe"
    File "$BUILD_DIR/x86_64-pc-windows-gnu/timesman-app.exe"
    File "$BUILD_DIR/x86_64-pc-windows-gnu/timesman-tools.exe"
    
    ; Copy documentation
    File /r "docs"
    File "README.md"
    File "LICENSE"
    File /oname=config.example.toml "timesman-server/config.toml"
    
    ; Create start menu shortcuts
    CreateDirectory "\$SMPROGRAMS\\\${APPNAME}"
    CreateShortCut "\$SMPROGRAMS\\\${APPNAME}\\\${APPNAME}.lnk" "\$INSTDIR\\\${APPEXE}"
    CreateShortCut "\$SMPROGRAMS\\\${APPNAME}\\TimesMan Server.lnk" "\$INSTDIR\\timesman-server.exe"
    CreateShortCut "\$SMPROGRAMS\\\${APPNAME}\\TimesMan Tools.lnk" "\$INSTDIR\\timesman-tools.exe"
    CreateShortCut "\$SMPROGRAMS\\\${APPNAME}\\Uninstall.lnk" "\$INSTDIR\\uninstall.exe"
    
    ; Create desktop shortcut
    CreateShortCut "\$DESKTOP\\\${APPNAME}.lnk" "\$INSTDIR\\\${APPEXE}"
    
    ; Create uninstaller
    WriteUninstaller "\$INSTDIR\\uninstall.exe"
    
    ; Registry entries
    WriteRegStr HKLM "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\\${APPNAME}" "DisplayName" "\${APPNAME}"
    WriteRegStr HKLM "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\\${APPNAME}" "UninstallString" "\$INSTDIR\\uninstall.exe"
    WriteRegStr HKLM "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\\${APPNAME}" "DisplayVersion" "\${APPVERSION}"
SectionEnd

Section "Uninstall"
    ; Remove files
    Delete "\$INSTDIR\\*.exe"
    RMDir /r "\$INSTDIR\\docs"
    Delete "\$INSTDIR\\README.md"
    Delete "\$INSTDIR\\LICENSE"
    Delete "\$INSTDIR\\config.example.toml"
    
    ; Remove shortcuts
    RMDir /r "\$SMPROGRAMS\\\${APPNAME}"
    Delete "\$DESKTOP\\\${APPNAME}.lnk"
    
    ; Remove registry entries
    DeleteRegKey HKLM "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\\${APPNAME}"
    
    ; Remove directory
    RMDir "\$INSTDIR"
SectionEnd
EOF
        
        echo -e "    ${GREEN}✅ NSIS installer script created${NC}"
        echo -e "    💡 Note: Use makensis to compile the installer"
        
        # Portable ZIP package
        echo -e "  📦 Creating portable ZIP package..."
        cd "$BUILD_DIR"
        if [ -d "x86_64-pc-windows-gnu" ]; then
            cp -r "x86_64-pc-windows-gnu" "../packages/TimesMan-v$VERSION-Windows-x64-Portable"
            cd "../packages"
            zip -r "TimesMan-v$VERSION-Windows-x64-Portable.zip" "TimesMan-v$VERSION-Windows-x64-Portable/"
            rm -rf "TimesMan-v$VERSION-Windows-x64-Portable"
            echo -e "    ${GREEN}✅ Portable ZIP created${NC}"
        fi
        cd - > /dev/null
    fi
}

# Function to create macOS packages
create_macos_packages() {
    echo -e "${YELLOW}🍎 Creating macOS packages...${NC}"
    
    # App Bundle for GUI application
    if [ -d "$BUILD_DIR/x86_64-apple-darwin" ] || [ -d "$BUILD_DIR/aarch64-apple-darwin" ]; then
        echo -e "  📦 Creating App Bundle..."
        
        APP_BUNDLE="$PACKAGE_DIR/TimesMan.app"
        mkdir -p "$APP_BUNDLE/Contents/MacOS"
        mkdir -p "$APP_BUNDLE/Contents/Resources"
        
        # Copy universal binary or Intel binary
        if [ -f "$BUILD_DIR/aarch64-apple-darwin/timesman-app" ]; then
            cp "$BUILD_DIR/aarch64-apple-darwin/timesman-app" "$APP_BUNDLE/Contents/MacOS/TimesMan"
        elif [ -f "$BUILD_DIR/x86_64-apple-darwin/timesman-app" ]; then
            cp "$BUILD_DIR/x86_64-apple-darwin/timesman-app" "$APP_BUNDLE/Contents/MacOS/TimesMan"
        fi
        
        # Create Info.plist
        cat > "$APP_BUNDLE/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>TimesMan</string>
    <key>CFBundleIdentifier</key>
    <string>com.timesman.app</string>
    <key>CFBundleName</key>
    <string>TimesMan</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>TMAN</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
</dict>
</plist>
EOF
        
        echo -e "    ${GREEN}✅ App Bundle created${NC}"
        
        # DMG script
        echo -e "  📦 Creating DMG creation script..."
        cat > "$PACKAGE_DIR/create-dmg.sh" << EOF
#!/bin/bash
# Script to create DMG for TimesMan

# Create temporary DMG directory
DMG_DIR="TimesMan-DMG"
mkdir -p "\$DMG_DIR"

# Copy app bundle
cp -r "TimesMan.app" "\$DMG_DIR/"

# Copy additional files
mkdir -p "\$DMG_DIR/Documentation"
cp -r ../docs "\$DMG_DIR/Documentation/" 2>/dev/null || true
cp ../README.md "\$DMG_DIR/" 2>/dev/null || true

# Create symbolic link to Applications
ln -s "/Applications" "\$DMG_DIR/Applications"

# Create DMG
hdiutil create -srcfolder "\$DMG_DIR" -format UDZO -imagekey zlib-level=9 "TimesMan-v$VERSION-macOS.dmg"

# Cleanup
rm -rf "\$DMG_DIR"

echo "DMG created: TimesMan-v$VERSION-macOS.dmg"
EOF
        chmod +x "$PACKAGE_DIR/create-dmg.sh"
        
        echo -e "    ${GREEN}✅ DMG creation script ready${NC}"
    fi
}

# Create packages for all platforms
create_linux_packages
create_windows_packages  
create_macos_packages

# Summary
echo -e "${GREEN}🎉 Package generation complete!${NC}"
echo -e "${BLUE}📊 Package Summary:${NC}"
echo -e "   Version: $VERSION"
echo -e "   Package directory: $PACKAGE_DIR"

echo ""
echo -e "${BLUE}📦 Created packages:${NC}"
ls -la "$PACKAGE_DIR/" 2>/dev/null

echo ""
echo -e "${YELLOW}🚀 Next steps:${NC}"
echo -e "   Linux:"
echo -e "     • Use appimagetool to create AppImage from AppDir"
echo -e "     • Use 'dpkg-deb --build' to create .deb package"
echo -e "   Windows:"
echo -e "     • Use makensis to compile NSIS installer"
echo -e "     • Portable ZIP is ready to distribute"
echo -e "   macOS:"
echo -e "     • Run create-dmg.sh to create DMG"
echo -e "     • Sign app bundle for distribution"

echo ""
echo -e "${BLUE}💡 Tools needed:${NC}"
echo -e "   • appimagetool (Linux AppImage)"
echo -e "   • dpkg-deb (Debian packages)"
echo -e "   • makensis (Windows NSIS installer)"
echo -e "   • hdiutil (macOS DMG creation)"