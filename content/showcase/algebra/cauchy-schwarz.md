+++
title = "Cauchy–Schwarz Inequality"
description = "The inner product of two vectors is bounded by the product of their norms"
weight = 30
tags = ["lean4-proof", "algebra", "inequality", "linear-algebra", "visualization"]
latex = "|\\langle u, v \\rangle| \\leq \\|u\\| \\, \\|v\\|"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For any two vectors $u$ and $v$ in an inner product space:

$$|\langle u, v \rangle| \leq \|u\| \cdot \|v\|$$

Equality holds if and only if $u$ and $v$ are linearly dependent (one is a scalar multiple of the other).

In $\mathbb{R}^n$ with the dot product, this reads:

$$\left| \sum_{i=1}^n u_i v_i \right| \leq \sqrt{\sum_{i=1}^n u_i^2} \cdot \sqrt{\sum_{i=1}^n v_i^2}$$

The geometric consequence: $|\cos\theta| \leq 1$ for the angle $\theta$ between two vectors.

## Visualization

Worked example in $\mathbb{R}^3$ with $u = (1, 2, 2)$ and $v = (2, 1, -2)$:

```
u = (1, 2, 2)        v = (2, 1, -2)

  dot product:          norms:
  u·v = 1·2            ‖u‖ = √(1²+2²+2²) = √9 = 3
      + 2·1            ‖v‖ = √(2²+1²+2²) = √9 = 3
      + 2·(-2)
      = 2+2-4 = 0      ‖u‖·‖v‖ = 3·3 = 9

  |u·v| = 0  ≤  9 = ‖u‖·‖v‖   ✓   (vectors are orthogonal)
```

Another example with $u = (1, 1, 0)$ and $v = (1, 0, 1)$:

| Quantity | Value |
|---|---|
| $u \cdot v = 1{\cdot}1 + 1{\cdot}0 + 0{\cdot}1$ | $1$ |
| $\|u\| = \sqrt{1^2+1^2+0^2}$ | $\sqrt{2} \approx 1.414$ |
| $\|v\| = \sqrt{1^2+0^2+1^2}$ | $\sqrt{2} \approx 1.414$ |
| $\|u\| \cdot \|v\|$ | $2$ |
| $|u \cdot v| \leq \|u\|\|v\|$? | $1 \leq 2$ ✓ |
| $\cos\theta = (u\cdot v)/(\|u\|\|v\|)$ | $0.5$, so $\theta = 60°$ |

When $u = (a, a)$ and $v = (b, b)$ (parallel), equality holds: $|ab + ab| = \sqrt{2a^2}\cdot\sqrt{2b^2} = 2|ab|$.

## Proof Sketch

Fix $v \neq 0$ and consider the real-valued function $f(t) = \|u - tv\|^2 \geq 0$:

$$\|u\|^2 - 2t\langle u,v\rangle + t^2\|v\|^2 \geq 0 \quad \text{for all } t \in \mathbb{R}$$

This quadratic in $t$ is always non-negative, so its discriminant must be $\leq 0$:

$$\Delta = 4|\langle u,v\rangle|^2 - 4\|u\|^2\|v\|^2 \leq 0$$

Therefore $|\langle u, v\rangle| \leq \|u\|\|v\|$.

## Connections

Cauchy–Schwarz is a cornerstone of analysis and algebra:

- [[AM–GM Inequality]] — a one-dimensional shadow; both follow from convexity / Jensen's inequality
- [[Quadratic Formula]] — the discriminant condition $b^2 \leq 4ac$ in disguise
- [[Binomial Theorem]] — the "cross term" bound in $\|u+v\|^2$ is Cauchy–Schwarz
- [[Geometric Series]] — Bessel's inequality for orthonormal expansions uses Cauchy–Schwarz
- [[Vieta Formulas]] — in the Gram matrix approach to polynomial roots, Cauchy–Schwarz controls error

## Lean4 Proof

Mathlib provides `norm_inner_le_norm` in `Mathlib.Analysis.InnerProductSpace.Basic`, which is exactly
the Cauchy–Schwarz inequality for inner product spaces.

```lean4
import Mathlib.Analysis.InnerProductSpace.Basic

/-- Cauchy–Schwarz inequality: the norm of the inner product is bounded by
    the product of the norms. This is `norm_inner_le_norm` in Mathlib.  -/
theorem cauchy_schwarz
    {E : Type*} [SeminormedAddCommGroup E] [InnerProductSpace ℝ E]
    (u v : E) : ‖(inner u v : ℝ)‖ ≤ ‖u‖ * ‖v‖ :=
  norm_inner_le_norm u v

/-- Consequence: the cosine of the angle between two vectors has absolute value ≤ 1. -/
theorem cos_angle_le_one
    {E : Type*} [SeminormedAddCommGroup E] [InnerProductSpace ℝ E]
    (u v : E) (hu : ‖u‖ ≠ 0) (hv : ‖v‖ ≠ 0) :
    ‖(inner u v : ℝ)‖ / (‖u‖ * ‖v‖) ≤ 1 := by
  apply div_le_one_of_le
  · exact norm_inner_le_norm u v
  · positivity

/-- The real dot-product version: (Σ uᵢvᵢ)² ≤ (Σ uᵢ²)(Σ vᵢ²). -/
theorem cauchy_schwarz_sum (n : ℕ) (u v : Fin n → ℝ) :
    (∑ i, u i * v i) ^ 2 ≤ (∑ i, u i ^ 2) * (∑ i, v i ^ 2) := by
  have h := norm_inner_le_norm (𝕜 := ℝ) (E := EuclideanSpace ℝ (Fin n)) u v
  simp only [EuclideanSpace.inner_eq_star_mulVec, Matrix.dotProduct, EuclideanSpace.norm_sq] at h
  nlinarith [sq_nonneg (∑ i, u i * v i), h]
```
