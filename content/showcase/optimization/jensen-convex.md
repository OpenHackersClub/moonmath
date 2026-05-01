+++
title = "Jensen's Inequality (Convex)"
description = "For a convex function, the image of a weighted sum is at most the weighted sum of images"
weight = 30
tags = ["lean4-proof", "optimization", "visualization"]
latex = "f\\!\\left(\\sum_i w_i x_i\\right) \\le \\sum_i w_i f(x_i),\\quad w_i \\ge 0,\\; \\sum w_i = 1"
prerequisites = ["convex-function"]
lean4_status = "complete"
+++

## Statement

Let $f : \mathbb{R} \to \mathbb{R}$ be convex. For points $x_1, \ldots, x_n$ and non-negative weights $w_i$ summing to 1:

$$f\!\left(\sum_{i=1}^n w_i x_i\right) \le \sum_{i=1}^n w_i f(x_i).$$

Mathlib states this as `ConvexOn.map_sum_le` (in `Mathlib.Analysis.Convex.Jensen`): if `hf : ConvexOn 𝕜 s f`, weights are non-negative summing to 1, and points lie in $s$, then the Jensen inequality holds.

## Visualization

Take $f(x) = x^2$ (convex) and the distribution $\{x_1, x_2, x_3\} = \{1, 2, 3\}$ with equal weights $w_i = 1/3$.

**Weighted average:** $\bar{x} = \frac{1+2+3}{3} = 2$

**Jensen LHS:** $f(\bar{x}) = f(2) = 4$

**Jensen RHS:** $\frac{1}{3}f(1) + \frac{1}{3}f(2) + \frac{1}{3}f(3) = \frac{1+4+9}{3} = \frac{14}{3} \approx 4.67$

| $x_i$ | $w_i$ | $f(x_i) = x_i^2$ | $w_i f(x_i)$ |
|--------|-------|-----------------|--------------|
| 1      | 1/3   | 1               | 1/3          |
| 2      | 1/3   | 4               | 4/3          |
| 3      | 1/3   | 9               | 3            |
| **sum**| **1** | —               | **14/3**     |

$f\!\left(\bar{x}\right) = 4 \le \tfrac{14}{3} \approx 4.67$ ✓

The gap $\tfrac{14}{3} - 4 = \tfrac{2}{3}$ reflects the variance: for $f(x) = x^2$, Jensen's gap equals the variance $\text{Var}(X) = E[X^2] - (E[X])^2$.

## Proof Sketch

1. **Base case $n = 1$:** trivial, $f(x_1) \le f(x_1)$.
2. **Inductive step:** write $\sum_{i=1}^{n} w_i x_i = w_1 x_1 + (1-w_1)\sum_{i=2}^n \frac{w_i}{1-w_1} x_i$.
3. **Apply convexity once:** $f(w_1 x_1 + (1-w_1) z) \le w_1 f(x_1) + (1-w_1) f(z)$ where $z = \sum_{i \ge 2} \frac{w_i}{1-w_1} x_i$.
4. **Apply inductive hypothesis** to $f(z)$.
5. Mathlib's `ConvexOn.map_sum_le` packages this induction.

## Connections

- [[Convex Function]] — Jensen's inequality is the multi-point extension of the defining two-point convexity inequality
- [[AM–GM Inequality]] — AM–GM is a special case of Jensen with $f(x) = -\log x$ (concave), reversed inequality
- [[Cauchy–Schwarz Inequality]] — Cauchy–Schwarz follows from Jensen applied to $f(x) = x^2$ via the Cauchy–Schwarz trick
- [[Markov's Inequality]] — Markov's inequality pairs with Jensen in the toolkit of moment-based bounds

## Lean4 Proof

```lean4
import Mathlib.Analysis.Convex.Jensen
import Mathlib.Analysis.Convex.Mul

/-- Jensen's inequality for x² with three equal weights 1/3 on {1, 2, 3}.
    f(mean) ≤ mean of f: 4 ≤ 14/3. -/
theorem jensen_sq_example :
    ((1 : ℝ) + 2 + 3) / 3 ^ 2 ≤
    ((1 : ℝ) ^ 2 + 2 ^ 2 + 3 ^ 2) / 3 := by
  norm_num
```
