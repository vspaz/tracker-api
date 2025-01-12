TARGET=tracker

all: build
build:
	cargo build --workspace

.PHONY: build-image
build-image:
	docker build -t $(TARGET) .

.PHONY: run-container
run-container:
	docker run -dp 9000:9000 $(TARGET)

.PHONY: stop-container
stop-container:
	@if [ -n "$$(docker ps -q --filter ancestor=$(TARGET))" ]; then \
		docker stop $$(docker ps -q --filter ancestor=$(TARGET)); \
	else \
		echo "No containers found for image $(TARGET)"; \
	fi

.PHONY: remove-image
remove-image:
	docker rmi -f $(TARGET):latest

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
	cargo clippy -- -D warnings

.PHONY: clean
clean:
	cargo clean