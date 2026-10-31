# CLAUDE.md

This file provides guidance to Claude Code when working in this repository.

## Project Overview

A Rust CLI project. Source code lives in `src/`, integration tests in `tests/`.

## Commands

- Build: `cargo build`
- Test: `cargo test`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Format: `cargo fmt`

## Coding Conventions

- Follow standard Rust style enforced by `rustfmt`.
- Prefer returning `Result` over panicking; avoid `unwrap()` outside tests.
- Keep functions small and focused; extract helpers when logic grows.
- Write doc comments (`///`) for public items.

## Testing

- Add unit tests in the same file under `#[cfg(test)] mod tests`.
- Add integration tests under `tests/`.
- Run `cargo test` before committing.

## Git

- Use Conventional Commits (see the `commit-message` skill).
- Keep commits focused on a single change.

## Skills

Project skills are defined in `.claude/skills/`:

- `commit-message`: draft Conventional Commits messages
- `pr-review`: review pull requests with a checklist
- `rust-test-runner`: run tests and summarize results
- `api-doc-generator`: generate API documentation
- `changelog-writer`: write changelog entries
