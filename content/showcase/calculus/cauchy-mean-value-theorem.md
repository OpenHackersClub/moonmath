+++
title = "Cauchy's Mean Value Theorem"
description = "A two-function generalization of the Mean Value Theorem relating derivative ratios to function value ratios"
weight = 80
tags = ["lean4-proof", "calculus", "visualization"]
latex = "\\exists\\,c\\in(a,b):\\;(g(b)-g(a))f'(c)=(f(b)-f(a))g'(c)"
prerequisites = ["mean-value-theorem", "rolle-theorem"]
lean4_status = "complete"
+++

## Statement

Let $f, g : \mathbb{R} \to \mathbb{R}$ both be continuous on $[a, b]$ and differentiable on $(a, b)$. Then there exists $c \in (a, b)$ such that

$$(g(b) - g(a)) \cdot f'(c) = (f(b) - f(a)) \cdot g'(c)$$

When $g'(c) \neq 0$ and $g(b) \neq g(a)$ this rearranges to

$$\frac{f'(c)}{g'(c)} = \frac{f(b) - f(a)}{g(b) - g(a)}$$

The ordinary Mean Value Theorem is the special case $g(x) = x$.

## Visualization

Take $f(x) = x^2$ and $g(x) = x^3$ on $[1, 2]$.

**Values:**

| quantity | value |
|---------|-------|
| $f(1) = 1$, $f(2) = 4$ | $f(2)-f(1) = 3$ |
| $g(1) = 1$, $g(2) = 8$ | $g(2)-g(1) = 7$ |
| ratio $\frac{f(b)-f(a)}{g(b)-g(a)}$ | $\frac{3}{7}$ |

**Derivatives:** $f'(x) = 2x$, $g'(x) = 3x^2$.

We need $\frac{f'(c)}{g'(c)} = \frac{2c}{3c^2} = \frac{2}{3c} = \frac{3}{7}$, so:

$$c = \frac{14}{9} \approx 1.556 \in (1, 2) \checkmark$$

| $x$ | $f'(x)$ | $g'(x)$ | ratio $f'/g'$ |
|-----|---------|---------|--------------|
| 1.0 | 2.0     | 3.0     | 0.667        |
| **1.556** | **3.111** | **7.259** | **0.429 = 3/7** ✓ |
| 2.0 | 4.0     | 12.0    | 0.333        |

## Proof Sketch

1. **Auxiliary function:** Define $h(x) = (g(b) - g(a)) f(x) - (f(b) - f(a)) g(x)$.
2. **Equal endpoints:** $h(a) = (g(b)-g(a))f(a) - (f(b)-f(a))g(a)$ and $h(b) = (g(b)-g(a))f(b) - (f(b)-f(a))g(b)$; direct algebra shows $h(a) = h(b)$.
3. **Apply Rolle's Theorem:** $h$ is continuous on $[a,b]$, differentiable on $(a,b)$, and $h(a) = h(b)$, so there exists $c \in (a,b)$ with $h'(c) = 0$.
4. **Expand:** $h'(c) = (g(b)-g(a)) f'(c) - (f(b)-f(a)) g'(c) = 0$ is exactly the conclusion.

## Connections

- [[Mean Value Theorem]] — Cauchy's MVT reduces to the standard MVT when $g(x) = x$
- [[L'Hôpital's Rule]] — L'Hopital's rule for $0/0$ indeterminate forms is proved using Cauchy's MVT
- [[Taylor's Theorem]] — higher-order Taylor remainder estimates use iterated applications of Cauchy's MVT
- [[Rolle's Theorem]] — the proof constructs an auxiliary function and applies Rolle directly

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.Deriv.MeanValue

/-- Cauchy's Mean Value Theorem: wraps `exists_ratio_hasDerivAt_eq_ratio_slope`. -/
theorem cauchy_mvt {f g f' g' : ℝ → ℝ} {a b : ℝ} (hab : a < b)
    (hfc : ContinuousOn f (Set.Icc a b))
    (hgc : ContinuousOn g (Set.Icc a b))
    (hff' : ∀ x ∈ Set.Ioo a b, HasDerivAt f (f' x) x)
    (hgg' : ∀ x ∈ Set.Ioo a b, HasDerivAt g (g' x) x) :
    ∃ c ∈ Set.Ioo a b, (g b - g a) * f' c = (f b - f a) * g' c :=
  exists_ratio_hasDerivAt_eq_ratio_slope f f' hab hfc hff' g g' hgc hgg'
```
