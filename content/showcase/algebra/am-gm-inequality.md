+++
title = "AM–GM Inequality"
description = "The arithmetic mean is always at least as large as the geometric mean"
weight = 20
tags = ["lean4-proof", "algebra", "inequality", "visualization"]
latex = "\\frac{a_1+\\cdots+a_n}{n} \\geq \\sqrt[n]{a_1 \\cdots a_n}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For non-negative real numbers $a_1, a_2, \ldots, a_n$, the **arithmetic mean** is always at least as
large as the **geometric mean**:

$$\frac{a_1 + a_2 + \cdots + a_n}{n} \geq \sqrt[n]{a_1 \cdot a_2 \cdots a_n}$$

with equality if and only if $a_1 = a_2 = \cdots = a_n$.

The simplest case, for two non-negative reals $a$ and $b$:

$$\frac{a + b}{2} \geq \sqrt{ab}$$

## Visualization

Comparing AM and GM for several tuples of non-negative reals:

| Tuple $(a, b)$ | AM $= \frac{a+b}{2}$ | GM $= \sqrt{ab}$ | AM $-$ GM |
|---|---|---|---|
| $(1, 1)$ | $1.000$ | $1.000$ | $0.000$ |
| $(2, 8)$ | $5.000$ | $4.000$ | $1.000$ |
| $(1, 9)$ | $5.000$ | $3.000$ | $2.000$ |
| $(4, 9)$ | $6.500$ | $6.000$ | $0.500$ |
| $(0, 4)$ | $2.000$ | $0.000$ | $2.000$ |
| $(3, 3)$ | $3.000$ | $3.000$ | $0.000$ |

The gap $\text{AM} - \text{GM} = \frac{(\sqrt{a} - \sqrt{b})^2}{2} \geq 0$ grows when $a$ and $b$ differ most.

**Geometric insight** (unit square area argument for two terms):

```
  b |----+----------+
    |    |//////////|    Area of rectangle = ab
    |    |//////////|    Area of square on AM-side ≥ area of rectangle
    |    |//////////|    ↔  ((a+b)/2)² ≥ ab
    +----+----------+--
    0              a+b
                    2
```

## Proof Sketch

The two-term case follows directly from the fact that squares are non-negative:

$$0 \leq (a - b)^2 = a^2 - 2ab + b^2$$

Adding $4ab$ to both sides:

$$4ab \leq a^2 + 2ab + b^2 = (a + b)^2$$

Taking square roots (all quantities non-negative):

$$2\sqrt{ab} \leq a + b \implies \sqrt{ab} \leq \frac{a+b}{2}$$

The general $n$-term case follows by induction combined with the two-term case, or via the
**weighted AM–GM** inequality with equal weights, which itself follows from the concavity of $\log$.

## Connections

AM–GM is tightly connected to several other fundamental results:

- [[Quadratic Formula]] — the discriminant $b^2 - 4ac \geq 0$ when roots are real is exactly AM–GM
- [[Cauchy–Schwarz]] — AM–GM is used in its proof; both are instances of Jensen's inequality
- [[Geometric Series]] — bounding partial products uses AM–GM-style reasoning
- [[Binomial Theorem]] — the middle term in $(a+b)^2 = a^2 + 2ab + b^2$ embodies the AM–GM core step
- [[Vieta Formulas]] — product of roots vs. sum of roots echoes the GM vs. AM structure

## Lean4 Proof

The two-term inequality $2ab \leq a^2 + b^2$ is `two_mul_le_add_sq` in Mathlib. We derive
$\sqrt{ab} \leq (a+b)/2$ from it, and then delegate the weighted general case to
`Real.geom_mean_le_arith_mean2_weighted`.

```lean4
import Mathlib.Analysis.MeanInequalities
import Mathlib.Analysis.SpecialFunctions.Pow.Real

/-- Two-term AM–GM: 2ab ≤ a² + b² (equivalent form).
    Note: `two_mul_le_add_sq` proves `2 * a * b ≤ ...`; we use `linarith` to
    convert to `2 * (a * b)`. -/
theorem am_gm_sq (a b : ℝ) : 2 * (a * b) ≤ a ^ 2 + b ^ 2 := by
  have h := two_mul_le_add_sq a b
  linarith

/-- Two-term AM–GM for non-negative reals: √(ab) ≤ (a+b)/2. -/
theorem am_gm_two (a b : ℝ) (ha : 0 ≤ a) (hb : 0 ≤ b) :
    Real.sqrt (a * b) ≤ (a + b) / 2 := by
  rw [← Real.sqrt_sq (by linarith : 0 ≤ (a + b) / 2)]
  apply Real.sqrt_le_sqrt
  nlinarith [sq_nonneg (a - b), sq_nonneg (a + b)]

/-- Weighted two-term AM–GM: for weights w₁, w₂ ≥ 0 with w₁ + w₂ = 1, and p₁, p₂ ≥ 0,
    we have p₁^w₁ · p₂^w₂ ≤ w₁·p₁ + w₂·p₂. -/
theorem am_gm_weighted (w₁ w₂ p₁ p₂ : ℝ)
    (hw₁ : 0 ≤ w₁) (hw₂ : 0 ≤ w₂) (hp₁ : 0 ≤ p₁) (hp₂ : 0 ≤ p₂) (hw : w₁ + w₂ = 1) :
    p₁ ^ w₁ * p₂ ^ w₂ ≤ w₁ * p₁ + w₂ * p₂ :=
  Real.geom_mean_le_arith_mean2_weighted hw₁ hw₂ hp₁ hp₂ hw
```
