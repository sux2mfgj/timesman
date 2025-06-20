# TimesMan Packaging and Distribution

This document explains how to build and package TimesMan for distribution across Linux, Windows, and macOS platforms.

## Overview

TimesMan provides automated build and packaging scripts for creating distribution packages on all major platforms. The system includes:

- **Cross-compilation** for multiple architectures
- **Native package formats** for each platform
- **Automated CI/CD** via GitHub Actions
- **Container images** for easy deployment

## Quick Start

### Prerequisites

1. **Rust toolchain** (latest stable)
2. **Cross-compilation targets** (installed via script)
3. **Platform-specific tools** (see below)

### Build All Platforms

```bash
# 1. Install cross-compilation targets
./scripts/install-targets.sh

# 2. Build for all platforms
./scripts/build-release.sh

# 3. Create native packages
./scripts/package.sh
```

## Supported Platforms

| Platform | Architecture | Target Triple | Package Formats |
|----------|--------------|---------------|-----------------|
| **Linux** | x86_64 | `x86_64-unknown-linux-gnu` | AppImage, .deb, .rpm, .tar.gz |
| **Linux** | ARM64 | `aarch64-unknown-linux-gnu` | AppImage, .deb, .rpm, .tar.gz |
| **Windows** | x86_64 | `x86_64-pc-windows-gnu` | .msi, .exe (NSIS), .zip |
| **Windows** | x86_64 | `x86_64-pc-windows-msvc` | .msi, .exe (NSIS), .zip |
| **macOS** | Intel | `x86_64-apple-darwin` | .dmg, .pkg, .app bundle |
| **macOS** | Apple Silicon | `aarch64-apple-darwin` | .dmg, .pkg, .app bundle |

## Build Scripts

### install-targets.sh

Installs all required Rust compilation targets and checks for cross-compilation toolchains.

```bash
./scripts/install-targets.sh
```

**What it does:**
- Installs Rust targets for all supported platforms
- Checks for required cross-compilation tools
- Provides installation instructions for missing tools

### build-release.sh

Cross-compiles all TimesMan binaries for all supported platforms.

```bash
./scripts/build-release.sh
```

**What it builds:**
- `timesman-server`: gRPC/HTTP server with authentication
- `timesman-app`: GUI application (egui-based)
- `timesman-tools`: CLI tools and TUI interface

**Output:**
- `target/releases/`: Built binaries organized by target
- `target/releases/*.tar.gz|.zip`: Distribution archives

### package.sh

Creates native installation packages for each platform.

```bash
./scripts/package.sh
```

**Creates:**
- **Linux**: AppImage structure, Debian package structure
- **Windows**: NSIS installer script, portable ZIP
- **macOS**: App bundle, DMG creation script

## Platform-Specific Requirements

### Linux Cross-Compilation

Install cross-compilation tools:

```bash
# Ubuntu/Debian
sudo apt install gcc-x86-64-linux-gnu gcc-aarch64-linux-gnu

# macOS
brew install FiloSottile/musl-cross/musl-cross
brew install aarch64-unknown-linux-gnu
```

**Required system libraries:**
- `pkg-config`
- `libssl-dev`
- `libfontconfig1-dev` (for GUI app)
- `libfreetype6-dev` (for GUI app)

### Windows Cross-Compilation

Install MinGW-w64:

```bash
# Ubuntu/Debian
sudo apt install gcc-mingw-w64

# macOS
brew install mingw-w64
```

**Package creation tools:**
- **NSIS**: For creating Windows installers
- **WiX Toolset**: For MSI packages (future)

### macOS Cross-Compilation

Cross-compilation from Linux requires additional setup:

```bash
# Install osxcross toolchain
git clone https://github.com/tpoechtrager/osxcross
# Follow osxcross setup instructions
```

**Package creation tools:**
- `hdiutil`: DMG creation (macOS only)
- `pkgbuild`: PKG installer creation (macOS only)

## Package Formats

### Linux Packages

#### AppImage
- **Format**: Portable application bundle
- **Benefits**: Works on any Linux distribution
- **Creation**: `appimagetool TimesMan.AppDir`

#### Debian Package (.deb)
- **Format**: Debian/Ubuntu package
- **Benefits**: Integrated with apt package manager
- **Creation**: `dpkg-deb --build timesman-deb`

#### RPM Package
- **Format**: RedHat/Fedora package
- **Benefits**: Integrated with yum/dnf package manager
- **Creation**: `rpmbuild` (manual setup required)

### Windows Packages

#### NSIS Installer
- **Format**: Executable installer
- **Benefits**: Professional Windows installation experience
- **Creation**: `makensis timesman-installer.nsi`

#### MSI Installer
- **Format**: Windows Installer package
- **Benefits**: Group Policy deployment support
- **Creation**: WiX Toolset (future enhancement)

