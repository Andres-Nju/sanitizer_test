cargo clean
cargo build --target x86_64-unknown-linux-gnu
objdump -d target/x86_64-unknown-linux-gnu/debug/slice > nosanitizer_program.s