FROM rust:1-bookworm

# System libraries required to build the workspace:
# - pkg-config + libssl-dev resolve the openssl-sys build dependency (via reqwest/native-tls transitive deps)
# - git + curl support source checkout and the OpenAPI refresh/generate scripts
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        git \
        curl \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# The crates target edition 2024 (needs Rust 1.85+); the rust:1 image satisfies that.
# Ensure the verification components used by `make verify` are present.
RUN rustup component add rustfmt clippy
