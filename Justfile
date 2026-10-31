default:
  @just --list
fmt:
  nix fmt

check:
  cargo check

lint:
  cargo clippy --all-targets

ci-lint:
  actionlint

build:
  cargo build

test:
  cargo test

run:
  cargo run -q
