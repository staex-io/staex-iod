fmt:
	cargo +nightly fmt

lint: fmt
	cargo clippy --tests --all-features -- -D warnings

update_peaq_did_proto:
	curl -s \
		https://raw.githubusercontent.com/peaqnetwork/peaq-network-did-proto-format/feature/1206361464402337_develop-data-verification-flutter-peaq-sdk-package/did_document_format.proto \
		-o src/peaq_did.proto
