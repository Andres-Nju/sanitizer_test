cargo clean
RUSTFLAGS="-Z sanitizer=address" cargo build --target x86_64-unknown-linux-gnu
objdump -d target/x86_64-unknown-linux-gnu/debug/slice > program.s