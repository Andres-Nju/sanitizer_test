cargo clean
RUSTFLAGS='-Zsanitizer=memory -Zsanitizer-memory-track-origins' \
  RUSTDOCFLAGS='-Zsanitizer=memory -Zsanitizer-memory-track-origins' \
cargo run -Zbuild-std --target x86_64-unknown-linux-gnu