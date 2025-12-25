# Zenith Installation Script for Windows (PowerShell)
#
# This script installs the Zenith binary for Windows.
# It automatically detects your architecture, downloads the appropriate binary,
# verifies checksums, and installs it to a directory in your PATH.

$ErrorActionPreference = "Stop"

# Default values
$Version = ""
$InstallDir = ""
$SkipChecksum = $false
$Proxy = ""
$OfflineMode = $false
$LocalFile = ""

Write-Host "=== Zenith Installation Script ===" -ForegroundColor Blue

# Print usage
function Show-Usage {
    Write-Host "Usage: .\install.ps1 [OPTIONS]" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Options:" -ForegroundColor Cyan
    Write-Host "  -v, --version VERSION    Install specific version (default: latest)"
    Write-Host "  -d, --dir DIR            Installation directory (default: auto-detect)"
    Write-Host "  -s, --skip-checksum      Skip checksum verification"
    Write-Host "  -p, --proxy URL          Use proxy for downloads"
    Write-Host "  -f, --file PATH          Install from local file (offline mode)"
    Write-Host "  -h, --help               Show this help message"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Cyan
    Write-Host "  .\install.ps1                           # Install latest version"
    Write-Host "  .\install.ps1 -v 1.0.0                  # Install version 1.0.0"
    Write-Host "  .\install.ps1 -d C:\Tools\zenith        # Install to custom directory"
    Write-Host "  .\install.ps1 -f .\zenith.exe          # Install from local file"
    Write-Host ""
    exit 0
}

# Parse command line arguments
for ($i = 0; $i -lt $args.Count; $i++) {
    $arg = $args[$i]
    switch -Regex ($arg) {
        "^(-v|--version)$" {
            if ($i + 1 -lt $args.Count) {
                $Version = $args[$i + 1]
                $i++
            } else {
                Write-Host "Error: --version requires a value" -ForegroundColor Red
                exit 1
            }
        }
        "^(-d|--dir)$" {
            if ($i + 1 -lt $args.Count) {
                $InstallDir = $args[$i + 1]
                $i++
            } else {
                Write-Host "Error: --dir requires a value" -ForegroundColor Red
                exit 1
            }
        }
        "^(-s|--skip-checksum)$" {
            $SkipChecksum = $true
        }
        "^(-p|--proxy)$" {
            if ($i + 1 -lt $args.Count) {
                $Proxy = $args[$i + 1]
                $i++
            } else {
                Write-Host "Error: --proxy requires a value" -ForegroundColor Red
                exit 1
            }
        }
        "^(-f|--file)$" {
            if ($i + 1 -lt $args.Count) {
                $LocalFile = $args[$i + 1]
                $OfflineMode = $true
                $i++
            } else {
                Write-Host "Error: --file requires a value" -ForegroundColor Red
                exit 1
            }
        }
        "^(-h|--help)$" {
            Show-Usage
        }
        default {
            Write-Host "Unknown option: $arg" -ForegroundColor Red
            Write-Host "Use --help for usage information" -ForegroundColor Yellow
            exit 1
        }
    }
}

Write-Host "Installing Zenith..." -ForegroundColor Yellow

# Detect architecture
$ARCH = $env:PROCESSOR_ARCHITECTURE
if ($ARCH -eq "AMD64") {
    $TARGET = "x86_64-pc-windows-msvc"
} elseif ($ARCH -eq "ARM64") {
    $TARGET = "aarch64-pc-windows-msvc"
} else {
    Write-Host "Unsupported architecture: $ARCH" -ForegroundColor Red
    exit 1
}

Write-Host "Detected architecture: $ARCH (target: $TARGET)" -ForegroundColor Yellow

# Determine installation directory
if ([string]::IsNullOrEmpty($InstallDir)) {
    $InstallDir = "$env:LOCALAPPDATA\Programs\zenith"
}

if (-not (Test-Path $InstallDir)) {
    try {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    } catch {
        Write-Host "Failed to create installation directory: $($_.Exception.Message)" -ForegroundColor Red
        exit 1
    }
}

Write-Host "Installing to: $InstallDir" -ForegroundColor Yellow

