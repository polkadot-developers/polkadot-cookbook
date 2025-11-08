# Polkadot Cookbook CLI Installer for Windows
# https://github.com/polkadot-developers/polkadot-cookbook

$ErrorActionPreference = "Stop"

# Configuration
$REPO = "polkadot-developers/polkadot-cookbook"
$BINARY_NAME = "dot.exe"
$INSTALL_DIR = "$env:LOCALAPPDATA\Programs\polkadot-cookbook"

# Colors for output
function Write-Pink {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Magenta
}

function Write-Cyan {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Cyan
}

function Write-Green {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Green
}

function Write-Yellow {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Yellow
}

function Write-Red {
    param([string]$Message)
    Write-Host $Message -ForegroundColor Red
}

# Main installation
function Install-DotCli {
    Write-Pink "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Write-Pink "  Polkadot Cookbook CLI Installer"
    Write-Pink "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Write-Host ""

    # Detect architecture
    $arch = if ([Environment]::Is64BitOperatingSystem) { "amd64" } else { "x86" }

    Write-Cyan "→ Detected OS: Windows"
    Write-Cyan "→ Detected Architecture: $arch"
    Write-Host ""

    if ($arch -ne "amd64") {
        Write-Red "✗ Unsupported architecture: $arch"
        Write-Yellow "  Only 64-bit Windows is supported"
        Write-Yellow "  Please build from source:"
        Write-Host "  git clone https://github.com/$REPO.git"
        Write-Host "  cd polkadot-cookbook"
        Write-Host "  cargo build --release --bin dot"
        exit 1
    }

    $BINARY_ARCHIVE = "dot-windows-amd64.exe.zip"

    # Get latest CLI release (prefer cli-v* tags, fall back to v* tags)
    Write-Cyan "→ Fetching latest CLI release..."

    try {
        $releases = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases"

        # Try to find cli-v* release first
        $cliRelease = $releases | Where-Object { $_.tag_name -like "cli-v*" } | Select-Object -First 1

        if ($cliRelease) {
            $version = $cliRelease.tag_name -replace 'cli-v', ''
            $tagPrefix = "cli-v"
            Write-Green "✓ Latest CLI version: v$version (CLI-specific release)"
        } else {
            # Fall back to latest v* release
            $mainRelease = $releases | Where-Object { $_.tag_name -like "v*" -and $_.tag_name -notlike "*-*" } | Select-Object -First 1
            if ($mainRelease) {
                $version = $mainRelease.tag_name -replace 'v', ''
                $tagPrefix = "v"
                Write-Green "✓ Latest CLI version: v$version (main release)"
            } else {
                throw "No suitable release found"
            }
        }
    } catch {
        Write-Red "✗ Failed to fetch release information"
        Write-Host "Error: $_"
        exit 1
    }

    Write-Host ""

    # Download URL
    $downloadUrl = "https://github.com/$REPO/releases/download/${tagPrefix}${version}/$BINARY_ARCHIVE"
    Write-Cyan "→ Downloading from: $downloadUrl"

    # Create temp directory
    $tempDir = Join-Path $env:TEMP "polkadot-cookbook-install-$(Get-Random)"
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null

    try {
        # Download binary
        $zipPath = Join-Path $tempDir $BINARY_ARCHIVE
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing

        Write-Green "✓ Downloaded successfully"
        Write-Host ""

        # Extract binary
        Write-Cyan "→ Extracting binary..."
        Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

        # Create install directory
        if (-not (Test-Path $INSTALL_DIR)) {
            New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
        }

        # Install binary
        Write-Cyan "→ Installing to $INSTALL_DIR..."
        $sourceBinary = Join-Path $tempDir $BINARY_NAME
        $destBinary = Join-Path $INSTALL_DIR $BINARY_NAME

        # Stop if binary is running
        $processes = Get-Process -Name "dot" -ErrorAction SilentlyContinue
        if ($processes) {
            Write-Yellow "⚠ Stopping running 'dot' processes..."
            $processes | Stop-Process -Force
            Start-Sleep -Seconds 1
        }

        Copy-Item -Path $sourceBinary -Destination $destBinary -Force

        Write-Green "✓ Binary installed successfully"
        Write-Host ""

        # Add to PATH if not already there
        $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($userPath -notlike "*$INSTALL_DIR*") {
            Write-Cyan "→ Adding to PATH..."
            [Environment]::SetEnvironmentVariable(
                "Path",
                "$userPath;$INSTALL_DIR",
                "User"
            )
            Write-Green "✓ Added to PATH (restart your terminal to use 'dot' command)"
            $needsRestart = $true
        } else {
            Write-Green "✓ Already in PATH"
            $needsRestart = $false
        }

        Write-Host ""

        # Verify installation
        Write-Green "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        Write-Green "  ✓ Installation complete!"
        Write-Green "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        Write-Host ""
        Write-Cyan "  Installed: v$version"
        Write-Cyan "  Location: $destBinary"
        Write-Host ""

        if ($needsRestart) {
            Write-Yellow "  ⚠ Please restart your terminal, then run:"
        } else {
            Write-Pink "  Get started:"
        }

        Write-Host "    dot --help          # Show all commands"
        Write-Host "    dot setup           # Setup development environment"
        Write-Host "    dot doctor          # Check your environment"
        Write-Host "    dot recipe create   # Create a new recipe"
        Write-Host ""

    } catch {
        Write-Red "✗ Installation failed"
        Write-Host "Error: $_"
        exit 1
    } finally {
        # Clean up
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

# Run installation
Install-DotCli
