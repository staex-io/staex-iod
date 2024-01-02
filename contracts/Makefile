build:
	cd $(name) && cargo contract build

deploy: build
	cd $(name) && cargo contract upload --suri //Alice -x
	cd $(name) && cargo contract instantiate --suri //Alice --args true -x --skip-confirm
	# subxt metadata -f bytes --pallets Contracts --runtime-apis "-" > ../provisioner/src/$(name).metadata.scale
	subxt metadata -f bytes > ../provisioner/$(name).metadata.scale
	cp $(name)/target/ink/did.json ../provisioner/$(name).metadata.json
	#subxt codegen --file ../provisioner/src/$(name).metadata.scale | \
	#	rustfmt --edition=2018 --emit=stdout > ../provisioner/src/$(name)_gen.rs
