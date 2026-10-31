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

## More Examples

### Example 1

```
fix(module1): handle edge case 1

Previously the edge case 1 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 2

```
fix(module2): handle edge case 2

Previously the edge case 2 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 3

```
fix(module3): handle edge case 3

Previously the edge case 3 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 4

```
fix(module4): handle edge case 4

Previously the edge case 4 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 5

```
fix(module5): handle edge case 5

Previously the edge case 5 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 6

```
fix(module6): handle edge case 6

Previously the edge case 6 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 7

```
fix(module7): handle edge case 7

Previously the edge case 7 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 8

```
fix(module8): handle edge case 8

Previously the edge case 8 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 9

```
fix(module9): handle edge case 9

Previously the edge case 9 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 10

```
fix(module10): handle edge case 10

Previously the edge case 10 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 11

```
fix(module11): handle edge case 11

Previously the edge case 11 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 12

```
fix(module12): handle edge case 12

Previously the edge case 12 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 13

```
fix(module13): handle edge case 13

Previously the edge case 13 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 14

```
fix(module14): handle edge case 14

Previously the edge case 14 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 15

```
fix(module15): handle edge case 15

Previously the edge case 15 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 16

```
fix(module16): handle edge case 16

Previously the edge case 16 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 17

```
fix(module17): handle edge case 17

Previously the edge case 17 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 18

```
fix(module18): handle edge case 18

Previously the edge case 18 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 19

```
fix(module19): handle edge case 19

Previously the edge case 19 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 20

```
fix(module20): handle edge case 20

Previously the edge case 20 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 21

```
fix(module21): handle edge case 21

Previously the edge case 21 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 22

```
fix(module22): handle edge case 22

Previously the edge case 22 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 23

```
fix(module23): handle edge case 23

Previously the edge case 23 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 24

```
fix(module24): handle edge case 24

Previously the edge case 24 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 25

```
fix(module25): handle edge case 25

Previously the edge case 25 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 26

```
fix(module26): handle edge case 26

Previously the edge case 26 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 27

```
fix(module27): handle edge case 27

Previously the edge case 27 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 28

```
fix(module28): handle edge case 28

Previously the edge case 28 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 29

```
fix(module29): handle edge case 29

Previously the edge case 29 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 30

```
fix(module30): handle edge case 30

Previously the edge case 30 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 31

```
fix(module31): handle edge case 31

Previously the edge case 31 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 32

```
fix(module32): handle edge case 32

Previously the edge case 32 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 33

```
fix(module33): handle edge case 33

Previously the edge case 33 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 34

```
fix(module34): handle edge case 34

Previously the edge case 34 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 35

```
fix(module35): handle edge case 35

Previously the edge case 35 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 36

```
fix(module36): handle edge case 36

Previously the edge case 36 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 37

```
fix(module37): handle edge case 37

Previously the edge case 37 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 38

```
fix(module38): handle edge case 38

Previously the edge case 38 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 39

```
fix(module39): handle edge case 39

Previously the edge case 39 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 40

```
fix(module40): handle edge case 40

Previously the edge case 40 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 41

```
fix(module41): handle edge case 41

Previously the edge case 41 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 42

```
fix(module42): handle edge case 42

Previously the edge case 42 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 43

```
fix(module43): handle edge case 43

Previously the edge case 43 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 44

```
fix(module44): handle edge case 44

Previously the edge case 44 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 45

```
fix(module45): handle edge case 45

Previously the edge case 45 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 46

```
fix(module46): handle edge case 46

Previously the edge case 46 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 47

```
fix(module47): handle edge case 47

Previously the edge case 47 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 48

```
fix(module48): handle edge case 48

Previously the edge case 48 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 49

```
fix(module49): handle edge case 49

Previously the edge case 49 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 50

```
fix(module50): handle edge case 50

Previously the edge case 50 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 51

```
fix(module51): handle edge case 51

Previously the edge case 51 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 52

```
fix(module52): handle edge case 52

Previously the edge case 52 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 53

```
fix(module53): handle edge case 53

Previously the edge case 53 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 54

```
fix(module54): handle edge case 54

Previously the edge case 54 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 55

```
fix(module55): handle edge case 55

Previously the edge case 55 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 56

```
fix(module56): handle edge case 56

Previously the edge case 56 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 57

```
fix(module57): handle edge case 57

Previously the edge case 57 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 58

```
fix(module58): handle edge case 58

Previously the edge case 58 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 59

```
fix(module59): handle edge case 59

Previously the edge case 59 caused a panic. Guard against it
and return a descriptive error instead.
```

### Example 60

```
fix(module60): handle edge case 60

Previously the edge case 60 caused a panic. Guard against it
and return a descriptive error instead.
```
