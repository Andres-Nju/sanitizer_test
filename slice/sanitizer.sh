cargo clean
RUSTFLAGS="-Z sanitizer=address" cargo run --target x86_64-unknown-linux-gnu