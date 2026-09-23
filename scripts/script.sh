#!/bin/bash

echo "cargo test: "
cargo test
echo "cargo clippy test: "
cargo clippy --all-targets
echo "cargo run 127.0.0.1 8080: "
cargo run -- 127.0.0.1 8080
echo "cargo run without args: "
cargo run -- || echo $?
echo "cargo run with invalid args: "
cargo run -- 999.999.999.999 12 || echo $?