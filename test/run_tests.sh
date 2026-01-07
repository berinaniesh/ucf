#!/bin/bash

# UCF Formatter Test Script
# Tests that formatters are working for all available formatters in the local environment

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SAMPLES_DIR="$SCRIPT_DIR/samples"
TMP_DIR=$(mktemp -d)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Cleanup on exit
cleanup() {
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

# Build ucf first
echo "Building ucf..."
cd "$PROJECT_DIR"
cargo build --release 2>/dev/null
UCF="$PROJECT_DIR/target/release/ucf"

if [[ ! -x "$UCF" ]]; then
    echo -e "${RED}Failed to build ucf${NC}"
    exit 1
fi
echo -e "${GREEN}ucf built successfully${NC}"
echo ""

# Track results
PASSED=0
FAILED=0
SKIPPED=0

echo "========================================"
echo "       UCF Formatter Test Results"
echo "========================================"
echo ""

# Test function
test_formatter() {
    local formatter="$1"
    local file="$2"
    local src_file="$SAMPLES_DIR/$file"

    # Check if formatter is available
    if ! command -v "$formatter" &> /dev/null; then
        echo -e "${YELLOW}[SKIP]${NC} $file ($formatter) - formatter not installed"
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    if [[ ! -f "$src_file" ]]; then
        echo -e "${YELLOW}[SKIP]${NC} $file - sample file not found"
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    # Copy to temp dir
    local tmp_file="$TMP_DIR/$file"
    cp "$src_file" "$tmp_file"

    # Get original content hash
    local original_hash=$(md5 -q "$tmp_file" 2>/dev/null || md5sum "$tmp_file" | cut -d' ' -f1)

    # Run ucf
    if "$UCF" "$tmp_file" &> /dev/null; then
        # Get new content hash
        local new_hash=$(md5 -q "$tmp_file" 2>/dev/null || md5sum "$tmp_file" | cut -d' ' -f1)

        if [[ "$original_hash" != "$new_hash" ]]; then
            echo -e "${GREEN}[PASS]${NC} $file ($formatter) - formatting applied"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}[FAIL]${NC} $file ($formatter) - no changes made"
            FAILED=$((FAILED + 1))
        fi
    else
        echo -e "${RED}[FAIL]${NC} $file ($formatter) - ucf returned error"
        FAILED=$((FAILED + 1))
    fi
}

# Run tests for each formatter and file combination
# Format: test_formatter "formatter_binary" "sample_file"

# clang-format
test_formatter "clang-format" "sample.c"
test_formatter "clang-format" "sample.json"

# cmake-format
test_formatter "cmake-format" "sample.cmake"

# gofmt
test_formatter "gofmt" "sample.go"

# prettier
test_formatter "prettier" "sample.js"
test_formatter "prettier" "sample.ts"
test_formatter "prettier" "sample.html"
test_formatter "prettier" "sample.css"
test_formatter "prettier" "sample.yaml"

# black
test_formatter "black" "sample.py"

# rustfmt
test_formatter "rustfmt" "sample.rs"

# stylish-haskell
test_formatter "stylish-haskell" "sample.hs"

# stylua
test_formatter "stylua" "sample.lua"

# shfmt
test_formatter "shfmt" "sample.sh"

# taplo
test_formatter "taplo" "sample.toml"

# xmllint
test_formatter "xmllint" "sample.xml"

# zig
test_formatter "zig" "sample.zig"

# tex-fmt
test_formatter "tex-fmt" "sample.tex"

# terraform
test_formatter "terraform" "sample.tf"

echo ""
echo "========================================"
echo "              Summary"
echo "========================================"
echo -e "${GREEN}Passed:${NC}  $PASSED"
echo -e "${RED}Failed:${NC}  $FAILED"
echo -e "${YELLOW}Skipped:${NC} $SKIPPED"
echo ""

if [[ $FAILED -gt 0 ]]; then
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
else
    echo -e "${GREEN}All available formatters working correctly!${NC}"
    exit 0
fi
