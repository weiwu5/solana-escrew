.PHONY: help build build-bpf test test-bpf clean fmt lint check all install deploy

# Default target
help:
	@echo "Solana BPF Program Template - Available Commands:"
	@echo ""
	@echo "  make build       - Build the program for native testing"
	@echo "  make build-bpf   - Build the program for BPF deployment"
	@echo "  make test        - Run native tests"
	@echo "  make test-bpf    - Run BPF integration tests"
	@echo "  make check       - Run cargo check (fast compile check)"
	@echo "  make fmt         - Format code with rustfmt"
	@echo "  make lint        - Run clippy linter"
	@echo "  make clean       - Remove build artifacts"
	@echo "  make all         - Run fmt, lint, build, and test"
	@echo "  make install     - Install development dependencies"
	@echo "  make deploy-dev  - Deploy to Solana devnet (requires setup)"
	@echo ""

# Build for native testing
build:
	@echo "Building for native testing..."
	cargo build

# Build for BPF deployment
build-bpf:
	@echo "Building for BPF deployment..."
	cargo build-bpf

# Build release version
build-release:
	@echo "Building release version..."
	cargo build --release

# Run native tests
test:
	@echo "Running native tests..."
	cargo test

# Run BPF integration tests
test-bpf:
	@echo "Running BPF integration tests..."
	cargo test-bpf

# Fast compile check
check:
	@echo "Running cargo check..."
	cargo check

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Check if code is formatted
fmt-check:
	@echo "Checking code formatting..."
	cargo fmt -- --check

# Run clippy linter
lint:
	@echo "Running clippy linter..."
	cargo clippy --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Removing BPF build artifacts..."
	rm -rf target/deploy

# Run all checks (format, lint, build, test)
all: fmt lint build test
	@echo "All checks passed!"

# Install development dependencies
install:
	@echo "Installing Rust toolchain..."
	rustup install stable
	rustup default stable
	@echo "Installing rustfmt and clippy..."
	rustup component add rustfmt clippy
	@echo "Installing cargo-watch (optional)..."
	cargo install cargo-watch || true
	@echo "Installing cargo-audit (optional)..."
	cargo install cargo-audit || true
	@echo "Development dependencies installed!"
	@echo ""
	@echo "Note: You still need to install Solana CLI tools manually:"
	@echo "  sh -c \"\$$(curl -sSfL https://release.solana.com/stable/install)\""

# Verify installation
verify:
	@echo "Verifying installation..."
	@rustc --version
	@cargo --version
	@solana --version || echo "Warning: Solana CLI not found"
	@echo "Installation verified!"

# Watch mode - automatically rebuild on file changes (requires cargo-watch)
watch:
	@echo "Watching for changes (requires cargo-watch)..."
	cargo watch -x build -x test

# Run security audit (requires cargo-audit)
audit:
	@echo "Running security audit..."
	cargo audit

# Deploy to devnet (requires Solana CLI configured for devnet)
deploy-dev:
	@echo "Deploying to Solana devnet..."
	@echo "Checking Solana configuration..."
	@solana config get
	@echo ""
	@echo "Building BPF program..."
	cargo build-bpf
	@echo ""
	@echo "Deploying program..."
	solana program deploy target/deploy/bpf_program_template.so

# Show program info (requires PROGRAM_ID environment variable)
program-info:
	@if [ -z "$(PROGRAM_ID)" ]; then \
		echo "Error: PROGRAM_ID environment variable not set"; \
		echo "Usage: make program-info PROGRAM_ID=<your_program_id>"; \
		exit 1; \
	fi
	@echo "Program information for $(PROGRAM_ID):"
	@solana program show $(PROGRAM_ID)

# Quick development loop
dev: fmt build test
	@echo "Development build and test complete!"

# Pre-commit checks
pre-commit: fmt-check lint test
	@echo "Pre-commit checks passed!"
