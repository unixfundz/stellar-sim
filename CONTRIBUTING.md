# Contributing to stellar-sim

Thank you for contributing! This guide explains everything you need to know.

---

## Table of Contents
1. [Getting Started](#getting-started)
2. [Finding an Issue](#finding-an-issue)
3. [Development Workflow](#development-workflow)
4. [Code Style](#code-style)
5. [Commit Messages](#commit-messages)
6. [Pull Request Process](#pull-request-process)
7. [Review Criteria](#review-criteria)

---

## Getting Started

```bash
git clone https://github.com/unixfundz/stellar-sim
cd stellar-sim
cargo build
cargo test
```

Requirements:
- Rust stable (https://rustup.rs)
- Git

---

## Finding an Issue

Browse [open issues](../../issues) and pick one that is not assigned.

| Label | Meaning |
|---|---|
| `good first issue` | No prior Soroban knowledge needed |
| `help wanted` | Moderate complexity, some Rust helpful |
| `core` | Requires Soroban/XDR knowledge |

**Always comment on the issue to claim it before starting work.**
This prevents duplicate effort.

---

## Development Workflow

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/stellar-sim

# 2. Create a branch named after the issue
git checkout -b feat/issue-42-json-output

# 3. Make your changes

# 4. Format and lint before every commit
cargo fmt
cargo clippy -- -D warnings

# 5. Run tests
cargo test

# 6. Commit using conventional commits (see below)
git commit -m "feat(cli): add --json output flag (#42)"

# 7. Push and open PR
git push origin feat/issue-42-json-output
```

---

## Code Style

- Run `cargo fmt` before every commit — no exceptions
- Run `cargo clippy -- -D warnings` — fix all warnings before submitting
- Every `pub fn` must have a `///` doc comment with description, params, and return value
- New features must include at least one test
- Test names must be descriptive: `test_decode_vm_traps`, not `test1`
- No `unwrap()` in production code — use `?` or explicit error handling
- No `println!` in library code — use `tracing::info!` or `tracing::debug!`

---

## Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org):

```
<type>(<scope>): <short description> (#issue)
```

**Types:**
| Type | When to use |
|---|---|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `test` | Adding or fixing tests |
| `refactor` | Code change with no behaviour change |
| `ci` | CI/CD changes |
| `chore` | Maintenance tasks |

**Examples:**
```
feat(cli): add --json output flag (#32)
fix(wasm): validate magic bytes before parsing (#17)
docs(contributing): add commit message guide
test(diagnostics): add test_decode_vm_traps
```

---

## Pull Request Process

1. Fill out the PR template completely
2. Link the issue it closes with `Closes #N`
3. Make sure CI passes (fmt, clippy, tests)
4. Request review — a maintainer will review within 3 days
5. Address all review comments before merging
6. Squash commits if requested

**PRs that do not pass `cargo clippy -- -D warnings` will not be reviewed.**

---

## Review Criteria

Reviewers will check:
- [ ] Code is correct and handles edge cases
- [ ] Tests cover the new behaviour
- [ ] No new compiler warnings
- [ ] Doc comments on all public functions
- [ ] Commit messages follow conventional commits
- [ ] CHANGELOG.md updated
- [ ] No unnecessary dependencies added

---

## License

By contributing you agree your work will be licensed under Apache 2.0.
