#!/bin/bash
# Copyright (c) 2025 Kirky.X
#
# Licensed under the MIT License
# See LICENSE file in the project root for full license information.

# Release script for Zenith
# Creates release packages for all platforms

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source common modules
source "${SCRIPT_DIR}/../common/config.sh"
source "${SCRIPT_DIR}/../common/functions.sh"
source "${SCRIPT_DIR}/../common/version.sh"

# Get version from Cargo.toml
VERSION=$(get_cargo_version)

if [ -z "$VERSION" ]; then
    echo -e "${RED}Error: Could not determine version from Cargo.toml${NC}"
    exit 1
fi

echo -e "${BLUE}=== Zenith Release Script v${VERSION} ===${NC}"

# Parse command line arguments
SKIP_BUILD=false
SKIP_CHECKSUM=false
DRY_RUN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-build)
            SKIP_BUILD=true
            shift
            ;;
        --skip-checksum)
            SKIP_CHECKSUM=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --skip-build      Skip building (use existing binaries)"
            echo "  --skip-checksum    Skip checksum generation"
            echo "  --dry-run          Show what would be done without doing it"
            echo "  --help             Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Create release directory
RELEASE_DIR="release-${VERSION}"
mkdir -p "$RELEASE_DIR"

echo -e "${YELLOW}Release directory: $RELEASE_DIR${NC}"

# Build for all platforms if not skipped
if [ "$SKIP_BUILD" = false ]; then
    echo -e "${YELLOW}Building for all platforms...${NC}"
    if [ -f "scripts/build_cross_platform.sh" ]; then
        chmod +x scripts/build_cross_platform.sh
        ./scripts/build_cross_platform.sh --all --cross
    else
        echo -e "${RED}Error: scripts/build_cross_platform.sh not found${NC}"
        exit 1
    fi
fi

# Create release packages
echo -e "${YELLOW}Creating release packages...${NC}"
CHECKSUM_FILE="$RELEASE_DIR/checksums.txt"

if [ "$SKIP_CHECKSUM" = false ]; then
    echo "# Zenith ${VERSION} Checksums" > "$CHECKSUM_FILE"
    echo "# Generated on $(date -u +"%Y-%m-%d %H:%M:%S UTC")" >> "$CHECKSUM_FILE"
    echo "" >> "$CHECKSUM_FILE"
fi

for target in "${!ALL_TARGETS[@]}"; do
    platform="${ALL_TARGETS[$target]}"
    echo -e "${YELLOW}Processing $platform ($target)...${NC}"
    
    # Determine binary name and extension
    if [[ "$target" == *"windows"* ]]; then
        binary_name="zenith.exe"
        archive_name="zenith-${VERSION}-${platform}.zip"
    else
        binary_name="zenith"
        archive_name="zenith-${VERSION}-${platform}.tar.gz"
    fi
    
    # Find the binary
    if [ -f "dist/zenith-${target}.tar.gz" ]; then
        # Use pre-built tar.gz from dist
        source_file="dist/zenith-${target}.tar.gz"
        if [[ "$target" == *"windows"* ]]; then
            # For Windows, need to extract and repackage as zip
            temp_dir=$(mktemp -d)
            tar -xzf "$source_file" -C "$temp_dir"
            if [ "$DRY_RUN" = false ]; then
                cd "$temp_dir"
                zip -q "../$RELEASE_DIR/$archive_name" "$binary_name"
                cd - > /dev/null
                rm -rf "$temp_dir"
            else
                echo -e "${BLUE}[DRY RUN] Would create: $RELEASE_DIR/$archive_name${NC}"
                rm -rf "$temp_dir"
            fi
        else
            # For Linux/macOS, just copy
            if [ "$DRY_RUN" = false ]; then
                cp "$source_file" "$RELEASE_DIR/$archive_name"
            else
                echo -e "${BLUE}[DRY RUN] Would copy: $source_file -> $RELEASE_DIR/$archive_name${NC}"
            fi
        fi
    elif [ -f "dist/zenith-${target}.exe" ]; then
        # Use pre-built exe from dist
        source_file="dist/zenith-${target}.exe"
        if [ "$DRY_RUN" = false ]; then
            cd dist
            zip -q "../$RELEASE_DIR/$archive_name" "zenith-${target}.exe"
            cd - > /dev/null
        else
            echo -e "${BLUE}[DRY RUN] Would create: $RELEASE_DIR/$archive_name${NC}"
        fi
    else
        echo -e "${RED}Warning: Binary not found for $target${NC}"
        continue
    fi
    
    # Generate checksum
    if [ "$SKIP_CHECKSUM" = false ] && [ "$DRY_RUN" = false ]; then
        if [ -f "$RELEASE_DIR/$archive_name" ]; then
            checksum=$(sha256sum "$RELEASE_DIR/$archive_name" | cut -d' ' -f1)
            echo "$checksum  $archive_name" >> "$CHECKSUM_FILE"
            echo -e "${GREEN}  Checksum: $checksum${NC}"
        fi
    fi
done

# Create source archive
echo -e "${YELLOW}Creating source archive...${NC}"
SOURCE_ARCHIVE="zenith-${VERSION}-source.tar.gz"

