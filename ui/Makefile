install:
	./run.sh bun install

new_dep:
	./run.sh bun install $(name)

format:
	./run.sh bun format

lint: format
	./run.sh bun lint

run:
	./run.sh bun dev --host 0.0.0.0

run_docker:
	./run.sh bun dev-docker --host 0.0.0.0

build:
	./run.sh bun run build

.PHONY: rsync
rsync:
	rsync -a --delete dist/ he1:/srv/iod/
