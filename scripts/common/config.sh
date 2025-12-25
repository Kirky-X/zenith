#!/bin/bash
# Copyright (c) 2025 Kirky.X
#
# Licensed under the MIT License
# See LICENSE file in the project root for full license information.

# Zenith Common Configuration
# This file contains shared configuration for all scripts

# Project information
export PROJECT_NAME="zenith"

# Colors for output
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[1;33m'
export BLUE='\033[0;34m'
export NC='\033[0m'

# Target platforms definition
declare -A ALL_TARGETS=(
    ["x86_64-unknown-linux-gnu"]="Linux x86_64"
    ["aarch64-unknown-linux-gnu"]="Linux ARM64"
    ["x86_64-unknown-linux-musl"]="Linux x86_64 (musl)"
    ["aarch64-unknown-linux-musl"]="Linux ARM64 (musl)"
    ["x86_64-pc-windows-gnu"]="Windows x86_64 (GNU)"
    ["x86_64-pc-windows-msvc"]="Windows x86_64 (MSVC)"
    ["x86_64-apple-darwin"]="macOS Intel"
    ["aarch64-apple-darwin"]="macOS Apple Silicon"
)

# Platform aliases for release packages
declare -A PLATFORM_ALIASES=(
    ["x86_64-unknown-linux-gnu"]="linux-x86_64"
    ["aarch64-unknown-linux-gnu"]="linux-arm64"
    ["x86_64-unknown-linux-musl"]="linux-x86_64-musl"
    ["aarch64-unknown-linux-musl"]="linux-arm64-musl"
    ["x86_64-pc-windows-gnu"]="windows-x86_64-gnu"
    ["x86_64-pc-windows-msvc"]="windows-x86_64-msvc"
    ["x86_64-apple-darwin"]="macos-x86_64"
    ["aarch64-apple-darwin"]="macos-arm64"
)

# GitHub repository configuration
export GITHUB_REPO="user/zenith"
export GITHUB_API="https://api.github.com/repos/${GITHUB_REPO}"
export GITHUB_URL="https://github.com/${GITHUB_REPO}"

# Default installation directories
export DEFAULT_INSTALL_DIR_LINUX="/usr/local/bin"
export DEFAULT_INSTALL_DIR_MACOS="/usr/local/bin"
export DEFAULT_INSTALL_DIR_WINDOWS="${LOCALAPPDATA:-$HOME/AppData/Local}/Programs/zenith"

# Build output directory
export BUILD_OUTPUT_DIR="dist"

# Release directory prefix
export RELEASE_DIR_PREFIX="release"
