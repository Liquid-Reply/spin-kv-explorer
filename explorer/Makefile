SHELL := /bin/bash
GO := tinygo
TARGET := explorer.wasm

SRC = $(shell find . -type f -name '*.go' -not -path "./vendor/*")

.PHONY: default
default: all

.PHONY: all
all: lint test build

build:
	$(GO) build -target=wasi -gc=leaking -o $(TARGET) main.go

.PHONY: clean
clean:
	rm $(TARGET)

.PHONY: test
test: test-unit

.PHONY: lint
lint:
	@echo "no linting for go"
	gofmt -l -w $(SRC)

.PHONY: test-unit
test-unit:
	$(GO) test -v
