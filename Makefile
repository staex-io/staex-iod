run_substrate:
	substrate-contracts-node \
		--no-telemetry --dev -d data-substrate -l info \
		--unsafe-rpc-external --rpc-external --rpc-methods unsafe

build_provisioner:
	docker build -f deploy/Dockerfile -t ghcr.io/staex-mcc/staex-iod/provisioner .
