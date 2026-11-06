.DEFAULT_GOAL := check

.PHONY: run build release test fmt fmt-check lint typecheck check

run:
	cargo run --locked

build:
	cargo build --locked

release:
	cargo build --locked --release

test:
	cargo test --locked --all-targets --all-features

fmt:
	cargo fmt --all
	dprint fmt

fmt-check:
	cargo fmt --all -- --check
	dprint check

lint:
	cargo clippy --locked --all-targets --all-features -- -D warnings

typecheck:
	cargo check --locked --all-targets --all-features

check: fmt-check lint typecheck test
