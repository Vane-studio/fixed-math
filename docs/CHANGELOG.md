# Changelog

All notable changes to this project are documented in this file.

The format is inspired by *Keep a Changelog*.

---

# [0.1.0] - Initial Release

## Added

### Fixed-Point Arithmetic

- configurable binary Q formats
- integer encoding and decoding
- rescaling
- multiplication and division
- overflow detection
- four rounding modes

### Continued Fraction Framework

- continued fraction model
- coefficient abstraction
- configurable evaluator

### Evaluation Algorithms

- backward recurrence
- modified Lentz algorithm

### Coefficient Providers

- Lambert continued fraction
- atanh continued fraction

### Mathematical Functions

- exponential
- natural logarithm

### Configuration

- EvaluationConfig
- configurable evaluation depth
- configurable evaluation algorithm
- configurable rounding strategy
- configurable binary format

### Testing

- arithmetic tests
- configuration tests
- algorithm consistency tests
- function tests
- trait tests

Current validation status:

- cargo fmt
- cargo test
- cargo clippy

All checks pass successfully.