---
name: commit-message
description: Generate a Conventional Commits style commit message from staged changes. Use when the user asks to write, draft, or improve a git commit message.
---

# Commit Message

Generate a commit message that follows the Conventional Commits specification.

## Steps

1. Run `git diff --cached` to inspect the staged changes.
2. Determine the type: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`.
3. Pick a scope from the most affected module (optional).
4. Write a subject line in the imperative mood, max 72 characters.
5. Add a body explaining *why* the change was made when it is not obvious.

## Example

```
feat(parser): support YAML frontmatter in SKILL.md

Skills without frontmatter were silently skipped. Parse the
frontmatter block so name and description can be validated.
```
