all: build
build:
	cargo build --workspace

.PHONY: run
run:
	cargo run

.PHONY: test
test:
	cargo test -- --test-threads=8

.PHONY: style-fix
style-fix:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy

.PHONY: clean
clean:
	cargo clean