# Makefile for idionautic-agent

# Variables
WASM_TARGET := web
BUILD_DIR := pkg

# Targets
.PHONY: all build test lint clean

# Default target: lint, test, and build
all: lint test build

# Build the WebAssembly package with wasm-pack
build:
	wasm-pack build --target $(WASM_TARGET)

# Run tests with wasm-pack in headless mode with Chrome
test:
	wasm-pack test --headless --chrome

# Lint Rust code
lint:
	cargo fmt -- --check
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean
	rm -rf $(BUILD_DIR)
