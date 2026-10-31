---
name: pr-review
description: Review a GitHub pull request for correctness, readability, and test coverage. Use when the user asks to review a PR or provides a PR number or URL.
allowed-tools: Bash(gh pr view:*), Bash(gh pr diff:*), Read, Grep
---

# PR Review

Review the given pull request and report findings grouped by severity.

## Steps

1. Fetch PR metadata with `gh pr view <number>`.
2. Fetch the diff with `gh pr diff <number>`.
3. Check each changed file against the checklist in [references/checklist.md](references/checklist.md).
4. Report findings as `critical`, `major`, or `minor`.

## Output format

- **critical**: bugs, security issues, data loss
- **major**: missing tests, unclear error handling
- **minor**: naming, style, comments
