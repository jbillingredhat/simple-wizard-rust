.PHONY: all build release clean test run run-example install

all: build

# Build in debug mode
build:
	cargo build

# Build in release mode
release:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean

# Run tests
test:
	cargo test

# Run the wizard
run:
	cargo run --bin simple-wizard

# Run the example installer
run-example:
	cargo run --example example_install

# Install the binaries
install:
	cargo install --path .

# Build documentation
docs:
	cargo doc --no-deps --open

# Format code
fmt:
	cargo fmt

# Run clippy linter
lint:
	cargo clippy -- -D warnings

# Check without building
check:
	cargo check
