#!/bin/bash

# Development script for Rust Slint Sticky Notes

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}  Rust Slint Sticky Notes - Dev Tools  ${NC}"
    echo -e "${BLUE}========================================${NC}"
}

show_help() {
    echo -e "${GREEN}Available commands:${NC}"
    echo "  help     - Show this help message"
    echo "  run      - Run the application"
    echo "  build    - Build the application"
    echo "  test     - Run all tests"
    echo "  test:w   - Run tests in watch mode"
    echo "  lint     - Run clippy linting"
    echo "  fmt      - Format code"
    echo "  check    - Check code compilation"
    echo "  clean    - Clean build artifacts"
    echo "  deps     - Install/update dependencies"
    echo "  release  - Build release version"
    echo ""
    echo -e "${YELLOW}Example usage:${NC}"
    echo "  ./scripts/dev.sh run"
    echo "  ./scripts/dev.sh test"
    echo "  ./scripts/dev.sh lint"
}

run_app() {
    echo -e "${GREEN}🚀 Running Rust Slint Sticky Notes...${NC}"
    cargo run
}

build_app() {
    echo -e "${GREEN}🔨 Building application...${NC}"
    cargo build
}

run_tests() {
    echo -e "${GREEN}🧪 Running tests...${NC}"
    cargo test
}

run_tests_watch() {
    echo -e "${GREEN}🔍 Running tests in watch mode...${NC}"
    echo -e "${YELLOW}Note: Install cargo-watch with 'cargo install cargo-watch'${NC}"
    if command -v cargo-watch &> /dev/null; then
        cargo watch -x test
    else
        echo -e "${RED}Error: cargo-watch not found. Install it with:${NC}"
        echo "cargo install cargo-watch"
        exit 1
    fi
}

run_lint() {
    echo -e "${GREEN}🔍 Running clippy linting...${NC}"
    cargo clippy -- -D warnings
}

format_code() {
    echo -e "${GREEN}✨ Formatting code...${NC}"
    cargo fmt
}

check_code() {
    echo -e "${GREEN}✅ Checking code compilation...${NC}"
    cargo check
}

clean_build() {
    echo -e "${GREEN}🧹 Cleaning build artifacts...${NC}"
    cargo clean
}

update_deps() {
    echo -e "${GREEN}📦 Updating dependencies...${NC}"
    cargo update
}

build_release() {
    echo -e "${GREEN}🎯 Building release version...${NC}"
    cargo build --release
    echo -e "${GREEN}✅ Release build complete!${NC}"
    echo -e "${YELLOW}Binary location: target/release/rust_slint_sticky${NC}"
}

# Main script logic
print_header

case "${1:-help}" in
    "help")
        show_help
        ;;
    "run")
        run_app
        ;;
    "build")
        build_app
        ;;
    "test")
        run_tests
        ;;
    "test:w")
        run_tests_watch
        ;;
    "lint")
        run_lint
        ;;
    "fmt")
        format_code
        ;;
    "check")
        check_code
        ;;
    "clean")
        clean_build
        ;;
    "deps")
        update_deps
        ;;
    "release")
        build_release
        ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        echo ""
        show_help
        exit 1
        ;;
esac 