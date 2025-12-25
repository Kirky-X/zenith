#!/bin/bash

# Cross-platform build script for Zenith
# Builds Zenith for multiple platforms using cargo or cargo-cross

set -e  # Exit on any error

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source common modules
source "${SCRIPT_DIR}/../common/config.sh"
source "${SCRIPT_DIR}/../common/functions.sh"

echo -e "${GREEN}Starting cross-platform build for Zenith${NC}"

# Check if rustup is available
if ! command -v rustup &> /dev/null; then
    echo -e "${RED}Error: rustup is required but not installed.${NC}"
    exit 1
fi

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is required but not installed.${NC}"
    exit 1
fi

# Parse command line arguments
USE_CROSS=false
BUILD_ALL=false
SKIP_MISSING=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --cross)
            USE_CROSS=true
            shift
            ;;
        --all)
            BUILD_ALL=true
            shift
            ;;
        --skip-missing)
            SKIP_MISSING=true
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --cross          Use cargo-cross for cross-compilation"
            echo "  --all            Build for all supported platforms (may require docker)"
            echo "  --skip-missing   Skip platforms with missing toolchains"
            echo "  --help           Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if cargo-cross is requested and available
if [ "$USE_CROSS" = true ]; then
    if ! command -v cross &> /dev/null; then
        echo -e "${YELLOW}cargo-cross not found. Installing...${NC}"
        cargo install cross
    fi
    echo -e "${GREEN}Using cargo-cross for cross-compilation${NC}"
fi

# Determine the host platform
host_platform=$(uname -s)
host_arch=$(uname -m)

echo -e "${BLUE}Host platform: $host_platform $host_arch${NC}"

# Determine targets to build based on host platform and flags
targets=()

if [ "$BUILD_ALL" = true ]; then
    # Build for all platforms (requires cross or docker)
    echo -e "${YELLOW}Building for all platforms...${NC}"
    for target in "${!ALL_TARGETS[@]}"; do
        targets+=("$target")
    done
else
    # Build based on host platform
    if [[ "$host_platform" == "Linux" ]]; then
        targets=("x86_64-unknown-linux-gnu")
        
        # Check for aarch64 cross-compilation tools
        if command -v aarch64-linux-gnu-gcc &> /dev/null || command -v aarch64-linux-gnu-ld &> /dev/null; then
            targets+=("aarch64-unknown-linux-gnu")
            echo -e "${GREEN}aarch64 cross-compilation tools found${NC}"
        else
            echo -e "${YELLOW}aarch64 cross-compilation tools not found${NC}"
            echo -e "${YELLOW}  Install: sudo apt-get install gcc-aarch64-linux-gnu${NC}"
            if [ "$USE_CROSS" = true ] || [ "$SKIP_MISSING" = false ]; then
                echo -e "${YELLOW}  Will attempt with cargo-cross${NC}"
                targets+=("aarch64-unknown-linux-gnu")
            fi
        fi
        
        # Check for musl cross-compilation
        if command -v musl-gcc &> /dev/null || rustup target list --installed | grep -q "x86_64-unknown-linux-musl"; then
            targets+=("x86_64-unknown-linux-musl")
            echo -e "${GREEN}musl cross-compilation available${NC}"
        fi
        
        # Check for Windows cross-compilation
        if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
            targets+=("x86_64-pc-windows-gnu")
            echo -e "${GREEN}Windows cross-compilation tools found${NC}"
        else
            echo -e "${YELLOW}Windows cross-compilation tools not found${NC}"
            echo -e "${YELLOW}  Install: sudo apt-get install gcc-mingw-w64${NC}"
            if [ "$USE_CROSS" = true ] || [ "$SKIP_MISSING" = false ]; then
                echo -e "${YELLOW}  Will attempt with cargo-cross${NC}"
                targets+=("x86_64-pc-windows-gnu")
            fi
        fi
        
        echo -e "${YELLOW}For macOS builds, run this script on macOS or use --cross${NC}"
        
    elif [[ "$host_platform" == "Darwin" ]]; then
        # macOS builds
        if [[ "$host_arch" == "arm64" ]]; then
            targets=("aarch64-apple-darwin" "x86_64-apple-darwin")
        else
            targets=("x86_64-apple-darwin" "aarch64-apple-darwin")
        fi
        
        # Check for Linux cross-compilation on macOS
        if command -v x86_64-linux-gnu-gcc &> /dev/null || [ "$USE_CROSS" = true ]; then
            targets+=("x86_64-unknown-linux-gnu")
            echo -e "${GREEN}Linux cross-compilation available${NC}"
        else
            echo -e "${YELLOW}Linux cross-compilation not available${NC}"
            echo -e "${YELLOW}  Use --cross flag for cross-compilation${NC}"
        fi
        
    else
        # Windows or other platforms
        if [[ "$host_platform" == *"MINGW"* ]] || [[ "$host_platform" == *"MSYS"* ]]; then
            targets=("x86_64-pc-windows-msvc")
        else
            targets=("x86_64-pc-windows-msvc" "x86_64-unknown-linux-gnu")
        fi
    fi
