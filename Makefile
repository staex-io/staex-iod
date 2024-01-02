fmt:
	cd contracts/did && cargo +nightly fmt
	cd provisioner && cargo +nightly fmt

lint: fmt
	cd contracts/did && cargo clippy --tests --workspace -- -D warnings
	cd provisioner && cargo clippy --tests --workspace -- -D warnings

test:
	cd contracts/did && cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)
	cd provisioner && cargo test --jobs 1 -- --nocapture --test-threads 1 $(name)

run_substrate:
	# substrate-contracts-node --no-telemetry --dev -d data-substrate -l info
	substrate-contracts-node --no-telemetry --dev -d data-substrate
