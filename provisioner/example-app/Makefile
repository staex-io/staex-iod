format:
	@go fmt

lint: format
	@docker run --rm -v "${PWD}:/app" -w /app \
		golangci/golangci-lint:v1.57.2 \
		golangci-lint run main.go

build:
	go build -o example-app main.go
