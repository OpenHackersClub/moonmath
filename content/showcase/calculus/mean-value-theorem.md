+++
title = "Mean Value Theorem"
description = "There exists a point where the instantaneous rate of change equals the average rate of change"
weight = 20
tags = ["lean4-proof", "calculus", "visualization"]
latex = "\\exists\\, c \\in (a,b)\\;:\\; f'(c) = \\frac{f(b)-f(a)}{b-a}"
prerequisites = ["intermediate-value-theorem"]
lean4_status = "complete"
+++

## Statement

If $f$ is continuous on $[a, b]$ and differentiable on $(a, b)$, then there exists a point $c \in (a, b)$ such that:

$$f'(c) = \frac{f(b) - f(a)}{b - a}$$

The right-hand side is the slope of the secant line through $(a, f(a))$ and $(b, f(b))$. The theorem guarantees a point where the tangent slope matches this secant slope exactly.

## Visualization

Consider $f(x) = x^2$ on $[1, 3]$.

**Secant slope:** $\dfrac{f(3) - f(1)}{3 - 1} = \dfrac{9 - 1}{2} = 4$

**Tangent slope:** $f'(x) = 2x$, so $f'(c) = 4 \Rightarrow c = 2$

```
  f(x) = x²
  9 |              *  ← (3,9)
    |           /
  7 |         / secant
    |       /   slope=4
  5 |     /
    |   *·····  ← tangent at c=2, slope=4
  1 | *          ← (1,1)
    +--+---+---+--
       1   2   3
```

| $x$ | $f(x) = x^2$ | $f'(x) = 2x$ | secant slope |
|-----|-------------|--------------|--------------|
| 1   | 1           | 2            | —            |
| 2   | 4           | **4**        | **4** ✓      |
| 3   | 9           | 6            | —            |

The tangent at $c = 2$ is parallel to the secant joining the endpoints.

## Proof Sketch

1. **Define $h(x) = f(x) - L(x)$** where $L(x) = f(a) + \frac{f(b)-f(a)}{b-a}(x-a)$ is the secant line.
2. **Boundary values:** $h(a) = h(b) = 0$.
3. **Apply Rolle's Theorem:** Since $h$ is continuous on $[a, b]$, differentiable on $(a, b)$, and $h(a) = h(b)$, there exists $c \in (a, b)$ with $h'(c) = 0$.
4. **Conclude:** $h'(c) = f'(c) - \frac{f(b)-f(a)}{b-a} = 0$, so $f'(c) = \frac{f(b)-f(a)}{b-a}$.

Rolle's Theorem itself follows from the [[Intermediate Value Theorem]] applied to the derivative.

## Connections

- [[Chain Rule]] — MVT is used in the proof of the chain rule for Lipschitz compositions
- [[Intermediate Value Theorem]] — Rolle's Theorem (used in the MVT proof) is a special case of IVT applied to $h'$
- [[Fundamental Theorem of Calculus]] — MVT is the key lemma showing antiderivatives differ by at most a constant
- [[Taylor's Theorem]] — Taylor's theorem is a higher-order generalisation of the MVT
- [[L'Hopital's Rule]] — L'Hôpital's rule is proved using a generalised form of the MVT (Cauchy's MVT)

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.MeanValue

/-- Mean Value Theorem: wraps Mathlib's `exists_hasDerivAt_eq_slope`. -/
theorem mvt {f f' : ℝ → ℝ} {a b : ℝ} (hab : a < b)
    (hfc : ContinuousOn f (Set.Icc a b))
    (hff' : ∀ x ∈ Set.Ioo a b, HasDerivAt f (f' x) x) :
    ∃ c ∈ Set.Ioo a b, f' c = (f b - f a) / (b - a) :=
  exists_hasDerivAt_eq_slope f f' hab hfc hff'
```
