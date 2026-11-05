.PHONY: backend-check backend-test backend-lint run

backend-check:
	cargo check --workspace

# интеграционным тестам нужен Docker или TEST_DATABASE_URL
backend-test:
	cargo test --workspace

backend-lint:
	cargo fmt --all --check
	cargo clippy --workspace --all-targets --features library-web/smtp -- -D warnings

run:
	cargo run -p library-web
