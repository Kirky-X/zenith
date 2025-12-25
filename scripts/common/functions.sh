#!/bin/bash
# Zenith Common Functions Library
# This file contains shared utility functions for all scripts

# Get version from Cargo.toml
get_cargo_version() {
    grep "^version" Cargo.toml | head -1 | awk '{print $3}' | tr -d '"'
}

# Detect operating system
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "darwin";;
        MINGW*|MSYS*|CYGWIN*) echo "windows";;
        *)          echo "unknown";;
    esac
}

# Detect architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)     echo "x86_64";;
        aarch64|arm64)    echo "aarch64";;
        armv7l)           echo "armv7";;
        i386|i686)        echo "i686";;
        *)                echo "unknown";;
    esac
}

# Detect libc (Linux only)
detect_libc() {
    if command -v ldd &> /dev/null; then
        if ldd --version 2>&1 | grep -q "musl"; then
            echo "musl"
        else
            echo "gnu"
        fi
    else
        echo "gnu"
    fi
}

# Get target triple for current platform
get_target_triple() {
    local os=$(detect_os)
    local arch=$(detect_arch)
    
    if [[ "$os" == "unknown" ]]; then
        echo "unknown"
        return 1
    fi
    
    if [[ "$arch" == "unknown" ]]; then
        echo "unknown"
        return 1
    fi
    
    if [[ "$os" == "darwin" ]]; then
        echo "${arch}-apple-darwin"
    elif [[ "$os" == "linux" ]]; then
        local libc=$(detect_libc)
        echo "${arch}-unknown-linux-${libc}"
    elif [[ "$os" == "windows" ]]; then
        echo "${arch}-pc-windows-msvc"
    fi
}

# Calculate SHA256 checksum
calculate_sha256() {
    local file="$1"
    
    if command -v sha256sum &> /dev/null; then
        sha256sum "$file" | cut -d' ' -f1
    elif command -v shasum &> /dev/null; then
        shasum -a 256 "$file" | cut -d' ' -f1
    else
        echo ""
        return 1
    fi
}

# Verify checksum of a file
verify_checksum() {
    local file="$1"
    local expected_hash="$2"
    local skip_on_missing="${3:-false}"
    
    if [ ! -f "$file" ]; then
        echo -e "${RED}Error: File not found: $file${NC}"
        return 1
    fi
    
    local calculated_hash=$(calculate_sha256 "$file")
    
    if [ -z "$calculated_hash" ]; then
        if [ "$skip_on_missing" = true ]; then
            echo -e "${YELLOW}Checksum verification skipped: no sha256sum or shasum available${NC}"
            return 0
        else
            echo -e "${RED}Error: No checksum tool available${NC}"
            return 1
        fi
    fi
    
    if [ -z "$expected_hash" ]; then
        echo -e "${YELLOW}Warning: Expected hash is empty${NC}"
        return 1
    fi
    
    if [ "$calculated_hash" = "$expected_hash" ]; then
        echo -e "${GREEN}Checksum verified: $calculated_hash${NC}"
        return 0
    else
        echo -e "${RED}Checksum mismatch!${NC}"
        echo "Expected: $expected_hash"
        echo "Calculated: $calculated_hash"
        return 1
    fi
}

# Download file with optional proxy
download_file() {
    local url="$1"
    local output="$2"
    local proxy="${3:-}"
    local timeout="${4:-60}"
    
    local curl_opts="--max-time $timeout"
    
    if [ -n "$proxy" ]; then
        curl_opts="$curl_opts --proxy $proxy"
    fi
    
    if curl -L $curl_opts -o "$output" "$url"; then
        return 0
    else
        return 1
    fi
}

# Get file size
get_file_size() {
    local file="$1"
    
    if [[ "$OSTYPE" == "darwin"* ]]; then
        stat -f%z "$file" 2>/dev/null || echo "0"
    else
        stat -c%s "$file" 2>/dev/null || echo "0"
    fi
}

# Check if command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Check if directory is writable
is_writable() {
    local dir="$1"
    
    if [ -w "$dir" ]; then
        return 0
    else
        return 1
    fi
}

# Get latest release tag from GitHub
get_latest_release() {
    local repo="${1:-$GITHUB_REPO}"
    local api_url="https://api.github.com/repos/${repo}/releases/latest"
    
    local response=$(curl -s --max-time 30 "$api_url")
    echo "$response" | grep -o '"tag_name":"[^"]*' | head -1 | cut -d'"' -f4
}

# Extract archive based on file extension
extract_archive() {
    local archive="$1"
    local dest_dir="$2"
    
    mkdir -p "$dest_dir"
    
    case "$archive" in
        *.tar.gz|*.tgz)
            tar -xzf "$archive" -C "$dest_dir"
            ;;
        *.tar.bz2|*.tbz2)
            tar -xjf "$archive" -C "$dest_dir"
            ;;
        *.zip)
            unzip -q "$archive" -d "$dest_dir"
            ;;
        *)
            echo -e "${RED}Unsupported archive format: $archive${NC}"
            return 1
            ;;
    esac
    
    return 0
}

# Create archive based on platform
create_archive() {
    local binary="$1"
    local output="$2"
    local target="$3"
    
    if [[ "$target" == *"windows"* ]]; then
        # For Windows, create zip
        local dir=$(dirname "$output")
        mkdir -p "$dir"
        cd "$(dirname "$binary")"
        zip -q "$output" "$(basename "$binary")"
        cd - > /dev/null
    else
        # For Linux/macOS, create tar.gz
        local dir=$(dirname "$output")
        mkdir -p "$dir"
        tar -czf "$output" -C "$(dirname "$binary")" "$(basename "$binary")"
    fi
    
    return 0
}

# Cleanup temporary files
cleanup_temp_files() {
    local temp_files=("$@")
    
    for temp_file in "${temp_files[@]}"; do
        if [ -e "$temp_file" ]; then
            rm -rf "$temp_file" 2>/dev/null || true
        fi
    done
}

# Print error message and exit
error_exit() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit "${2:-1}"
}

# Print warning message
warn() {
    echo -e "${YELLOW}Warning: $1${NC}" >&2
}

# Print info message
info() {
    echo -e "${BLUE}Info: $1${NC}"
}

# Print success message
success() {
    echo -e "${GREEN}$1${NC}"
}
