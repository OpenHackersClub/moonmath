+++
title = "Vieta's Formulas"
description = "Relating the coefficients of a polynomial to the sums and products of its roots"
weight = 60
tags = ["lean4-proof", "algebra", "polynomials", "roots", "visualization"]
latex = "x_1 + x_2 + \\cdots + x_n = -\\frac{a_{n-1}}{a_n}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For a monic polynomial of degree $n$ with roots $x_1, x_2, \ldots, x_n$ (counted with multiplicity):

$$p(x) = x^n + a_{n-1}x^{n-1} + \cdots + a_1 x + a_0 = (x - x_1)(x - x_2)\cdots(x - x_n)$$

**Vieta's formulas** relate the coefficients to the **elementary symmetric polynomials** of the roots:

$$x_1 + x_2 + \cdots + x_n = -a_{n-1}$$

$$\sum_{i < j} x_i x_j = a_{n-2}$$

$$\vdots$$

$$x_1 x_2 \cdots x_n = (-1)^n a_0$$

For a **quadratic** $ax^2 + bx + c = a(x - x_1)(x - x_2)$:

$$x_1 + x_2 = -\frac{b}{a}, \qquad x_1 x_2 = \frac{c}{a}$$

## Visualization

**Worked cubic**: $p(x) = x^3 - 6x^2 + 11x - 6 = (x-1)(x-2)(x-3)$

Roots: $x_1 = 1$, $x_2 = 2$, $x_3 = 3$

```
Vieta checks:
  Sum of roots:           x₁ + x₂ + x₃ = 1 + 2 + 3 = 6   =  -(-6) = 6  ✓
  Sum of pairwise prods:  x₁x₂ + x₁x₃ + x₂x₃
                          = 2 + 3 + 6 = 11                 = 11           ✓
  Product of roots:       x₁ · x₂ · x₃ = 1·2·3 = 6        = -(-6) = 6   ✓
```

Coefficients: $a_3=1$, $a_2=-6$, $a_1=11$, $a_0=-6$

| Vieta relation | LHS | RHS | Match |
|---|---|---|---|
| $e_1 = x_1+x_2+x_3$ | $6$ | $-a_2/a_3 = 6$ | ✓ |
| $e_2 = \sum_{i<j} x_ix_j$ | $11$ | $a_1/a_3 = 11$ | ✓ |
| $e_3 = x_1x_2x_3$ | $6$ | $-a_0/a_3 = 6$ | ✓ |

**Newton's identity check** (sum of squares from Vieta): $x_1^2 + x_2^2 + x_3^2 = e_1^2 - 2e_2 = 36 - 22 = 14 = 1+4+9$ ✓

## Proof Sketch

Expand $(x - x_1)(x - x_2)\cdots(x - x_n)$ and collect terms by degree. When distributing $n$
factors, choosing $x$ from $k$ of them and $(-x_i)$ from the rest produces:

- Coefficient of $x^{n-1}$: $-(x_1 + x_2 + \cdots + x_n)$
- Coefficient of $x^{n-2}$: $\sum_{i < j} x_i x_j$
- Constant term: $(-1)^n x_1 x_2 \cdots x_n$

Matching with $a_{n-k}$ yields the formulas. The algebraic identity is the generating-function
statement that $\prod_{i=1}^n (x - x_i) = \sum_{k=0}^n (-1)^k e_k(x_1,\ldots,x_n) x^{n-k}$
where $e_k$ is the $k$-th **elementary symmetric polynomial**.

## Connections

- [[Quadratic Formula]] — Vieta for degree 2; $b^2 - 4ac = (x_1-x_2)^2 a^2$ links discriminant to root differences
- [[Binomial Theorem]] — $(x-r)^n$ is a degenerate Vieta case where all roots coincide
- [[AM–GM Inequality]] — AM–GM on the roots $x_i$ gives inequalities between the $e_k$
- [[Cauchy–Schwarz]] — in the Gram matrix of roots, Cauchy–Schwarz bounds cross-terms of the $e_k$
- [[Geometric Series]] — roots of $x^n - 1 = 0$ (roots of unity) satisfy $e_1 = 0$, $e_n = (-1)^{n+1}$

## Lean4 Proof

Mathlib's `Polynomial.SmallDegreeVieta` (v4.28.0) provides Vieta's formulas for quadratics
explicitly. We use `eq_neg_mul_add_of_roots_quadratic_eq_pair` (sum of roots) and
`eq_mul_mul_of_roots_quadratic_eq_pair` (product of roots).

```lean4
import Mathlib.RingTheory.Polynomial.SmallDegreeVieta

open Polynomial

/-- Vieta's sum formula for a quadratic ax²+bx+c with roots x₁, x₂:
    b = -a·(x₁+x₂), i.e., x₁+x₂ = -b/a. -/
theorem vieta_sum [CommRing R] [IsDomain R] {a b c x₁ x₂ : R}
    (hroots : (C a * X ^ 2 + C b * X + C c).roots = {x₁, x₂}) :
    b = -a * (x₁ + x₂) :=
  eq_neg_mul_add_of_roots_quadratic_eq_pair hroots

/-- Vieta's product formula for a quadratic ax²+bx+c with roots x₁, x₂:
    c = a·x₁·x₂, i.e., x₁·x₂ = c/a. -/
theorem vieta_product [CommRing R] [IsDomain R] {a b c x₁ x₂ : R}
    (hroots : (C a * X ^ 2 + C b * X + C c).roots = {x₁, x₂}) :
    c = a * x₁ * x₂ :=
  eq_mul_mul_of_roots_quadratic_eq_pair hroots

/-- Combined Vieta iff: roots are {x₁,x₂} iff Vieta relations hold (and a ≠ 0). -/
theorem vieta_quadratic_iff [CommRing R] [IsDomain R] {a b c x₁ x₂ : R} (ha : a ≠ 0) :
    (C a * X ^ 2 + C b * X + C c).roots = {x₁, x₂} ↔
      b = -a * (x₁ + x₂) ∧ c = a * x₁ * x₂ :=
  roots_quadratic_eq_pair_iff_of_ne_zero ha

/-- Concrete instance: x²-5x+6 has roots {2,3} and Vieta checks hold. -/
theorem vieta_example :
    (X ^ 2 - C 5 * X + C 6 : ℤ[X]).roots = {2, 3} ↔
      (-5 : ℤ) = -(1 * (2 + 3)) ∧ (6 : ℤ) = 1 * 2 * 3 := by
  constructor
  · intro h; exact ⟨by norm_num, by norm_num⟩
  · intro ⟨_, _⟩
    simp [roots_quadratic_eq_pair_iff_of_ne_zero (by norm_num : (1:ℤ) ≠ 0)]
    constructor <;> norm_num
```
