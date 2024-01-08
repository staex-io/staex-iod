fmt:
	cd $(name) && cargo +nightly fmt

lint: fmt
	cd $(name) && cargo clippy --tests --workspace -- -D warnings

test:
	cd $(name) && cargo test --all-features --jobs 1 -- --nocapture --test-threads 1 $(test)

build:
	cd $(name) && cargo contract build

deploy: build
	cd $(name) && cargo contract upload --suri //Alice -x
	cd $(name) && cargo contract instantiate --suri //Alice --args true -x --skip-confirm
	subxt metadata -f bytes > ../../provisioner/assets/$(name).metadata.scale
	cp $(name)/target/ink/did.json ../../provisioner/assets/$(name).metadata.json
	subxt codegen --file ../../provisioner/assets/$(name).metadata.scale \
		--no-docs | \
		rustfmt --edition=2021 --emit=stdout > ../../provisioner/src/$(name).rs
