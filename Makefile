run_substrate:
	substrate-contracts-node \
		--no-telemetry --dev -d data-substrate -l info \
		--finalize-delay-sec 10
