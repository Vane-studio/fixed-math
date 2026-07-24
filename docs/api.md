# API Reference

## Overview

The public API of `fixed-math` is intentionally compact.

The library is organized around four primary components:

- Fixed-point arithmetic
- Evaluation configuration
- Mathematical functions
- Evaluation traits

---

# Fixed

Represents a signed fixed-point value.

## Responsibilities

- store fixed-point numbers
- arithmetic operations
- format conversion
- integer encoding
- integer decoding

## Common Methods

```rust
Fixed::encode_integer(...)
```

Encodes an integer into a fixed-point representation.

```rust
Fixed::decode_integer(...)
```

Decodes a fixed-point value into an integer.

```rust
rescale(...)
```

Converts between binary formats.

---

# Format

Represents a binary Q format.

Examples include:

- Integer
- Q8
- Q16
- Q24
- Q32

Responsibilities:

- scaling
- integer conversion
- overflow detection

---

# EvaluationConfig

Describes numerical evaluation behavior.

Configuration options include:

- binary format
- rounding strategy
- continued fraction depth
- evaluation algorithm

Builder methods allow immutable configuration.

Setter methods allow mutable updates.

---

# EvaluationAlgorithm

Supported algorithms:

## Backward

Evaluates continued fractions using backward recurrence.

Default algorithm.

---

## Lentz

Evaluates continued fractions using the modified Lentz algorithm.

Useful for deep continued fractions.

---

# Exp

Computes the exponential function.

Construction:

```rust
Exp::new(input)
```

Evaluation:

```rust
evaluate()
```

or

```rust
eval(input)
```

---

# Ln

Computes the natural logarithm.

Construction:

```rust
Ln::new(input)
```

Evaluation:

```rust
evaluate()
```

or

```rust
eval(input)
```

---

# Traits

## Evaluate

Uses internally stored input.

```rust
fn evaluate(&self)
```

---

## Function

Uses caller supplied input.

```rust
fn evaluate(&self, input)
```

or

```rust
fn eval(&self, input)
```

---

# Error Handling

All public APIs return

```rust
Result<T, Error>
```

Possible errors include:

- arithmetic overflow
- invalid domain
- division by zero
- invalid continued fraction

---

# Stability

The current API corresponds to Version 0.1.

Future releases may extend the API while preserving backward compatibility whenever practical.