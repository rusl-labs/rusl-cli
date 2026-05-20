.PHONY: help install build release test check clippy fmt fmt-check verify prepare-release openapi-refresh openapi-generate

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
		'make prepare-release   Bump first-party crate versions; requires VERSION=x.y.z' \
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

prepare-release:
	@if [ -z "$(VERSION)" ]; then \
		printf '%s\n' 'VERSION is required. Example: make prepare-release VERSION=0.2.0'; \
		exit 1; \
	fi
	./scripts/prepare-release.sh "$(VERSION)"

openapi-refresh:
	./scripts/refresh-openapi-spec.sh

openapi-generate:
	./scripts/generate-rust-client.sh
