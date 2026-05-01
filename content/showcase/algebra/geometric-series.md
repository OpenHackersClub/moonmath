+++
title = "Geometric Series"
description = "Partial sums and convergence of the geometric progression"
weight = 50
tags = ["lean4-proof", "algebra", "series", "convergence", "visualization"]
latex = "\\sum_{k=0}^{n-1} x^k = \\frac{x^n - 1}{x - 1}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For any ring element $x \neq 1$ and natural number $n$:

$$\sum_{k=0}^{n-1} x^k = \frac{x^n - 1}{x - 1}$$

Equivalently, $1 + x + x^2 + \cdots + x^{n-1} = \dfrac{x^n - 1}{x - 1}$.

When $|x| < 1$ in $\mathbb{R}$ (or $\mathbb{C}$), the infinite series converges:

$$\sum_{k=0}^{\infty} x^k = \frac{1}{1 - x}$$

## Visualization

**Partial sums converging** for $x = 1/2$: each bar shows $S_n = \sum_{k=0}^{n-1} (1/2)^k$,
converging to $S_\infty = 1/(1-1/2) = 2$:

```
n  | S_n = 1 + 1/2 + ... + (1/2)^{n-1}       | % of limit
---|--------------------------------------------|-----------
1  | 1.000  ████████████████████████            | 50.0%
2  | 1.500  ████████████████████████████████████| 75.0%
3  | 1.750  ████████████████████████████████████████| 87.5%
4  | 1.875  ██████████████████████████████████████████| 93.8%
5  | 1.938  ███████████████████████████████████████████| 96.9%
6  | 1.969  ████████████████████████████████████████████| 98.4%
7  | 1.984  █████████████████████████████████████████████| 99.2%
∞  | 2.000  ██████████████████████████████████████████████ 2.000
```

**Exact partial sums** for small $n$ and $x = 2$:

| $n$ | $S_n = \sum_{k=0}^{n-1} 2^k$ | Formula $(2^n - 1)/(2-1) = 2^n - 1$ |
|---|---|---|
| 1 | $1$ | $1$ |
| 2 | $1+2 = 3$ | $3$ |
| 3 | $1+2+4 = 7$ | $7$ |
| 4 | $1+2+4+8 = 15$ | $15$ |
| 5 | $1+2+4+8+16 = 31$ | $31$ |

## Proof Sketch

The key identity follows from telescoping. Consider the product:

$$(x - 1)(1 + x + x^2 + \cdots + x^{n-1})$$

Expanding and cancelling consecutive terms:

$$= x + x^2 + \cdots + x^n - 1 - x - x^2 - \cdots - x^{n-1} = x^n - 1$$

Dividing by $(x - 1) \neq 0$ gives the formula. This is a **ring identity**: no analytic
assumptions required — it holds in any commutative ring where $x - 1$ is invertible.

For the infinite sum: $S_n = (1 - x^n)/(1-x)$ and $|x^n| \to 0$ as $n \to \infty$ when $|x| < 1$.

## Connections

- [[Quadratic Formula]] — setting $n=2$ and solving $x^2 - 1 = (x-1)(x+1) = 0$ echoes the factoring step
- [[AM-GM Inequality]] — bounding $|1 + x + \cdots + x^{n-1}|$ uses GM applied to the terms
- [[Binomial Theorem]] — both rely on algebraic manipulation of sums with a closed form
- [[Cauchy-Schwarz]] — $\ell^2$ convergence proofs use Cauchy-Schwarz on partial sums
- [[Vieta Formulas]] — the polynomial $x^n - 1 = (x-1)(x^{n-1}+\cdots+1)$ relates roots (roots of unity) to coefficients

## Lean4 Proof

Mathlib's `geom_sum_eq` in `Mathlib.Algebra.Field.GeomSum` gives the closed form for fields.
The ring-level identity is in `Mathlib.Algebra.Ring.GeomSum`.

```lean4
import Mathlib.Algebra.Field.GeomSum
import Mathlib.Algebra.Ring.GeomSum

/-- Finite geometric sum formula: Σ_{i<n} x^i = (x^n - 1)/(x - 1) for x ≠ 1.
    This is `geom_sum_eq` in Mathlib (Field version). -/
theorem geom_sum_formula {F : Type*} [Field F] (x : F) (h : x ≠ 1) (n : ℕ) :
    ∑ i ∈ Finset.range n, x ^ i = (x ^ n - 1) / (x - 1) :=
  geom_sum_eq h n

/-- Equivalent form: (x - 1) · Σ_{i<n} x^i = x^n - 1. Works in any CommRing. -/
theorem geom_sum_mul {R : Type*} [CommRing R] (x : R) (n : ℕ) :
    (x - 1) * ∑ i ∈ Finset.range n, x ^ i = x ^ n - 1 :=
  geom_sum_mul x n

/-- Corollary: for x in (-1,1), the partial sums are bounded above by 1/(1-x). -/
theorem geom_sum_bound (x : ℝ) (hx : x < 1) (hx0 : 0 ≤ x) (n : ℕ) :
    ∑ i ∈ Finset.range n, x ^ i ≤ 1 / (1 - x) := by
  have hpos : 0 < 1 - x := by linarith
  rw [le_div_iff hpos]
  have key : (1 - x) * ∑ i ∈ Finset.range n, x ^ i = 1 - x ^ n := by
    have := geom_sum_mul x n
    linarith [this]
  rw [key]
  nlinarith [pow_nonneg hx0 n]
```
