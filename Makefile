build_provisioner:
	docker buildx build \
		--platform linux/amd64 \
		-t ghcr.io/staex-io/staex-iod/provisioner:$(shell date +%d%m%Y%H%M) \
		-f deploy/Dockerfile \
		--push \
		.
