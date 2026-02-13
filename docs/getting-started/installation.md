---
layout: doc
title: "Installation Guide"
---

# Installation Guide

This guide will help you install the `dot` CLI tool for creating and managing Polkadot Cookbook recipes.

## Choose Your Installation Method

- **[Option 1: Install from Release](#option-1-install-from-release-recommended)** (Recommended) - Pre-built binaries
- **[Option 2: Build from Source](#option-2-build-from-source)** - Build with cargo
- **[Option 3: Install via Cargo](#option-3-install-via-cargo-coming-soon)** (Coming Soon) - `cargo install`

---

## Option 1: Install from Release (Recommended)

Download pre-built binaries for your platform from the latest release.

### macOS (Intel)

```bash
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-macos-intel.tar.gz | tar xz
sudo mv dot /usr/local/bin/
```

### macOS (Apple Silicon)

```bash
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-macos-apple-silicon.tar.gz | tar xz
sudo mv dot /usr/local/bin/
```

### Linux (x86_64)

```bash
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-linux-amd64.tar.gz | tar xz
sudo mv dot /usr/local/bin/
```

### Linux (ARM64)

```bash
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-linux-arm64.tar.gz | tar xz
sudo mv dot /usr/local/bin/
```

### Windows

1. Download [dot-windows-amd64.exe.zip](https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-windows-amd64.exe.zip)
2. Extract the archive
3. Move `dot.exe` to a directory in your PATH

**Or using PowerShell:**
```powershell
# Download and extract
Invoke-WebRequest -Uri "https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-windows-amd64.exe.zip" -OutFile "dot.zip"
Expand-Archive -Path "dot.zip" -DestinationPath "."

# Move to user bin directory
New-Item -ItemType Directory -Force -Path "$env:LOCALAPPDATA\bin"
Move-Item -Path "dot.exe" -Destination "$env:LOCALAPPDATA\bin\dot.exe"

# Add to PATH (PowerShell profile)
$env:PATH += ";$env:LOCALAPPDATA\bin"
```

### Verify Installation

```bash
dot --version
```

You should see output like: `dot 0.2.0`

---

## Option 2: Build from Source

Build the latest version from the repository.

### Prerequisites

- **Rust** - Latest stable version
- **Git** - Any recent version

### Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify Rust installation:

```bash
rustc --version
cargo --version
```

### Build the CLI

```bash
# Clone the repository
git clone https://github.com/polkadot-developers/polkadot-cookbook.git
cd polkadot-cookbook

# Build the CLI
cargo build --release

# The binary is available at:
# target/release/dot
```

### Add to PATH

**Option A: Symlink to /usr/local/bin**

```bash
sudo ln -s "$(pwd)/target/release/dot" /usr/local/bin/dot
```

**Option B: Add to PATH in shell profile**

```bash
# Add to ~/.bashrc, ~/.zshrc, or ~/.profile
export PATH="$PATH:/path/to/polkadot-cookbook/target/release"
```

**Option C: Install globally**

```bash
cargo install --path ./cli
```

### Verify Installation

```bash
dot --version
```

---

## Option 3: Install via Cargo (Coming Soon)

In the future, you'll be able to install directly from crates.io:

```bash
cargo install polkadot-cookbook-cli
```

This feature is planned for a future release.

---

## Next Steps

Now that you have the CLI installed:

1. **[Create Your First Project](first-project.md)** - Follow the tutorial
2. **[Read the CLI Reference](../developers/cli-reference.md)** - Learn all commands
3. **[Join the Community](https://github.com/polkadot-developers/polkadot-cookbook/blob/master/CONTRIBUTING.md)** - Start contributing

---

## Troubleshooting

### Command Not Found

**Symptom:** `dot: command not found` after installation

**Cause:** Binary not in PATH or not executable

**Solutions:**

```bash
# Check if dot is in PATH
which dot

# If not found, add directory to PATH
export PATH="$PATH:/usr/local/bin"

# Or verify binary location
ls -la /usr/local/bin/dot

# Make sure it's executable
chmod +x /usr/local/bin/dot
```

### Permission Denied (macOS)

**Symptom:** macOS blocks the binary with security warning

**Cause:** macOS Gatekeeper security

**Solution:**

1. Go to System Preferences > Security & Privacy
2. Click "Allow Anyway" next to the blocked message
3. Or remove quarantine attribute:
   ```bash
   xattr -d com.apple.quarantine /usr/local/bin/dot
   ```

### Build Fails

**Symptom:** `cargo build` fails with errors

**Causes & Solutions:**

**Rust version too old:**
```bash
rustup update stable
```

**Missing dependencies:**
```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora
sudo dnf install gcc pkg-config openssl-devel
```

**Disk space:**
```bash
# Check available space
df -h

# Clean cargo cache if needed
cargo clean
```

### Slow Build

**Symptom:** Build takes a very long time

**Solution:** Use release mode or enable parallel builds

```bash
# Use all CPU cores (default behavior)
cargo build --release

# Or specify job count
cargo build --release -j 8
```

### Windows: Missing DLL

**Symptom:** `dot.exe` fails with missing DLL error

**Cause:** Missing Visual C++ redistributables

**Solution:**

Download and install [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist)

---

## Platform-Specific Notes

### macOS

- **Homebrew formula** (planned): `brew install polkadot-cookbook`
- **Gatekeeper**: First run may require security approval
- **ARM/Intel**: Download the correct version for your chip

### Linux

- **Package managers** (planned): apt, dnf, pacman packages
- **AppImage** (planned): Portable application
- **Permissions**: May need `sudo` for `/usr/local/bin`

### Windows

- **PowerShell vs CMD**: Examples use PowerShell
- **PATH**: Requires manual PATH configuration
- **WSL**: Can also use Linux binaries in WSL

---

## Updating

### Update Release Binary

Download the latest release and replace the existing binary:

```bash
# macOS/Linux
curl -L https://github.com/polkadot-developers/polkadot-cookbook/releases/latest/download/dot-[PLATFORM].tar.gz | tar xz
sudo mv dot /usr/local/bin/
```

### Update from Source

```bash
cd polkadot-cookbook
git pull origin master
cargo build --release
```

### Check for Updates

```bash
dot --version
```

Compare with latest release: https://github.com/polkadot-developers/polkadot-cookbook/releases/latest

---

## Uninstallation

### Remove Binary

```bash
# If installed to /usr/local/bin
sudo rm /usr/local/bin/dot

# If installed via cargo
cargo uninstall polkadot-cookbook-cli
```

### Remove Configuration (Optional)

```bash
# Remove config directory (when implemented)
rm -rf ~/.config/polkadot-cookbook
```

---

## Related Documentation

- [First Project Tutorial](first-project.md) - Create your first project
- [CLI Reference](../developers/cli-reference.md) - Complete command reference
- [Contributing Guide](https://github.com/polkadot-developers/polkadot-cookbook/blob/master/CONTRIBUTING.md) - Start contributing

---

[← Back to Getting Started](README.md)
