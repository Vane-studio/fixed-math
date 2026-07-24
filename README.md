# fixed-math

A deterministic fixed-point mathematics library for Rust, built around configurable binary formats and continued fraction evaluation.

`fixed-math` provides reusable numerical infrastructure for implementing elementary and special functions without depending on floating-point arithmetic during evaluation.

## Features

- Signed fixed-point arithmetic
- Configurable binary Q formats
- Integer encoding and decoding
- Fixed-point rescaling
- Checked multiplication and division
- Configurable rounding behavior
- Continued fraction abstractions
- Backward recurrence evaluation
- Modified Lentz evaluation
- Lambert continued fraction coefficients
- Atanh continued fraction coefficients
- Exponential function
- Natural logarithm
- Explicit error handling
- Configurable evaluation depth and algorithm

## Project Status

Version `0.1.0` establishes the first complete implementation stage.

Current validation status:

```text
98 unit tests passed
0 test failures
0 Clippy warnings
rustfmt clean
```

The following commands are used to validate the project:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## Architecture

```text
Mathematical Functions
          │
          ▼
  EvaluationConfig
          │
          ▼
Continued Fraction Model
          │
    ┌─────┴─────┐
    ▼           ▼
Backward      Lentz
    │           │
    └─────┬─────┘
          ▼
 Fixed-Point Arithmetic
```

The implementation is divided into independent layers:

```text
src/
├── number/      Fixed-point values, formats, constants, and rounding
├── cf/          Continued fraction models, coefficients, and evaluators
├── function/    Mathematical functions
├── traits/      Public evaluation traits
└── error/       Error definitions
```

## Implemented Functions

| Function | Status | Main method |
|---|---:|---|
| `exp(x)` | Implemented | Lambert continued fraction |
| `ln(x)` | Implemented | Power-of-two reduction and atanh transform |

Both functions support the following continued fraction evaluation algorithms:

- Backward recurrence
- Modified Lentz method

## Evaluation Configuration

Numerical behavior is controlled through `EvaluationConfig`.

It includes:

- fixed-point format
- rounding strategy
- continued fraction depth
- evaluation algorithm

The default evaluation algorithm is backward recurrence.

## Documentation

- [Unit 1 implementation summary](docs/unit1.md)
- [Software architecture](docs/design.md)
- [Mathematical background](docs/math.md)
- [API reference](docs/api.md)
- [Development roadmap](docs/roadmap.md)
- [Changelog](CHANGELOG.md)
- [Contributing guide](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

## Roadmap

Planned areas include:

- square root and power functions
- inverse tangent
- hyperbolic functions
- trigonometric functions
- inverse trigonometric functions
- error and gamma functions
- benchmarks
- expanded API documentation
- crates.io publication

The roadmap is provisional and may change as the numerical design evolves.

## Contributing

Contributions should pass all formatting, testing, and lint checks:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

Licensed under the MIT License. See [LICENSE](LICENSE).