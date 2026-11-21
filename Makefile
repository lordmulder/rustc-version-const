export RUSTFLAGS=-Dwarnings

.PHONY: all check clippy test run doc clean

all: clean check clippy test run doc

check:
	cargo fmt --all --check
	cargo check --locked --all-targets --no-default-features
	cargo check --locked --all-targets --all-features

clippy:
	cargo clippy --locked --all-targets --no-default-features -- -Dwarnings
	cargo clippy --locked --all-targets --all-features -- -Dwarnings

test:
	cargo test --locked --no-default-features
	cargo test --locked --all-features
	cargo test --locked --release --no-default-features
	cargo test --locked --release --all-features

run:
	cargo run --no-default-features --example rustc_version_example
	cargo run --all-features --example rustc_version_example
	cargo run --release --no-default-features --example rustc_version_example
	cargo run --release --all-features --example rustc_version_example

doc:
	cargo doc --all-features

clean:
	cargo clean
