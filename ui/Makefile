install:
	./run.sh bun install

new-dep:
	./run.sh bun install $(name)

fmt:
	./run.sh bun format

lint: fmt
	./run.sh bun lint

run:
	./run.sh bun dev --host 0.0.0.0
