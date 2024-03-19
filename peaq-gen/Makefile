update:
	subxt metadata --pallets "System,Balances,PeaqDid,PeaqRbac" --runtime-apis "" -f bytes --url $(url) \
		> assets/peaq.metadata.scale
	subxt metadata --pallets "System,Balances,PeaqDid,PeaqRbac" --runtime-apis "" -f json --url $(url) \
		> assets/peaq.metadata.json
	subxt codegen --file assets/peaq.metadata.scale \
		--no-docs | \
		rustfmt +nightly --edition=2021 --emit=stdout > src/metadata.rs

fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-features -- -D warnings