#### Portable ZIP
- **Format**: No-installation archive
- **Benefits**: No admin rights required
- **Creation**: Automated in package script

### macOS Packages

#### App Bundle (.app)
- **Format**: Native macOS application
- **Benefits**: Integrates with macOS launcher
- **Creation**: Automated in package script

#### DMG Disk Image
- **Format**: Mountable disk image
- **Benefits**: Standard macOS distribution format
- **Creation**: `./create-dmg.sh` (requires macOS)

#### PKG Installer
- **Format**: macOS installer package
- **Benefits**: Integrated with macOS installer system
- **Creation**: `pkgbuild` (future enhancement)

## Automated CI/CD

### GitHub Actions

The project includes a comprehensive GitHub Actions workflow (`.github/workflows/release.yml`) that:

1. **Builds** all platforms automatically
2. **Tests** on multiple operating systems
3. **Creates** release artifacts
4. **Publishes** to GitHub Releases
5. **Builds** container images

### Triggering Releases

```bash
# Create and push a version tag
git tag v1.0.0
git push origin v1.0.0

# GitHub Actions will automatically:
# - Build for all platforms
# - Create GitHub release
# - Upload all artifacts
```

### Manual Workflow Dispatch

You can also trigger builds manually from the GitHub Actions tab with a custom version number.

## Container Deployment

### Docker

```bash
# Build container image
docker build -t timesman/server .

# Run with Docker Compose
docker-compose up -d
```

### Configuration

The container accepts these environment variables:

- `TIMESMAN_JWT_SECRET`: JWT signing secret (required)
- `TIMESMAN_LISTEN`: Server listen address (default: 0.0.0.0:50051)
- `TIMESMAN_TOKEN_EXPIRY_HOURS`: Token expiry time (default: 24)
- `RUST_LOG`: Logging level (default: info)

## Distribution Channels

### GitHub Releases

All builds are automatically uploaded to GitHub Releases when you push a version tag.

### Package Repositories

Future enhancements will include:

- **Homebrew**: macOS package manager
- **Chocolatey**: Windows package manager  
- **AUR**: Arch Linux user repository
- **Flathub**: Universal Linux packages

### Container Registries

- **Docker Hub**: `timesman/server`
- **GitHub Container Registry**: `ghcr.io/your-repo/timesman`

## Local Development

### Testing Cross-Compilation

```bash
# Test a specific target
cargo build --target x86_64-unknown-linux-gnu --bin timesman-server

# Test all binaries for a target
for bin in timesman-server timesman-app timesman-tools; do
  cargo build --target x86_64-pc-windows-gnu --bin $bin
done
```

### Testing Packages

1. **Extract** a built package
2. **Verify** all binaries work: `./timesman-server --version`
3. **Test** basic functionality
4. **Check** documentation is included

## Troubleshooting

### Common Issues

#### "Linker not found"
**Solution**: Install cross-compilation tools for target platform

#### "OpenSSL not found"
**Solution**: Install development headers or use vendored OpenSSL

#### "GUI app won't start on Linux"
**Solution**: Install GUI dependencies: `libfontconfig1 libfreetype6`

#### "Permission denied" on scripts
**Solution**: Make scripts executable: `chmod +x scripts/*.sh`

### Build Debugging

```bash
# Verbose cargo output
RUST_BACKTRACE=1 cargo build --target ... --verbose

# Check available targets
rustup target list --installed

# Verify toolchain
rustup show
```

## Security Considerations

### Code Signing

For production releases:

- **Windows**: Sign executables with Authenticode certificate
- **macOS**: Sign app bundles and notarize with Apple
- **Linux**: Sign packages with GPG keys

### Supply Chain Security

- All dependencies are locked in `Cargo.lock`
- Build reproducibility via consistent build environment
- Container images use minimal base images
- Regular security audits via `cargo audit`

## Performance Optimizations

### Binary Size

The build configuration includes optimizations for smaller binaries:

```toml
[profile.release]
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization  
panic = "abort"     # Smaller binaries
strip = true        # Remove debug symbols
```

### Cross-Compilation Speed

- Use `sccache` for faster builds
- Cache cargo registry and build artifacts
- Parallel builds in CI/CD

## Future Enhancements

### Planned Features

- **Automatic updates**: Built-in update mechanism
- **Digital signatures**: Code signing for all platforms
- **Package repositories**: Submit to official repositories
- **Universal binaries**: macOS universal binaries (Intel + Apple Silicon)
- **Static linking**: Fully static Linux binaries

### Community Packages

We welcome community contributions for:
- Distribution-specific packages (Snap, Flatpak, etc.)
- Package manager integrations
- Alternative build systems

---

For questions about packaging, see the [main documentation](./README.md) or create an issue in the project repository.