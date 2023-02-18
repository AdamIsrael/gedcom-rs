# prog :=wc

build:
	cargo build
lint:
	cargo fmt
	cargo clippy -- -D warnings
test:
	cargo test
	cargo fmt
	cargo clippy -- -D warnings

all: build test

help:
	@echo "usage: make $(prog) [debug=1]"
