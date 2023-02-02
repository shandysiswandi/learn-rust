run-basic: 
	cargo run --bin basic

run-actix: 
	cargo run --bin rest_actix_web

release:
	@cargo build --release