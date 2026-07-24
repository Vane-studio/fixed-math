# fixed-math

![Rust](https://img.shields.io/badge/Rust-stable-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Tests](https://img.shields.io/badge/tests-passing-brightgreen)


> Deterministic fixed-point mathematics for Rust based on configurable continued fraction evaluation.

`fixed-math` is a numerical computation library for Rust that provides configurable fixed-point arithmetic together with a reusable continued fraction evaluation framework.

Unlike many numerical libraries that rely on floating-point polynomial approximations, **fixed-math** evaluates elementary functions using configurable continued fraction algorithms, making it suitable for deterministic numerical computation, embedded environments, and systems where floating-point arithmetic is undesirable.

---

## Features

### Fixed-point arithmetic

- Configurable binary Q formats
- Signed fixed-point numbers
- Integer encoding and decoding
- Fixed-point rescaling
- Checked multiplication and division
- Multiple rounding strategies

### Continued fraction framework

- Generic continued fraction abstraction
- Backward recurrence evaluator
- Modified Lentz evaluator
- Configurable evaluation depth
- Runtime-selectable evaluation algorithm

### Mathematical functions

Currently implemented:

| Function | Method |
|----------|--------|
| exp(x) | Lambert continued fraction |
| ln(x) | Atanh transformation |

Planned:

- atan
- tanh
- sqrt
- pow
- sin
- cos
- asin
- acos
- gamma
- erf

---

# Why Continued Fractions?

Most fixed-point numerical libraries implement transcendental functions using polynomial approximations.

This library instead builds around **continued fractions**, which generally provide:

- improved numerical stability
- better convergence over wide input ranges
- reusable evaluation infrastructure
- configurable evaluation algorithms
- deterministic fixed-point execution

Instead of embedding approximation coefficients directly into each function, `fixed-math` separates:

- coefficient generation
- evaluation algorithm
- fixed-point arithmetic

This allows the same evaluation engine to be reused across many mathematical functions.

---

# Quick Start

Clone the repository:

```bash
git clone https://github.com/Vane-studio/fixed-math.git
cd fixed-math
```

Run the examples:

```bash
cargo run --example basic

cargo run --example configuration
```

Run all tests:

```bash
cargo test
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

---

# Examples

The project provides runnable examples demonstrating the public API.

| Example | Description |
|---------|-------------|
| examples/basic.rs | Basic fixed-point function evaluation |
| examples/configuration.rs | Configuring evaluation algorithms |

The examples are intended to demonstrate the recommended usage of the library and are kept synchronized with the public API.

---

# Architecture

```
               Mathematical Functions
                        │
                        ▼
               EvaluationConfig
                        │
        ┌───────────────┴───────────────┐
        ▼                               ▼
 Backward Evaluation            Modified Lentz
        │                               │
        └───────────────┬───────────────┘
                        ▼
             Continued Fraction Engine
                        │
                        ▼
              Fixed-point Arithmetic
```

The implementation is intentionally layered.

```
src/

├── number/
│   Fixed-point numbers
│   Formats
│   Rounding
│
├── cf/
│   Continued fractions
│   Coefficients
│   Evaluators
│
├── function/
│   Mathematical functions
│
├── traits/
│   Public traits
│
└── error/
    Error types
```

---

# Evaluation Framework

Evaluation behavior is configured through `EvaluationConfig`.

It controls:

- binary format
- continued fraction depth
- rounding strategy
- evaluation algorithm

Supported algorithms:

- Backward recurrence
- Modified Lentz

Future algorithms can be added without changing the public function interfaces.

---

# Documentation

Additional documentation is available in the `docs/` directory.

| Document | Description |
|----------|-------------|
| unit1.md | Unit 1 implementation summary |
| design.md | Architecture overview |
| math.md | Mathematical background |
| api.md | Public API reference |
| roadmap.md | Development roadmap |

Project policies:

- CONTRIBUTING.md
- CHANGELOG.md
- SECURITY.md

---

# Project Status

Current implementation:

- Fixed-point arithmetic
- Continued fraction framework
- Lambert continued fractions
- Atanh continued fractions
- Exponential function
- Natural logarithm

Quality status:

- cargo fmt
- cargo test
- cargo clippy

All existing tests pass.

---

# Roadmap

Unit 2

- atan
- tanh
- sqrt
- pow

Unit 3

- sin
- cos
- asin
- acos

Future work

- special functions
- benchmark suite
- docs.rs documentation
- crates.io release
- no_std support

---

# Contributing

Contributions are welcome.

Before submitting changes, please ensure:

```bash
cargo fmt

cargo test

cargo clippy --all-targets --all-features -- -D warnings
```

See `CONTRIBUTING.md` for details.

---

# License

Licensed under the MIT License.

See `LICENSE`.