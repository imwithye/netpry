build:
	cd proxy/_webui && pnpm build
	go build -o dist/netpry cmd/cli.go

deps:
	cd proxy/_webui && pnpm install
	go get ./...

fmt:
	cd proxy/_webui && pnpm run format
	go fmt ./...

.PHONY: build deps fmt
