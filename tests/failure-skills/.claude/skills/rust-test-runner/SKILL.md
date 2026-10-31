---
name: rust-test-runner
description: Run cargo tests, summarize failures, and suggest fixes. Use when the user asks to run tests or fix failing tests in a Rust project.
allowed-tools: Bash(cargo test:*), Bash(./scripts/summarize.sh:*), Read, Edit
---

# Rust Test Runner

## Steps

1. Run `cargo test 2>&1 | tee /tmp/cargo-test.log`.
2. Summarize the result with `scripts/summarize.sh /tmp/cargo-test.log`.
3. For each failing test, open the test source and the code under test.
4. Propose a minimal fix and re-run only the failing test with
   `cargo test <test_name>`.

## Notes

- Do not modify test expectations unless the user confirms the spec changed.
- Prefer fixing the implementation over adding `#[ignore]`.