if [ "$DRY_RUN" = false ]; then
    # Create a clean source archive
    temp_dir=$(mktemp -d)
    git archive --format=tar --prefix="zenith-${VERSION}/" HEAD | tar -x -C "$temp_dir"
    
    # Add submodules if any
    if [ -f ".gitmodules" ]; then
        git submodule update --init --recursive
        git submodule foreach --recursive 'git archive --format=tar --prefix=$path/ HEAD | tar -x -C "'"$temp_dir/zenith-${VERSION}"'"'
    fi
    
    # Create the archive
    cd "$temp_dir"
    tar -czf "../$RELEASE_DIR/$SOURCE_ARCHIVE" "zenith-${VERSION}"
    cd - > /dev/null
    rm -rf "$temp_dir"
    
    # Generate checksum for source
    if [ "$SKIP_CHECKSUM" = false ]; then
        checksum=$(sha256sum "$RELEASE_DIR/$SOURCE_ARCHIVE" | cut -d' ' -f1)
        echo "$checksum  $SOURCE_ARCHIVE" >> "$CHECKSUM_FILE"
        echo -e "${GREEN}Source checksum: $checksum${NC}"
    fi
else
    echo -e "${BLUE}[DRY RUN] Would create: $RELEASE_DIR/$SOURCE_ARCHIVE${NC}"
fi

# Create installation script
echo -e "${YELLOW}Creating installation script...${NC}"
INSTALL_SCRIPT="install.sh"

if [ "$DRY_RUN" = false ]; then
    cp scripts/install.sh "$RELEASE_DIR/$INSTALL_SCRIPT"
    chmod +x "$RELEASE_DIR/$INSTALL_SCRIPT"
else
    echo -e "${BLUE}[DRY RUN] Would copy: scripts/install.sh -> $RELEASE_DIR/$INSTALL_SCRIPT${NC}"
fi

# Create README for release
echo -e "${YELLOW}Creating release README...${NC}"
RELEASE_README="$RELEASE_DIR/README.md"

if [ "$DRY_RUN" = false ]; then
    cat > "$RELEASE_README" << EOF
# Zenith ${VERSION}

Zenith is a multi-language code formatter with backup capabilities.

## Installation

### Quick Install (Linux/macOS)

\`\`\`bash
# Install latest version
./scripts/install.sh

# Install this specific version
./scripts/install.sh --version ${VERSION}
\`\`\`

### Manual Installation

Extract the appropriate archive for your platform:

\`\`\`bash
# Linux x86_64
tar -xzf zenith-${VERSION}-linux-x86_64.tar.gz
sudo mv zenith /usr/local/bin/

# macOS Intel
tar -xzf zenith-${VERSION}-macos-x86_64.tar.gz
sudo mv zenith /usr/local/bin/

# macOS Apple Silicon
tar -xzf zenith-${VERSION}-macos-arm64.tar.gz
sudo mv zenith /usr/local/bin/

# Windows
unzip zenith-${VERSION}-windows-x86_64-msvc.zip
# Add zenith.exe to your PATH
\`\`\`

## Verification

Verify the integrity of the downloaded files using the checksums:

\`\`\`bash
# Linux/macOS
sha256sum -c checksums.txt

# Windows
certutil -hashfile <file> SHA256
\`\`\`

## Quick Start

\`\`\`bash
# Format a file
zenith format main.rs

# Format a directory
zenith format src/

# Format recursively
zenith format ./ --recursive

# Check mode (no changes)
zenith format src/ --check

# Initialize configuration
zenith init

# Show help
zenith --help
\`\`\`

## Documentation

- [User Guide](https://github.com/user/zenith/blob/main/docs/USE_GUIDE.md)
- [Developer Guide](https://github.com/user/zenith/blob/main/docs/CONTRIBUTING.md)
- [Building Instructions](https://github.com/user/zenith/blob/main/docs/BUILDING.md)
- [API Documentation](https://docs.rs/zenith)

## Support

- GitHub Issues: https://github.com/user/zenith/issues
- Discussions: https://github.com/user/zenith/discussions

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Checksums

EOF
else
    echo -e "${BLUE}[DRY RUN] Would create: $RELEASE_README${NC}"
fi

# List all release files
echo ""
echo -e "${BLUE}=== Release Files ===${NC}"
if [ "$DRY_RUN" = false ]; then
    ls -lh "$RELEASE_DIR"
else
    echo -e "${BLUE}[DRY RUN] Would create files in: $RELEASE_DIR${NC}"
fi

# Print summary
echo ""
echo -e "${BLUE}=== Release Summary ===${NC}"
echo -e "${GREEN}Version: ${VERSION}${NC}"
echo -e "${GREEN}Release directory: $RELEASE_DIR${NC}"

if [ "$DRY_RUN" = false ]; then
    file_count=$(find "$RELEASE_DIR" -type f | wc -l)
    echo -e "${GREEN}Total files: $file_count${NC}"
fi

echo ""
echo -e "${GREEN}Release package created successfully!${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Review the release files in $RELEASE_DIR"
echo "2. Test the binaries on each platform"
echo "3. Create a GitHub release:"
echo "   git tag -a v${VERSION} -m \"Release v${VERSION}\""
echo "   git push origin v${VERSION}"
echo "4. Upload the release files to GitHub"
echo ""
echo -e "${YELLOW}To upload to GitHub:${NC}"
echo "   gh release create v${VERSION} $RELEASE_DIR/* --title \"Zenith ${VERSION}\" --notes \"See docs/CHANGELOG.md for details\""
