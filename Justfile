default:
  @just --list
fmt:
  nix fmt

check:
  cargo check

lint:
  cargo clippy --all-targets

build:
  cargo build

test:
  cargo test

run:
  cargo run -q
