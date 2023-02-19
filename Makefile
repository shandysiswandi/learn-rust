run-basic: 
	cargo run --bin basic

run-axum: 
	cargo run --bin axum

release:
	@cargo build --release

test-unit:
	cargo test --lib

e2e-test:
	cargo test --test "*"