default:
  @just --list
fmt:
  nix fmt

check:
  cargo check

lint:
  cargo clippy

build:
  cargo build

test:
  cargo test

run:
  cargo run
