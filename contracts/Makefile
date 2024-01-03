build:
	cd $(name) && cargo contract build

deploy: build
	cd $(name) && cargo contract upload --suri //Alice -x
	cd $(name) && cargo contract instantiate --suri //Alice --args true -x --skip-confirm
	subxt metadata -f bytes > ../provisioner/$(name).metadata.scale
	cp $(name)/target/ink/did.json ../provisioner/$(name).metadata.json
	subxt codegen --file ../provisioner/$(name).metadata.scale \
		--no-docs | \
		rustfmt --edition=2021 --emit=stdout > ../provisioner/src/$(name).rs
