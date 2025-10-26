# Mouse Macro - Just Commands

# Set shell for Windows
set shell := ["powershell.exe", "-NoLogo", "-Command"]

# Default recipe
default:
    @just --list

# Install all dependencies
install:
    @echo "Installing Rust dependencies..."
    cargo fetch
    @echo "Installing Tauri CLI..."
    npm install
    @echo "Installing frontend dependencies..."
    npm --prefix ui install
    @echo "All dependencies installed"

# Run GUI in development mode
dev:
    @echo "Running GUI in development mode..."
    npm run dev

# Run CLI in development mode
dev-cli:
    @echo "Running CLI in development mode..."
    cargo run --bin mouse_macro

# Build CLI release binary
build-cli:
    @echo "Building CLI release binary..."
    cargo build --release --bin mouse_macro
    @echo "CLI binary: target/release/mouse_macro"

# Build GUI installer (requires icons - see ICONS_NEEDED.md)
build-gui:
    @echo "WARNING: GUI build requires icon files in src-tauri/icons/"
    @echo "See ICONS_NEEDED.md for details"
    @echo ""
    @echo "Installing frontend dependencies..."
    npm --prefix ui install
    @echo "Building GUI installer..."
    npm run build
    @echo "GUI installer: src-tauri/target/release/bundle/"

# Build CLI only (recommended)
build: build-cli
    @echo "Build complete! Run with: ./target/release/mouse_macro"
    @echo ""
    @echo "Note: To build GUI installer, run 'just build-gui' (requires icons)"

# Build both CLI and GUI
build-all: build-cli build-gui
    @echo "All builds complete"

# Run all tests
test:
    @echo "Running Rust tests..."
    cargo test --workspace
    @echo "All tests passed"

# Check code without building
check:
    @echo "Checking Rust code..."
    cargo check --workspace
    @echo "Check complete"

# Format Rust code
fmt:
    @echo "Formatting Rust code..."
    cargo fmt --all
    @echo "Code formatted"

# Run clippy linter
clippy:
    @echo "Running clippy..."
    cargo clippy --workspace
    @echo "Clippy passed"

# Check formatting
check-fmt:
    @echo "Checking formatting..."
    cargo fmt --all --check
    @echo "Formatting check passed"

# Run all CI checks
ci: check-fmt clippy test
    @echo "All CI checks passed"

# Clean build artifacts
clean:
    @echo "Cleaning Rust artifacts..."
    cargo clean
    @echo "Clean complete"

# Show project info
info:
    @echo "Project: Mouse Macro"
    @echo "Version: 0.1.0"

# Quick release build
release: build
    @echo "Release builds complete"
