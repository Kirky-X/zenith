#!/bin/bash
# Zenith Build and Release Management Script
# This is the unified entry point for all build, install, and release operations

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/common/config.sh"
source "${SCRIPT_DIR}/common/functions.sh"

VERSION=$(get_cargo_version)

usage() {
    cat << EOF
${BLUE}Zenith Build and Release Management${NC} v${VERSION}

Usage: $0 <command> [options]

Commands:
  build           Build for current platform
  build-all       Build for all supported platforms
  install         Install zenith on current platform
  release         Create release packages for all platforms
  version         Show or manage version
  clean           Clean build artifacts
  help            Show this help message

Build Options:
  --release       Build in release mode (default)
  --debug         Build in debug mode
  --target TRIPLET  Build for specific target (e.g., x86_64-unknown-linux-gnu)

Install Options:
  -v, --version VERSION  Install specific version
  -d, --dir DIR          Installation directory
  -s, --skip-checksum    Skip checksum verification
  -f, --file PATH        Install from local file

Release Options:
  --skip-build      Skip building (use existing binaries)
  --skip-checksum    Skip checksum generation
  --dry-run          Show what would be done without doing it

Version Options:
  show               Show current version
  update <ver>       Update version number
  validate           Validate version format
  bump <type>        Bump version (major|minor|patch)

Examples:
  $0 build                          # Build for current platform
  $0 build-all                      # Build for all platforms
  $0 install                        # Install latest version
  $0 install -v 1.0.0               # Install specific version
  $0 release --dry-run              # Preview release process
  $0 version show                   # Show current version
  $0 version bump patch             # Bump patch version

EOF
}

show_targets() {
    echo -e "${BLUE}Supported Targets:${NC}"
    for target in "${!ALL_TARGETS[@]}"; do
        echo "  ${target} - ${ALL_TARGETS[$target]}"
    done
}

case "${1:-}" in
    build)
        shift
        "${SCRIPT_DIR}/build/build_cross_platform.sh" "$@"
        ;;
    build-all)
        shift
        "${SCRIPT_DIR}/build/build_cross_platform.sh" --all "$@"
        ;;
    install)
        shift
        "${SCRIPT_DIR}/install/install.sh" "$@"
        ;;
    release)
        shift
        "${SCRIPT_DIR}/release/release.sh" "$@"
        ;;
    version)
        shift
        "${SCRIPT_DIR}/common/version.sh" "$@"
        ;;
    clean)
        echo -e "${YELLOW}Cleaning build artifacts...${NC}"
        rm -rf "${BUILD_OUTPUT_DIR}"
        rm -rf "${RELEASE_DIR_PREFIX}-"* target/
        echo -e "${GREEN}Clean complete!${NC}"
        ;;
    targets)
        show_targets
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        echo -e "${RED}Error: Unknown command '${1:-}'${NC}"
        echo ""
        usage
        exit 1
        ;;
esac
