#!/usr/bin/env just --justfile

fmt:
    cargo +nightly fmt -- --config format_code_in_doc_comments=true

check:
    cargo check --no-default-features
    cargo check --no-default-features --features alloc

clippy:
    cargo clippy --no-default-features
    cargo clippy --no-default-features --features alloc

test:
    cargo test --no-default-features
    cargo test --no-default-features --features alloc

precommit: fmt check clippy test
