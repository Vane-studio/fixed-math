# Software Architecture

## Overview

The architecture of `fixed-math` is organized into independent layers.

Each layer has a single responsibility and communicates only through stable interfaces, allowing mathematical functions to evolve independently from the underlying numerical algorithms.

---

# Architecture

```text
Application
      │
      ▼
Mathematical Functions
      │
      ▼
Evaluation Configuration
      │
      ▼
Continued Fraction Framework
      │
      ▼
Fixed-Point Arithmetic
```

The separation between mathematical models and numerical evaluation is the primary design principle of this project.

---

# Module Organization

```text
src/

number/
    Fixed
    Format
    Constant
    Rounding

cf/
    Model
    Coefficient
    Evaluator
    Lambert
    Atanh

function/
    Exp
    Ln
    Common

traits/
    Evaluate
    Function

error/
    Error
```

---

# Fixed-Point Layer

The lowest layer provides deterministic arithmetic.

Responsibilities include:

- binary Q formats
- integer encoding
- scaling
- multiplication
- division
- configurable rounding
- overflow detection

Higher layers never manipulate raw integers directly.

---

# Continued Fraction Layer

The continued fraction framework is completely independent from individual mathematical functions.

Its responsibilities include:

- continued fraction model
- coefficient generation
- evaluation algorithms
- numerical convergence

This design allows multiple mathematical functions to reuse the same evaluator.

---

# Evaluation Algorithms

Currently two evaluation algorithms are implemented.

## Backward Recurrence

Characteristics:

- simple implementation
- deterministic execution
- excellent numerical stability

Suitable as the default evaluation strategy.

---

## Modified Lentz Method

Characteristics:

- forward evaluation
- efficient for deep continued fractions
- robust near convergence difficulties

Both algorithms expose the same public interface.

---

# Function Layer

Each mathematical function only contains mathematical transformations.

For example:

Exp

- range reduction
- coefficient provider
- evaluator

Ln

- power decomposition
- atanh transform
- evaluator

The numerical algorithm is selected through configuration rather than hardcoded into each function.

---

# Configuration

All numerical behavior is described by `EvaluationConfig`.

Configuration options include:

- binary format
- rounding strategy
- continued fraction depth
- evaluation algorithm

Builder and setter APIs are provided for ergonomic usage.

---

# Trait Design

Two public traits are exposed.

## Evaluate

Evaluates a function using internally stored input.

```rust
fn evaluate(&self) -> Result<Fixed>;
```

---

## Function

Evaluates arbitrary user supplied input.

```rust
fn evaluate(&self, input: Fixed) -> Result<Fixed>;
```

This separation allows both object-oriented and functional programming styles.

---

# Error Handling

The project uses explicit error propagation.

No function silently ignores:

- arithmetic overflow
- invalid domains
- invalid continued fractions
- division by zero

Errors are propagated through `Result`.

---

# Extensibility

New mathematical functions only need to implement:

1. mathematical transformation
2. coefficient generator

The evaluation framework remains unchanged.

This minimizes duplicated code while maintaining numerical consistency across the library.