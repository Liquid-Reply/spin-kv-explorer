.PHONY: default
default:
	cargo build --manifest-path=env-explorer/Cargo.toml --target wasm32-wasi --release

# .PHONY: test
# test: lint test-unit

# .PHONY: lint
# lint:
# 	cargo clippy --manifest-path=env-explorer/Cargo.toml --all-features -- -D warnings
# 	cargo fmt --manifest-path=env-explorer/Cargo.toml -- --check

# .PHONY: test-unit
# test-unit:
# 	RUST_LOG=$(LOG_LEVEL) cargo test --target=$$(rustc -vV | sed -n 's|host: ||p') -- --test-threads=1
