#!/bin/bash
# Copyright (c) 2025 Kirky.X
#
# Licensed under the MIT License
# See LICENSE file in the project root for full license information.

# Zenith Installation Script for Linux/macOS
#
# This script installs the Zenith binary for your platform.
# It automatically detects your platform, downloads the appropriate binary,
# verifies checksums, and installs it to /usr/local/bin or $HOME/.local/bin.

set -e  # Exit on any error
set -u  # Exit on undefined variable
set -o pipefail  # Exit on pipe failure

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source common modules
source "${SCRIPT_DIR}/../common/config.sh"
source "${SCRIPT_DIR}/../common/functions.sh"

# Default values
VERSION=""
INSTALL_DIR=""
SKIP_CHECKSUM=false
PROXY=""
OFFLINE_MODE=false
LOCAL_FILE=""

# Temporary files for cleanup
TEMP_FILES=()

# Cleanup function
cleanup() {
    echo -e "${YELLOW}Cleaning up temporary files...${NC}"
    for temp_file in "${TEMP_FILES[@]}"; do
        if [ -e "$temp_file" ]; then
            rm -rf "$temp_file" 2>/dev/null || true
        fi
    done
}

# Register cleanup on exit
trap cleanup EXIT

# Print usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Options:
  -v, --version VERSION    Install specific version (default: latest)
  -d, --dir DIR            Installation directory (default: auto-detect)
  -s, --skip-checksum      Skip checksum verification
  -p, --proxy URL          Use proxy for downloads
  -f, --file PATH          Install from local file (offline mode)
  -h, --help               Show this help message

Examples:
  $0                                    # Install latest version
  $0 -v 1.0.0                           # Install version 1.0.0
  $0 -d /opt/zenith                    # Install to custom directory
  $0 -f ./zenith-x86_64-linux-gnu      # Install from local file

EOF
    exit 0
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -d|--dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        -s|--skip-checksum)
            SKIP_CHECKSUM=true
            shift
            ;;
        -p|--proxy)
            PROXY="$2"
            shift 2
            ;;
        -f|--file)
            LOCAL_FILE="$2"
            OFFLINE_MODE=true
            shift 2
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}=== Zenith Installation Script ===${NC}"

OS=$(detect_os)
ARCH=$(detect_arch)

if [[ "$OS" == "unknown" ]]; then
    echo -e "${RED}Unsupported operating system: $(uname -s)${NC}"
    exit 1
fi

if [[ "$ARCH" == "unknown" ]]; then
    echo -e "${RED}Unsupported architecture: $(uname -m)${NC}"
    exit 1
fi

# Determine the correct target triple
if [[ "$OS" == "darwin" ]]; then
    TARGET="${ARCH}-apple-darwin"
elif [[ "$OS" == "linux" ]]; then
    LIBC=$(detect_libc)
    TARGET="${ARCH}-unknown-linux-${LIBC}"
elif [[ "$OS" == "windows" ]]; then
    TARGET="${ARCH}-pc-windows-msvc"
fi

echo -e "${YELLOW}Detected platform: $OS/$ARCH (target: $TARGET)${NC}"

