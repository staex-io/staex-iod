build:
	cd $(name) && cargo contract build

deploy: build
	cd $(name) && cargo contract upload --suri //Alice -x
	cd $(name) && cargo contract instantiate --suri //Alice --args true -x --skip-confirm
	subxt metadata -f bytes > ../provisioner/src/$(name).metadata.scale
