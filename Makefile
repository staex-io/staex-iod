build_provisioner:
	docker buildx build \
		--platform linux/amd64 \
		-t ghcr.io/staex-mcc/staex-iod/provisioner \
		-f deploy/Dockerfile \
		--push \
		.
