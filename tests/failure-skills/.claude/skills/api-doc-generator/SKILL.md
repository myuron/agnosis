---
name: api-doc-generator
description: Generate Markdown API documentation for public Rust items (pub fn, pub struct, pub enum). Use when the user asks to document an API or generate reference docs.
---

# API Doc Generator

Generate reference documentation for the public API of a Rust crate.

## Steps

1. Find public items with `grep -rn "^pub " src/`.
2. For each item, read its signature and existing `///` doc comments.
3. Fill in [templates/item.md](templates/item.md) for each item.
4. Write the result to `docs/api.md`, grouped by module.

## Rules

- Do not invent behavior that is not visible in the code.
- If an item has no doc comment, mark it with `TODO: undocumented`.
