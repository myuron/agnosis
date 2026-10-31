---
name: changelog-writer
description: Update CHANGELOG.md following the Keep a Changelog format based on commits since the last tag. Use when preparing a release or when the user asks to update the changelog.
---

# Changelog Writer

## Steps

1. Find the latest tag: `git describe --tags --abbrev=0`.
2. List commits since that tag: `git log <tag>..HEAD --oneline`.
3. Group commits into sections:
   - `Added` for `feat:`
   - `Fixed` for `fix:`
   - `Changed` for `refactor:` and `perf:`
   - `Removed` for commits that delete features
4. Insert a new `## [Unreleased]` section at the top of `CHANGELOG.md`.

## Rules

- Write entries for users, not developers. Skip `chore:` and `test:` commits.
- Keep each entry to one line.
