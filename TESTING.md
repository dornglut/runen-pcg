# Testing

## Canonical validation

```text
cargo validate
```

The local `xtask` checks required files, product identity and licensing, formatting, locked workspace tests, Clippy, rustdoc, Git whitespace, and worktree cleanliness.

The pinned GitHub workflow invokes this command on the exact pull-request head. A new head requires new validation and review.

## Current coverage

This is a repository bootstrap, not a generation implementation. No procedural-generation conformance, seed replay, or independent consumer tests exist yet. Each accepted capability must add its own behavioral and downstream evidence before release.
