run_substrate:
	substrate-contracts-node \
		--no-telemetry --dev -d data-substrate -l info \
		--unsafe-rpc-external --rpc-external --rpc-methods unsafe
