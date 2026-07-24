# Contributing

Thank you for your interest in contributing to **fixed-math**.

## Development Environment

Required tools:

- Rust (stable)
- Cargo
- rustfmt
- Clippy

---

## Build

```bash
cargo build
```

---

## Format

```bash
cargo fmt
```

All source code must be formatted before submission.

---

## Tests

```bash
cargo test
```

All tests must pass.

---

## Lints

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

No Clippy warnings are accepted.

---

## Pull Requests

Pull requests should:

- compile successfully
- pass all tests
- introduce no formatting changes
- introduce no Clippy warnings
- include documentation when appropriate

---

## Commit Messages

Recommended style:

```
feat: add atan implementation

fix: correct overflow detection

docs: improve mathematical documentation

refactor: simplify evaluator construction

test: increase coverage for ln
```

---

## Coding Style

The project emphasizes:

- readability
- deterministic behavior
- explicit error handling
- minimal abstraction
- reusable numerical components

---

## Issues

Bug reports should include:

- platform
- Rust version
- reproduction steps
- expected behavior
- actual behavior