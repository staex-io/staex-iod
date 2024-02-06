fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-features -- -D warnings
