# Mathematical Background

## Overview

The numerical algorithms implemented in `fixed-math` are based on continued fraction representations of elementary functions.

Compared with polynomial approximations, continued fractions often provide improved numerical stability and wider convergence regions when implemented using fixed-point arithmetic.

---

# Exponential Function

The exponential function is evaluated through range reduction.

The input is decomposed as

```text
x = k ln(2) + r
```

where

```text
|r| ≤ ln(2)/2
```

The final result becomes

```text
exp(x) = exp(r) · 2^k
```

This greatly improves convergence.

---

# Lambert Continued Fraction

The reduced exponential is evaluated using Lambert's continued fraction.

Advantages include:

- excellent convergence near zero
- compact representation
- efficient fixed-point implementation

The coefficient provider generates the continued fraction terms without exposing implementation details to the evaluator.

---

# Natural Logarithm

The logarithm first decomposes the input.

```text
x = m · 2^k
```

where

```text
1 ≤ m < 2
```

The logarithm becomes

```text
ln(x) = ln(m) + k ln(2)
```

---

# Atanh Transformation

To improve convergence,

```text
z = (m − 1)/(m + 1)
```

is introduced.

Then

```text
ln(m) = 2 atanh(z)
```

The resulting continued fraction converges significantly faster than direct logarithm expansions.

---

# Continued Fractions

A continued fraction has the form

```text
b0 +
a1
────────────
b1 +
a2
────────────
b2 +
...
```

Unlike polynomial approximations, each coefficient contributes through recursive evaluation.

This structure is particularly suitable for fixed-point arithmetic because intermediate values remain numerically stable.

---

# Evaluation Algorithms

Two numerical algorithms are currently implemented.

## Backward Recurrence

Evaluation starts from the deepest level and proceeds upward.

Advantages:

- deterministic
- simple
- highly stable

---

## Modified Lentz Method

Evaluation proceeds from the beginning of the continued fraction.

Advantages:

- efficient for deep fractions
- avoids explicit backward recursion
- numerically robust

---

# Fixed-Point Arithmetic

All computations are performed using signed fixed-point integers.

Advantages include:

- deterministic results
- reproducibility
- platform independence
- embedded suitability

No floating-point operations are required during evaluation.

---

# Numerical Accuracy

Accuracy depends on:

- binary format
- continued fraction depth
- rounding strategy
- evaluation algorithm

These parameters are configurable through `EvaluationConfig`.

---

# References

The implementation is influenced by classical literature on continued fractions and numerical computation, including:

- Hubert Stanley Wall, *Analytic Theory of Continued Fractions*
- William J. Cody
- Numerical Recipes
- Cuyt et al., *Handbook of Continued Fractions for Special Functions*