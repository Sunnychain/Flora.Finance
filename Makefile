run-tmp:
	cargo run -- --dev --tmp

run:
	cargo run -- --dev

toolchain:
	./scripts/init.sh

build:
	cargo build

check:
	SKIP_WASM_BUILD= cargo check --all --tests

check-dummy:
	BUILD_DUMMY_WASM_BINARY= cargo check

test:
	SKIP_WASM_BUILD= cargo test --all

purge:
	cargo run -- purge-chain --dev -y

restart: purge run

init: toolchain build-full

benchmark:
	cargo run --manifest-path node/Cargo.toml --features runtime-benchmarks -- benchmark --extrinsic '*' --pallet '*'

benchmark-output:
	cargo run --manifest-path node/Cargo.toml --release --features runtime-benchmarks -- benchmark --extrinsic '*' --pallet pallet_kitties --output runtime/src/weights --execution=wasm

benchmark-traits:
	cargo run --manifest-path node/Cargo.toml --release --features runtime-benchmarks -- benchmark --extrinsic '*' --pallet pallet_kitties --output pallets/kitties/src/weights.rs --template=frame-weight-template.hbs

test-benchmark:
	cargo test --manifest-path pallets/kitties/Cargo.toml --features runtime-benchmarks -- --nocapture
