c:
	@cargo clean

r:
	@RUST_LOG=debug cargo run --release