# Determine installation directory
if [ -z "$INSTALL_DIR" ]; then
    if [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
    elif [ -w "$HOME/.local/bin" ]; then
        INSTALL_DIR="$HOME/.local/bin"
        if ! mkdir -p "$INSTALL_DIR" 2>/dev/null; then
            echo -e "${RED}Failed to create installation directory: $INSTALL_DIR${NC}"
            exit 1
        fi
    else
        echo -e "${RED}No writable installation directory found${NC}"
        echo "Please ensure either /usr/local/bin or $HOME/.local/bin is writable"
        echo "Or use -d option to specify a custom directory"
        exit 1
    fi
else
    if ! mkdir -p "$INSTALL_DIR" 2>/dev/null; then
        echo -e "${RED}Failed to create installation directory: $INSTALL_DIR${NC}"
        exit 1
    fi
fi

echo -e "${YELLOW}Installing to: $INSTALL_DIR${NC}"

# Handle offline mode (local file installation)
if [ "$OFFLINE_MODE" = true ]; then
    if [ ! -f "$LOCAL_FILE" ]; then
        echo -e "${RED}Local file not found: $LOCAL_FILE${NC}"
        exit 1
    fi
    
    echo -e "${YELLOW}Installing from local file: $LOCAL_FILE${NC}"
    BINARY_PATH="$LOCAL_FILE"
    
    # Verify the file is executable
    if [ ! -x "$BINARY_PATH" ]; then
        chmod +x "$BINARY_PATH" 2>/dev/null || echo -e "${YELLOW}Warning: Could not set executable bit${NC}"
    fi
else
    # Online mode: download from GitHub
    # Get the latest release from GitHub
    echo -e "${YELLOW}Fetching release info...${NC}"
    
    # Set proxy if specified
    CURL_OPTS=""
    if [ -n "$PROXY" ]; then
        CURL_OPTS="--proxy $PROXY"
        echo -e "${YELLOW}Using proxy: $PROXY${NC}"
    fi
    
    RELEASE_URL="${GITHUB_API}/releases/latest"
    if [ -z "$VERSION" ]; then
        RESPONSE=$(curl -s --max-time 30 $CURL_OPTS "$RELEASE_URL")
        VERSION=$(echo $RESPONSE | grep -o '"tag_name":"[^"]*' | head -1 | cut -d'"' -f4)
    else
        VERSION="v$VERSION"
    fi
    
    if [ -z "$VERSION" ]; then
        echo -e "${RED}Could not fetch release info${NC}"
        echo "Please check your network connection or use -f option for offline installation"
        exit 1
    fi
    
    echo -e "${YELLOW}Version: $VERSION${NC}"
    
    # Construct the binary name based on the target
    BINARY_NAME="zenith"
    
    # Download the appropriate binary
    BINARY_URL="${GITHUB_URL}/releases/download/${VERSION}/zenith-${TARGET}.tar.gz"
    DOWNLOAD_PATH="/tmp/zenith-${VERSION}-${TARGET}.tar.gz"
    
    echo -e "${YELLOW}Downloading binary from: $BINARY_URL${NC}"
    if ! curl -L --max-time 60 $CURL_OPTS -o "$DOWNLOAD_PATH" "$BINARY_URL"; then
        echo -e "${RED}Failed to download binary${NC}"
        echo "Please check your network connection or use -f option for offline installation"
        exit 1
    fi
    
    # Add to cleanup list
    TEMP_FILES+=("$DOWNLOAD_PATH")
    
    # Verify the downloaded file exists and is not empty
    if [ ! -f "$DOWNLOAD_PATH" ]; then
        echo -e "${RED}Downloaded file not found${NC}"
        exit 1
    fi
    
    file_size=$(stat -c%s "$DOWNLOAD_PATH" 2>/dev/null || stat -f%z "$DOWNLOAD_PATH" 2>/dev/null)
    if [ "$file_size" -eq 0 ]; then
        echo -e "${RED}Downloaded file is empty${NC}"
        exit 1
    fi
    
    echo -e "${YELLOW}Downloaded file size: $file_size bytes${NC}"
    
    # Download and verify checksum if not skipped
    if [ "$SKIP_CHECKSUM" = false ]; then
        CHECKSUM_URL="${GITHUB_URL}/releases/download/${VERSION}/checksums.txt"
        CHECKSUM_PATH="/tmp/checksums-${VERSION}.txt"
        
        echo -e "${YELLOW}Downloading checksums from: $CHECKSUM_URL${NC}"
        if curl -L --max-time 30 $CURL_OPTS -o "$CHECKSUM_PATH" "$CHECKSUM_URL" 2>/dev/null; then
            TEMP_FILES+=("$CHECKSUM_PATH")
            
            # Extract expected hash from checksums file
            EXPECTED_HASH=$(grep "zenith-${TARGET}.tar.gz" "$CHECKSUM_PATH" 2>/dev/null | cut -d' ' -f1)
            
            if [ -n "$EXPECTED_HASH" ]; then
                if ! verify_checksum "$DOWNLOAD_PATH" "$EXPECTED_HASH" true; then
                    echo -e "${RED}Checksum verification failed${NC}"
                    exit 1
                fi
            else
                echo -e "${YELLOW}Checksum not found for this platform in checksums file${NC}"
            fi
        else
            echo -e "${YELLOW}Could not download checksums file, skipping verification${NC}"
        fi
    else
        echo -e "${YELLOW}Checksum verification skipped${NC}"
    fi
    
    # Extract the binary
    EXTRACT_DIR="/tmp/zenith-${VERSION}-${TARGET}"
    mkdir -p "$EXTRACT_DIR"
    
    # Add to cleanup list
    TEMP_FILES+=("$EXTRACT_DIR")
    
    if ! tar -xzf "$DOWNLOAD_PATH" -C "$EXTRACT_DIR"; then
        echo -e "${RED}Failed to extract binary${NC}"
        exit 1
    fi
    
    # Find the actual binary in the extracted content
    BINARY_PATH=""
    for file in "$EXTRACT_DIR"/*; do
        if [[ -f "$file" && -x "$file" ]]; then
            BINARY_PATH="$file"
            break
        fi
    done
    
    if [ -z "$BINARY_PATH" ] || [ ! -f "$BINARY_PATH" ]; then
        echo -e "${RED}Binary not found in extracted content${NC}"
        ls -la "$EXTRACT_DIR"
        exit 1
    fi
fi

INSTALL_PATH="$INSTALL_DIR/$BINARY_NAME"
if cp -f "$BINARY_PATH" "$INSTALL_PATH"; then
    chmod +x "$INSTALL_PATH"
    echo -e "${GREEN}Successfully installed Zenith to $INSTALL_PATH${NC}"
    
    # Print version to confirm installation
    if command -v zenith &> /dev/null; then
        echo -e "${GREEN}Zenith version: $(zenith --version)${NC}"
    else
        echo -e "${YELLOW}Please restart your terminal or run 'source ~/.bashrc' to update PATH${NC}"
    fi
    
    echo -e "${GREEN}Installation complete!${NC}"
    echo -e "${GREEN}Run 'zenith --help' to get started.${NC}"
else
    echo -e "${RED}Failed to install binary${NC}"
    exit 1
fi