fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-features -- -D warnings

sql_format:
	docker run --rm \
		-v "${PWD}/migrations:/migrations" \
		-v "${PWD}/scripts/sql_format.sh:/sql_format.sh" \
		--entrypoint="" \
		node:20-alpine ./sql_format.sh
