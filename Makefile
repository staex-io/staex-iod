run_substrate:
	substrate-contracts-node \
		--no-telemetry --dev -d data-substrate -l info \
		--unsafe-rpc-external --rpc-external --rpc-methods unsafe

fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --workspace --all-features -- -D warnings

test:
	cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)
