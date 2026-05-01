+++
title = "Binomial Theorem"
description = "Expanding powers of a sum via binomial coefficients"
weight = 40
tags = ["lean4-proof", "algebra", "combinatorics", "polynomials", "visualization"]
latex = "(x+y)^n = \\sum_{k=0}^{n} \\binom{n}{k} x^k y^{n-k}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For any commutative ring elements $x$ and $y$, and any natural number $n$:

$$(x + y)^n = \sum_{k=0}^{n} \binom{n}{k} x^k y^{n-k}$$

where $\binom{n}{k} = \dfrac{n!}{k!(n-k)!}$ is the **binomial coefficient**, counting the number of
ways to choose $k$ items from $n$ without repetition.

## Visualization

**Pascal's Triangle** (rows 0–6) organises the binomial coefficients. Each entry is the sum of the
two entries above it — the **Pascal recurrence** $\binom{n}{k} = \binom{n-1}{k-1} + \binom{n-1}{k}$:

```
Row 0:                    1
Row 1:                  1   1
Row 2:                1   2   1
Row 3:              1   3   3   1
Row 4:            1   4   6   4   1
Row 5:          1   5  10  10   5   1
Row 6:        1   6  15  20  15   6   1
```

Each row gives the coefficients for $(x+y)^n$. For example, row 4 yields:

$$(x+y)^4 = x^4 + 4x^3y + 6x^2y^2 + 4xy^3 + y^4$$

**Row sum identity**: every row sums to $2^n$ (set $x = y = 1$):

| $n$ | Row sum | $2^n$ |
|---|---|---|
| 0 | $1$ | $1$ |
| 1 | $1+1$ | $2$ |
| 2 | $1+2+1$ | $4$ |
| 3 | $1+3+3+1$ | $8$ |
| 4 | $1+4+6+4+1$ | $16$ |
| 5 | $1+5+10+10+5+1$ | $32$ |

## Proof Sketch

**Combinatorial argument**: expanding $(x+y)^n = (x+y)(x+y)\cdots(x+y)$ ($n$ factors), we pick
either $x$ or $y$ from each factor. The term $x^k y^{n-k}$ arises from exactly $\binom{n}{k}$
selections (choose which $k$ of the $n$ factors contribute an $x$). Summing over $k = 0,\ldots,n$
accounts for all terms.

**Inductive step**: the Pascal recurrence drives the induction:
$$\binom{n+1}{k} = \binom{n}{k-1} + \binom{n}{k}$$
which corresponds to distributing the extra factor of $(x+y)$:
$$(x+y)^{n+1} = x \cdot (x+y)^n + y \cdot (x+y)^n$$

## Connections

- [[Quadratic Formula]] — $(x+y)^2 = x^2 + 2xy + y^2$ is the $n=2$ special case
- [[AM–GM Inequality]] — AM–GM applied to the $\binom{n}{k}$ terms yields useful norm bounds
- [[Cauchy–Schwarz]] — the square of a sum bound uses the $n=2$ binomial expansion
- [[Geometric Series]] — setting $y = 1$ and letting $x \to 1$ connects to sums of powers
- [[Vieta Formulas]] — the coefficients of $(x - r_1)(x - r_2)\cdots(x - r_n)$ are elementary symmetric polynomials, relatives of binomial coefficients

## Lean4 Proof

Mathlib's `add_pow` in `Mathlib.Data.Nat.Choose.Sum` provides the binomial theorem directly for
`CommSemiring`. The non-commutative version is `Commute.add_pow`.

```lean4
import Mathlib.Data.Nat.Choose.Sum

/-- The Binomial Theorem: (x+y)^n = Σ_{k=0}^{n} x^k · y^{n-k} · C(n,k)
    for commutative semirings. This is `add_pow` in Mathlib. -/
theorem binomial_theorem {R : Type*} [CommSemiring R] (x y : R) (n : ℕ) :
    (x + y) ^ n = ∑ k ∈ Finset.range (n + 1), x ^ k * y ^ (n - k) * (n.choose k : R) :=
  add_pow x y n

/-- The row-sum identity: setting x=y=1 gives 2^n = Σ C(n,k). -/
theorem binomial_row_sum (n : ℕ) :
    2 ^ n = ∑ k ∈ Finset.range (n + 1), n.choose k := by
  have h := binomial_theorem (R := ℕ) 1 1 n
  simp at h
  exact_mod_cast h

/-- Non-commutative version: Commute.add_pow handles semirings where x·y ≠ y·x. -/
theorem binomial_noncomm {R : Type*} [Semiring R] (x y : R) (h : Commute x y) (n : ℕ) :
    (x + y) ^ n = ∑ k ∈ Finset.range (n + 1), x ^ k * y ^ (n - k) * (n.choose k : R) :=
  h.add_pow n
```
