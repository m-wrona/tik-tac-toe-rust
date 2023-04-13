
.PHONY: default
default: lint test

.PHONY: test
test:
	@cargo test

.PHONY: run
run:
	@cargo run

.PHONY: lint
lint: fmt clippy

.PHONY: fmt
fmt:
	@cargo fmt

.PHONY: clippy
clippy:
	@cargo clippy -- -D warnings