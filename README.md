# fixed-math

A fixed-point mathematical library implemented in Rust.

`fixed-math` is a research-oriented numerical library that implements elementary mathematical functions using **fixed-point arithmetic** and **continued fraction expansions**. The project aims to provide deterministic, configurable, and portable numerical computation without relying on floating-point hardware.

---

## Features

Current implementation includes:

- Fixed-point arithmetic (`Fixed`)
- Configurable binary Q formats
- Multiple rounding strategies
- Continued fraction evaluation framework
- Backward recurrence evaluator
- Modified Lentz evaluator
- Lambert continued fractions
- Atanh continued fractions
- Exponential function (`exp`)
- Natural logarithm (`ln`)

---

## Design Goals

The project is designed around four principles:

- Deterministic numerical computation
- Modular mathematical components
- Configurable evaluation algorithms
- Extensible architecture for future special functions

---

## Project Structure

```text
src/
├── number/      Fixed-point arithmetic
├── cf/          Continued fraction framework
├── function/    Mathematical functions
├── traits/      Public evaluation interfaces
└── error/       Error definitions
```

Additional documentation is located in the `docs/` directory.

---

## Current Status

Completed in Version 0.1:

- Fixed-point number system
- Four rounding modes
- Continued fraction framework
- Backward recurrence evaluator
- Modified Lentz evaluator
- Lambert coefficient provider
- Atanh coefficient provider
- Exponential function
- Natural logarithm

---

## Quality Assurance

The project is continuously validated using:

```bash
cargo fmt
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Current status:

- 98 unit tests
- All tests passing
- Zero Clippy warnings

---

## Roadmap

Planned mathematical functions include:

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

See `docs/roadmap.md` for long-term planning.

---

## License

This project is released under the MIT License.