#!/usr/bin/env bash

cargo fmt --all
cargo build --all
cargo build --examples
cargo test --all
cargo run -- -i ./examples/README.md -o ./README.md -r ./examples
