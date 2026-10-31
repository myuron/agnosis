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

## Module Notes

### module1

- Responsible for feature 1.
- Keep public API stable.
- Add tests for every bug fix.

### module2

- Responsible for feature 2.
- Keep public API stable.
- Add tests for every bug fix.

### module3

- Responsible for feature 3.
- Keep public API stable.
- Add tests for every bug fix.

### module4

- Responsible for feature 4.
- Keep public API stable.
- Add tests for every bug fix.

### module5

- Responsible for feature 5.
- Keep public API stable.
- Add tests for every bug fix.

### module6

- Responsible for feature 6.
- Keep public API stable.
- Add tests for every bug fix.

### module7

- Responsible for feature 7.
- Keep public API stable.
- Add tests for every bug fix.

### module8

- Responsible for feature 8.
- Keep public API stable.
- Add tests for every bug fix.

### module9

- Responsible for feature 9.
- Keep public API stable.
- Add tests for every bug fix.

### module10

- Responsible for feature 10.
- Keep public API stable.
- Add tests for every bug fix.

### module11

- Responsible for feature 11.
- Keep public API stable.
- Add tests for every bug fix.

### module12

- Responsible for feature 12.
- Keep public API stable.
- Add tests for every bug fix.

### module13

- Responsible for feature 13.
- Keep public API stable.
- Add tests for every bug fix.

### module14

- Responsible for feature 14.
- Keep public API stable.
- Add tests for every bug fix.

### module15

- Responsible for feature 15.
- Keep public API stable.
- Add tests for every bug fix.

### module16

- Responsible for feature 16.
- Keep public API stable.
- Add tests for every bug fix.

### module17

- Responsible for feature 17.
- Keep public API stable.
- Add tests for every bug fix.

### module18

- Responsible for feature 18.
- Keep public API stable.
- Add tests for every bug fix.

### module19

- Responsible for feature 19.
- Keep public API stable.
- Add tests for every bug fix.

### module20

- Responsible for feature 20.
- Keep public API stable.
- Add tests for every bug fix.

### module21

- Responsible for feature 21.
- Keep public API stable.
- Add tests for every bug fix.

### module22

- Responsible for feature 22.
- Keep public API stable.
- Add tests for every bug fix.

### module23

- Responsible for feature 23.
- Keep public API stable.
- Add tests for every bug fix.

### module24

- Responsible for feature 24.
- Keep public API stable.
- Add tests for every bug fix.

### module25

- Responsible for feature 25.
- Keep public API stable.
- Add tests for every bug fix.

### module26

- Responsible for feature 26.
- Keep public API stable.
- Add tests for every bug fix.

### module27

- Responsible for feature 27.
- Keep public API stable.
- Add tests for every bug fix.

### module28

- Responsible for feature 28.
- Keep public API stable.
- Add tests for every bug fix.

### module29

- Responsible for feature 29.
- Keep public API stable.
- Add tests for every bug fix.

### module30

- Responsible for feature 30.
- Keep public API stable.
- Add tests for every bug fix.

### module31

- Responsible for feature 31.
- Keep public API stable.
- Add tests for every bug fix.

### module32

- Responsible for feature 32.
- Keep public API stable.
- Add tests for every bug fix.

### module33

- Responsible for feature 33.
- Keep public API stable.
- Add tests for every bug fix.

### module34

- Responsible for feature 34.
- Keep public API stable.
- Add tests for every bug fix.

### module35

- Responsible for feature 35.
- Keep public API stable.
- Add tests for every bug fix.

### module36

- Responsible for feature 36.
- Keep public API stable.
- Add tests for every bug fix.

### module37

- Responsible for feature 37.
- Keep public API stable.
- Add tests for every bug fix.

### module38

- Responsible for feature 38.
- Keep public API stable.
- Add tests for every bug fix.

### module39

- Responsible for feature 39.
- Keep public API stable.
- Add tests for every bug fix.

### module40

- Responsible for feature 40.
- Keep public API stable.
- Add tests for every bug fix.
