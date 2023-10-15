SHELL := /bin/bash

.PHONY: default
default: all

.PHONY: all
all: lint test build

.PHONY: build
build:
	cargo build --target wasm32-wasi --release

.PHONY: clean
clean:
	cargo clean

.PHONY: test
test: test-unit

.PHONY: lint
lint:
	cargo clippy --all-features -- -D warnings
	cargo fmt

.PHONY: test-unit
test-unit:
	RUST_LOG=$(LOG_LEVEL) cargo test --target=$$(rustc -vV | sed -n 's|host: ||p') -- --test-threads=1