# Handle offline mode (local file installation)
if ($OfflineMode) {
    if (-not (Test-Path $LocalFile)) {
        Write-Host "Local file not found: $LocalFile" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "Installing from local file: $LocalFile" -ForegroundColor Yellow
    $binaryPath = $LocalFile
} else {
    # Online mode: download from GitHub
    # Get the latest release from GitHub
    Write-Host "Fetching release info..." -ForegroundColor Yellow
    
    # Set proxy if specified
    $webParams = @{}
    if (-not [string]::IsNullOrEmpty($Proxy)) {
        $webParams.Proxy = $Proxy
        Write-Host "Using proxy: $Proxy" -ForegroundColor Yellow
    }
    
    try {
        $releaseInfo = Invoke-RestMethod -Uri "https://api.github.com/repos/user/zenith/releases/latest" -TimeoutSec 30 @webParams
        if ([string]::IsNullOrEmpty($Version)) {
            $tagName = $releaseInfo.tag_name
        } else {
            $tagName = "v$Version"
        }
    } catch {
        Write-Host "Could not fetch release info: $($_.Exception.Message)" -ForegroundColor Red
        Write-Host "Please check your network connection or use -f option for offline installation" -ForegroundColor Yellow
        exit 1
    }

    if ([string]::IsNullOrEmpty($tagName)) {
        Write-Host "Could not fetch release info" -ForegroundColor Red
        exit 1
    }

    Write-Host "Version: $tagName" -ForegroundColor Yellow

    # Construct the binary URL
    $binaryUrl = "https://github.com/user/zenith/releases/download/${tagName}/zenith-${TARGET}.zip"
    $tempPath = "$env:TEMP\zenith-$tagName-$TARGET.zip"

    Write-Host "Downloading binary from: $binaryUrl" -ForegroundColor Yellow

    try {
        Invoke-WebRequest -Uri $binaryUrl -OutFile $tempPath -TimeoutSec 60 @webParams
    } catch {
        Write-Host "Failed to download binary: $($_.Exception.Message)" -ForegroundColor Red
        Write-Host "Please check your network connection or use -f option for offline installation" -ForegroundColor Yellow
        exit 1
    }

    # Verify the downloaded file exists and is not empty
    if (-not (Test-Path $tempPath)) {
        Write-Host "Downloaded file not found" -ForegroundColor Red
        exit 1
    }

    $fileSize = (Get-Item $tempPath).Length
    if ($fileSize -eq 0) {
        Write-Host "Downloaded file is empty" -ForegroundColor Red
        exit 1
    }

    Write-Host "Downloaded file size: $fileSize bytes" -ForegroundColor Yellow

    # Download and verify checksum if not skipped
    if (-not $SkipChecksum) {
        $checksumUrl = "https://github.com/user/zenith/releases/download/${tagName}/checksums.txt"
        $checksumPath = "$env:TEMP\checksums-$tagName.txt"
        
        Write-Host "Downloading checksums from: $checksumUrl" -ForegroundColor Yellow
        try {
            Invoke-WebRequest -Uri $checksumUrl -OutFile $checksumPath -TimeoutSec 30 @webParams
            
            # Calculate SHA256 hash of downloaded file
            $calculatedHash = (Get-FileHash -Path $tempPath -Algorithm SHA256).Hash.ToLower()
            
            # Extract expected hash from checksums file
            $expectedHash = (Select-String -Path $checksumPath -Pattern "zenith-${TARGET}.zip").Line.Split(' ')[0]
            
            if ($expectedHash) {
                if ($calculatedHash -eq $expectedHash) {
                    Write-Host "Checksum verified: $calculatedHash" -ForegroundColor Green
                } else {
                    Write-Host "Checksum mismatch!" -ForegroundColor Red
                    Write-Host "Expected: $expectedHash"
                    Write-Host "Calculated: $calculatedHash"
                    Write-Host "The downloaded file may be corrupted or tampered with."
                    exit 1
                }
            } else {
                Write-Host "Checksum not found for this platform in checksums file" -ForegroundColor Yellow
            }
        } catch {
            Write-Host "Could not download or verify checksums: $($_.Exception.Message)" -ForegroundColor Yellow
        }
    } else {
        Write-Host "Checksum verification skipped" -ForegroundColor Yellow
    }

    # Extract the binary
    $extractDir = "$env:TEMP\zenith-$tagName-$TARGET"
    if (Test-Path $extractDir) {
        Remove-Item -Path $extractDir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $extractDir -Force | Out-Null

    try {
        Expand-Archive -Path $tempPath -DestinationPath $extractDir -Force
    } catch {
        Write-Host "Failed to extract binary: $($_.Exception.Message)" -ForegroundColor Red
        exit 1
    }

    # Find the actual binary in the extracted content
    $binaryPath = Get-ChildItem -Path $extractDir -Filter "*.exe" | Select-Object -First 1 -ExpandProperty FullName

    if ([string]::IsNullOrEmpty($binaryPath) -or -not (Test-Path $binaryPath)) {
        Write-Host "Binary not found in extracted content" -ForegroundColor Red
        Get-ChildItem -Path $extractDir
        exit 1
    }
}

# Move the binary to the installation directory
$installPath = "$InstallDir\zenith.exe"
try {
    Copy-Item -Path $binaryPath -Destination $installPath -Force
    Write-Host "Successfully installed Zenith to $installPath" -ForegroundColor Green
    
    # Add to PATH if not already there
    $currentPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$InstallDir*") {
        $newPath = $currentPath + ";$InstallDir"
        [System.Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        Write-Host "Added Zenith to PATH" -ForegroundColor Green
        Write-Host "You may need to restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
    }
    
    # Try to get version to confirm installation
    try {
        $versionOutput = & $installPath --version 2>$null
        if ($versionOutput) {
            Write-Host "Zenith version: $versionOutput" -ForegroundColor Green
        }
    } catch {
        Write-Host "Could not verify installation: $($_.Exception.Message)" -ForegroundColor Yellow
    }
    
    Write-Host "Installation complete!" -ForegroundColor Green
    Write-Host "Run 'zenith --help' to get started." -ForegroundColor Green
} catch {
    Write-Host "Failed to install binary: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
} finally {
    # Cleanup temporary files
    if (Test-Path $tempPath) {
        Remove-Item -Path $tempPath -Force
    }
    if (Test-Path $extractDir) {
        Remove-Item -Path $extractDir -Recurse -Force
    }
    if (Test-Path $checksumPath) {
        Remove-Item -Path $checksumPath -Force
    }
}