default: help

# Show available commands
help:
	@echo "Just commands:" && \
	printf "\n  setup-all           Install Rust toolchain, omni-node, chain-spec-builder\n" && \
	printf "  setup-rust          Install/pin Rust toolchain from script\n" && \
	printf "  install-omni        Install polkadot-omni-node\n" && \
	printf "  install-chain-spec  Install staging-chain-spec-builder\n" && \
	printf "\n  build               Build kitchensink runtime & workspace (production)\n" && \
	printf "  chain-spec          Generate chain_spec.json in kitchensink-parachain\n" && \
	printf "  start-dev           Start omni-node in --dev using generated chain spec\n" && \
	printf "  spawn-omni          Spawn Zombienet using omni-node TOML\n" && \
	printf "  spawn-node          Spawn Zombienet using template node TOML\n" && \
	printf "\n  fmt                 Run cargo fmt across workspace\n" && \
	printf "  clippy              Run cargo clippy with -D warnings\n" && \
	printf "  clean               Clean kitchensink workspace targets\n" && \
	printf "\n  test-pallet NAME=custom-pallet  Run unit tests for a specific pallet\n" && \
	printf "  test-runtime        Run runtime crate tests (if any)\n" && \
	printf "  test-e2e            Placeholder for e2e tests (Node/TS)\n"

# One-shot environment setup
setup-all: setup-rust install-omni install-chain-spec

setup-rust:
	bash scripts/zero-to-hero/setup-rust.sh

install-omni:
	bash scripts/zero-to-hero/install-omni-node.sh

install-chain-spec:
	bash scripts/zero-to-hero/install-chain-spec-builder.sh

# Build the kitchensink workspace with production profile
build:
	cd kitchensink-parachain && cargo build --profile production --workspace

# Generate chain_spec.json for kitchensink-parachain
chain-spec:
	cd kitchensink-parachain && ../scripts/zero-to-hero/generate-chain-spec.sh

# Start omni-node in --dev using generated chain_spec.json
start-dev:
	cd kitchensink-parachain && ../scripts/zero-to-hero/start-node.sh

# Spawn Zombienet network using omni-node config
spawn-omni:
	cd kitchensink-parachain && zombienet --provider native spawn zombienet-omni-node.toml | cat

# Spawn Zombienet network using template node config
spawn-node:
	cd kitchensink-parachain && zombienet --provider native spawn zombienet.toml | cat

# Linting and hygiene
fmt:
	cargo fmt --all --manifest-path kitchensink-parachain/Cargo.toml

clippy:
	cargo clippy --all-targets --all-features --manifest-path kitchensink-parachain/Cargo.toml -- -D warnings

clean:
	cargo clean --manifest-path kitchensink-parachain/Cargo.toml

# Tests
test-pallet NAME="custom-pallet":
	cargo test --manifest-path kitchensink-parachain/pallets/{{NAME}}/Cargo.toml

test-runtime:
	cargo test --manifest-path kitchensink-parachain/runtime/Cargo.toml || true

test-e2e:
	@echo "No e2e tests found. Add tests under tutorials/<slug>/tests and wire them here."


