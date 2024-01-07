build: clean
	cargo build --release

clean:
	rm -rf mofish.db
	rm -rf target/release/mofish

run:
	RUST_LOG=info ./target/release/mofish -s

fix: 
	cargo fix --allow-dirty --allow-staged --all-features
	cargo fmt --all -- --check