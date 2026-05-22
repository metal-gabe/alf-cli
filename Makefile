.DEFAULT_GOAL := help

.PHONY: build build-release check clean clippy fmt fmt-fix help install lint run snap test test-cov test-fresh watch

help:
	@echo "Usage: make <target>" | bat --language=help --style=plain
	@echo "" | bat --language=help --style=plain
	@echo "Targets:" | bat --language=help --style=plain
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-16s %s\n", $$1, $$2}' | bat --language=help --style=plain

build: ## Build (debug)
	cargo build

build-release: ## Build (release)
	cargo build --release

check: ## Check for compile errors without building
	cargo check

clean: ## Remove build artifacts
	cargo clean

clippy: ## Run Clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Check formatting without modifying files
	cargo fmt --check

fmt-fix: ## Format source files
	cargo fmt

install: ## Install the binary locally via cargo
	cargo install --path .

lint: fmt clippy ## Run all lint checks (fmt + clippy)

run: ## Run the TUI (debug build)
	cargo run

snap: ## Review pending insta snapshots interactively
	cargo insta review

test: ## Run all tests via nextest
	cargo nextest run

test-cov: ## Generate code coverage report (HTML) via cargo-llvm-cov
	cargo llvm-cov nextest --all-features --open

test-fresh: ## Run all tests via nextest with no cache
	cargo nextest run --no-capture

watch: ## Watch for changes and rebuild
	cargo watch -x build
