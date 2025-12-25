#!/bin/bash
# Copyright (c) 2025 Kirky.X
#
# Licensed under the MIT License
# See LICENSE file in the project root for full license information.

# Zenith Version Management Script
# This script handles version-related operations

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/config.sh"
source "$SCRIPT_DIR/functions.sh"

usage() {
    cat << EOF
Usage: $0 [command] [options]

Commands:
  show           Show current version
  update <ver>   Update version number in all files
  validate       Validate version format
  bump <type>    Bump version (major, minor, patch)

Examples:
  $0 show
  $0 update 1.0.2
  $0 validate
  $0 bump patch
EOF
}

validate_version_format() {
    local version="$1"
    
    if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo -e "${RED}Error: Invalid version format: $version${NC}"
        echo "Expected format: X.Y.Z (e.g., 1.0.0)"
        return 1
    fi
    
    return 0
}

bump_version() {
    local bump_type="$1"
    local current_version=$(get_cargo_version)
    
    if ! validate_version_format "$current_version"; then
        return 1
    fi
    
    local major=$(echo "$current_version" | cut -d'.' -f1)
    local minor=$(echo "$current_version" | cut -d'.' -f2)
    local patch=$(echo "$current_version" | cut -d'.' -f3)
    
    case "$bump_type" in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
        *)
            echo -e "${RED}Error: Invalid bump type: $bump_type${NC}"
            echo "Expected: major, minor, or patch"
            return 1
            ;;
    esac
    
    local new_version="${major}.${minor}.${patch}"
    echo "$new_version"
}

update_cargo_toml() {
    local new_version="$1"
    
    if [ ! -f "Cargo.toml" ]; then
        echo -e "${RED}Error: Cargo.toml not found${NC}"
        return 1
    fi
    
    sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    echo -e "${GREEN}Updated Cargo.toml to $new_version${NC}"
}

update_pkgbuild() {
    local new_version="$1"
    local pkgbuild_file="scripts/packages/PKGBUILD"
    
    if [ ! -f "$pkgbuild_file" ]; then
        pkgbuild_file="scripts/PKGBUILD"
    fi
    
    if [ -f "$pkgbuild_file" ]; then
        sed -i "s/^pkgver=.*/pkgver=$new_version/" "$pkgbuild_file"
        echo -e "${GREEN}Updated PKGBUILD to $new_version${NC}"
    else
        echo -e "${YELLOW}Warning: PKGBUILD not found${NC}"
    fi
}

update_zenith_rb() {
    local new_version="$1"
    local zenith_rb_file="scripts/packages/zenith.rb"
    
    if [ ! -f "$zenith_rb_file" ]; then
        zenith_rb_file="scripts/zenith.rb"
    fi
    
    if [ -f "$zenith_rb_file" ]; then
        sed -i "s|url \".*v.*\.tar\.gz\"|url \"https://github.com/${GITHUB_REPO}/archive/refs/tags/v${new_version}.tar.gz\"|" "$zenith_rb_file"
        echo -e "${GREEN}Updated zenith.rb to $new_version${NC}"
    else
        echo -e "${YELLOW}Warning: zenith.rb not found${NC}"
    fi
}

update_version() {
    local new_version="$1"
    
    if ! validate_version_format "$new_version"; then
        return 1
    fi
    
    echo -e "${BLUE}Updating version to $new_version${NC}"
    
    update_cargo_toml "$new_version"
    update_pkgbuild "$new_version"
    update_zenith_rb "$new_version"
    
    echo -e "${GREEN}Version update complete!${NC}"
    echo -e "${YELLOW}Remember to commit the changes:${NC}"
    echo "  git add Cargo.toml scripts/packages/PKGBUILD scripts/packages/zenith.rb"
    echo "  git commit -m \"chore: bump version to $new_version\""
}

# Only execute command handling if script is run directly (not sourced)
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then

case "${1:-}" in
    show)
        version=$(get_cargo_version)
        if [ -n "$version" ]; then
            echo "$version"
        else
            echo -e "${RED}Error: Could not determine version${NC}"
            exit 1
        fi
        ;;
    update)
        if [ -z "${2:-}" ]; then
            echo -e "${RED}Error: Version number required${NC}"
            echo "Usage: $0 update <version>"
            exit 1
        fi
        update_version "$2"
        ;;
    validate)
        version=$(get_cargo_version)
        if validate_version_format "$version"; then
            echo -e "${GREEN}Version format is valid: $version${NC}"
        else
            exit 1
        fi
        ;;
    bump)
        if [ -z "${2:-}" ]; then
            echo -e "${RED}Error: Bump type required${NC}"
            echo "Usage: $0 bump <major|minor|patch>"
            exit 1
        fi
        new_version=$(bump_version "$2")
        update_version "$new_version"
        ;;
    *)
        usage
        exit 1
        ;;
esac
fi
