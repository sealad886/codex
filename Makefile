# Makefile for Codex CLI

.PHONY: help setup build test lint format clean run install docker-build docker-run
.DEFAULT_GOAL := help

# Configuration
CARGO_TARGET_DIR ?= target
BINARY_NAME = codex
OUT_DIR = out

help: ## Display this help message
	@echo "Codex CLI - OpenAI Coding Agent"
	@echo ""
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}'

setup: ## Install dependencies and prepare development environment
	@echo "Setting up Codex CLI development environment..."
	@rustup show active-toolchain
	@command -v just >/dev/null 2>&1 || cargo install just
	@command -v cargo-insta >/dev/null 2>&1 || cargo install cargo-insta
	@cd codex-rs && cargo fetch
	@echo "✅ Setup complete!"

build: ## Build the project
	@echo "Building Codex CLI..."
	@cd codex-rs && cargo build --release
	@echo "✅ Build complete! Binary available at: codex-rs/target/release/$(BINARY_NAME)"

test: ## Run all tests
	@echo "Running tests..."
	@cd codex-rs && CODEX_SANDBOX_NETWORK_DISABLED=1 cargo test --all-features
	@echo "✅ All tests passed!"

lint: ## Run linter (clippy)
	@echo "Running linter..."
	@cd codex-rs && cargo clippy --all-features --tests
	@echo "✅ Linting complete!"

format: ## Format code
	@echo "Formatting code..."
	@cd codex-rs && just fmt
	@echo "✅ Code formatted!"

clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	@cd codex-rs && cargo clean
	@rm -rf $(OUT_DIR)
	@echo "✅ Clean complete!"

run: ## Run Codex CLI (requires OpenAI authentication)
	@echo "Running Codex CLI..."
	@cd codex-rs && cargo run --release --bin $(BINARY_NAME)

install: build ## Install Codex CLI globally
	@echo "Installing Codex CLI..."
	@install -m 755 codex-rs/target/release/$(BINARY_NAME) /usr/local/bin/$(BINARY_NAME)
	@echo "✅ Codex CLI installed to /usr/local/bin/$(BINARY_NAME)"

examples: ## Generate example outputs
	@echo "Generating example outputs..."
	@mkdir -p $(OUT_DIR)
	@cd codex-rs && cargo run --release --bin $(BINARY_NAME) -- --help > ../$(OUT_DIR)/help-output.txt
	@cd codex-rs && cargo run --release --bin $(BINARY_NAME) -- completion bash > ../$(OUT_DIR)/bash-completion.sh
	@cd codex-rs && cargo run --release --bin $(BINARY_NAME) -- completion zsh > ../$(OUT_DIR)/zsh-completion.zsh
	@cd codex-rs && cargo run --release --bin $(BINARY_NAME) -- completion fish > ../$(OUT_DIR)/fish-completion.fish
	@echo "✅ Example outputs generated in $(OUT_DIR)/"

docker-build: ## Build Docker image
	@echo "Building Docker image..."
	@docker build -t codex-cli:latest -f Dockerfile .
	@echo "✅ Docker image built: codex-cli:latest"

docker-run: ## Run Codex CLI in Docker
	@echo "Running Codex CLI in Docker..."
	@docker run -it --rm -v $(PWD):/workspace codex-cli:latest

# Development shortcuts
dev-test: ## Run tests for current project only (faster for development)
	@cd codex-rs && CODEX_SANDBOX_NETWORK_DISABLED=1 cargo test -p codex-core

dev-build: ## Quick development build (debug mode)
	@cd codex-rs && cargo build

dev-run: ## Run in development mode
	@cd codex-rs && cargo run --bin $(BINARY_NAME)

# CI/Automation targets
ci-test: setup test lint ## Full CI test suite
	@echo "✅ All CI checks passed!"

release-build: ## Build optimized release binaries
	@echo "Building release binaries..."
	@cd codex-rs && cargo build --release --all-features
	@echo "✅ Release build complete!"