fi

echo -e "${BLUE}Building for ${#targets[@]} platform(s):${NC}"
for target in "${targets[@]}"; do
    echo "  - ${ALL_TARGETS[$target]:-$target}"
done

# Add targets if not already installed
echo -e "${YELLOW}Checking and installing build targets...${NC}"
for target in "${targets[@]}"; do
    if ! rustup target list --installed | grep -q "$target"; then
        echo -e "${YELLOW}Installing target: $target${NC}"
        rustup target add "$target"
    else
        echo -e "${GREEN}Target already installed: $target${NC}"
    fi
done

# Create output directory
output_dir="dist"
mkdir -p "$output_dir"

# Build for each target
echo -e "${YELLOW}Building for each platform...${NC}"
success_count=0
failed_count=0
skipped_count=0

for target in "${targets[@]}"; do
    echo -e "${YELLOW}Building for $target...${NC}"
    
    # Determine build command based on cross flag
    if [ "$USE_CROSS" = true ]; then
        build_cmd="cross"
    else
        build_cmd="cargo"
    fi
    
    # Build the application
    if $build_cmd build --target "$target" --release --locked 2>&1; then
        # Find the built binary and prepare it for release
        if [[ "$target" == *"windows"* ]]; then
            # Windows binaries have .exe extension
            binary_name="zenith.exe"
            release_name="zenith-${target}.exe"
        else
            # Unix-like systems don't have extension
            binary_name="zenith"
            release_name="zenith-${target}.tar.gz"
        fi
        
        binary_path="target/$target/release/$binary_name"
        
        if [ -f "$binary_path" ]; then
            if [[ "$target" == *"windows"* ]]; then
                # For Windows, copy the .exe file directly to dist
                cp "$binary_path" "$output_dir/$release_name"
                echo -e "${GREEN}Successfully built $output_dir/$release_name${NC}"
                success_count=$((success_count + 1))
            else
                # For Linux/macOS, create a tar.gz archive
                tar -czf "$output_dir/$release_name" -C "target/$target/release" "$binary_name"
                echo -e "${GREEN}Successfully built $output_dir/$release_name${NC}"
                success_count=$((success_count + 1))
            fi
        else
            echo -e "${RED}Error: Binary not found at $binary_path${NC}"
            failed_count=$((failed_count + 1))
        fi
    else
        echo -e "${RED}Error building for $target${NC}"
        if [ "$SKIP_MISSING" = true ]; then
            echo -e "${YELLOW}Skipping $target due to --skip-missing flag${NC}"
            skipped_count=$((skipped_count + 1))
        else
            failed_count=$((failed_count + 1))
        fi
    fi
done

# Print summary
echo ""
echo -e "${BLUE}=== Build Summary ===${NC}"
echo -e "${GREEN}Successful: $success_count${NC}"
echo -e "${RED}Failed: $failed_count${NC}"
echo -e "${YELLOW}Skipped: $skipped_count${NC}"
echo -e "${GREEN}Total: $((success_count + failed_count + skipped_count))${NC}"

if [ $failed_count -gt 0 ]; then
    echo ""
    echo -e "${YELLOW}Tip: Use --cross flag to use cargo-cross for cross-compilation${NC}"
    echo -e "${YELLOW}Tip: Use --skip-missing to skip platforms with missing toolchains${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}Cross-platform build completed!${NC}"
echo -e "${GREEN}Release files are located in the 'dist' directory:${NC}"
find "$output_dir" -type f | sort | while read -r file; do
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "  - $file ($(stat -f%z "$file") bytes)"
    else
        echo "  - $file ($(stat -c%s "$file") bytes)"
    fi
done