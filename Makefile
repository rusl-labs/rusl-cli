.PHONY: help install build release test check clippy fmt fmt-check verify openapi-refresh openapi-generate

help:
	@printf '%s\n' \
		'make install           Install the rusl CLI into ~/.cargo/bin' \
		'make build             Build the workspace' \
		'make release           Build the workspace in release mode' \
		'make test              Run workspace tests' \
		'make check             Run cargo check for the workspace' \
		'make clippy            Run clippy with warnings denied' \
		'make fmt               Format the workspace' \
		'make fmt-check         Check formatting' \
		'make verify            Run the full required verification suite' \
		'make openapi-refresh   Refresh the committed OpenAPI snapshot' \
		'make openapi-generate  Regenerate the checked-in Rust OpenAPI client'

install:
	cargo install --path crates/rusl-cli --force

build:
	cargo build --workspace

release:
	cargo build --workspace --release

test:
	cargo test --workspace

check:
	cargo check --workspace

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --check

verify:
	cargo check --workspace
	cargo clippy --workspace --all-targets -- -D warnings
	cargo test --workspace
	cargo fmt --check
	cargo build --workspace --release

openapi-refresh:
	./scripts/refresh-openapi-spec.sh

openapi-generate:
	./scripts/generate-rust-client.sh
