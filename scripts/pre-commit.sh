#!/bin/sh
# pre-commit hook

echo "Running fmt check..."
cargo fmt -- --check || exit 1

echo "Running clippy..."
cargo clippy -- -D warnings || exit 1

echo "Running tests..."
cargo test || exit 1
