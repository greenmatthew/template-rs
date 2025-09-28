# Justfile for template-rs

name := "template-rs"
release_binary := "target/release" / name
install_dir := "/usr/local/bin"
windows_cwd := `wslpath -aw $(pwd)`

# Default recipe to run when just is called without arguments
_default: help

# Display available commands and their descriptions
@help:
    just --list

# Build the project in debug mode
@build:
    cargo build

# Build the project in release mode
@build-release:
    cargo build --release

# Run the binary in debug mode
@run *args:
    cargo run -- {{args}}

# Clean the build artifacts
@clean:
    cargo clean

# Check code without building
@check:
    cargo check

# Run tests
@test:
    cargo test

# Formats code consistently
@fmt:
    cargo fmt

# Run clippy with pedantic lints
@clippy:
    cargo clippy -- -W clippy::pedantic

# Install using cargo (user-level installation)
install:
    cargo install --path .

# Install the binary to system-wide location (requires sudo)
@install-system: build-release
    echo "Installing {{name}} to {{install_dir}}"
    sudo cp {{release_binary}} {{install_dir}}
    echo "Installation complete!"

# Install using cargo (user-level installation) to Windows
@install-windows:
    echo "Installing {{name}} to Windows using Windows Cargo..."
    powershell.exe -Command "cd \{{windows_cwd}}; cargo install --path ."

# Install to both WSL and Windows
@install-all: install install-windows
    echo "Installed to both WSL and Windows environments!"

# Uninstall the application from cargo bin directory
@uninstall:
    echo "Removing {{name}} from cargo bin directory"
    cargo uninstall {{name}}
    echo "Uninstallation complete!"

# Uninstall the application from system-wide location (requires sudo)
@uninstall-system:
    echo "Removing {{name}} from {{install_dir}}"
    sudo rm -f {{install_dir}}/{{name}}
    echo "Uninstallation complete!"

# Uninstall the application from Windows
@uninstall-windows:
    echo "Removing {{name}} from Windows..."
    powershell.exe -Command "cd \{{windows_cwd}}; cargo uninstall {{name}}"
    echo "Windows uninstallation complete!"

# Uninstall the application from WSL and Windows
@uninstall-all: uninstall uninstall-system uninstall-windows
    echo "Uninstalled from both WSL and Windows environments!"