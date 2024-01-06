fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --workspace -- -D warnings

test:
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)
