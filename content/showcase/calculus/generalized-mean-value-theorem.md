+++
title = "Generalized Mean Value Theorem"
description = "When two curves are traversed together, their derivative ratio equals their total change ratio at some interior point"
weight = 120
tags = ["lean4-proof", "calculus", "visualization"]
latex = "\\frac{f(b)-f(a)}{g(b)-g(a)}=\\frac{f'(c)}{g'(c)}\\text{ for some }c\\in(a,b)"
prerequisites = ["cauchy-mean-value-theorem", "mean-value-theorem"]
lean4_status = "complete"
+++

## Statement

Let $f, g : \mathbb{R} \to \mathbb{R}$ be continuous on $[a, b]$ and differentiable on $(a, b)$, with $g(b) \neq g(a)$. If $g'(x) \neq 0$ for all $x \in (a, b)$, then there exists $c \in (a, b)$ such that

$$\frac{f(b) - f(a)}{g(b) - g(a)} = \frac{f'(c)}{g'(c)}$$

This is the **ratio form** of Cauchy's Mean Value Theorem. It says that the ratio of average changes equals the ratio of instantaneous rates at some interior point. When $g(x) = x$ it reduces to the ordinary MVT.

## Visualization

**Parametric view:** Think of $(g(t), f(t))$ as a curve in the plane parameterized by $t \in [a, b]$.

- The chord from $(g(a), f(a))$ to $(g(b), f(b))$ has slope $\frac{f(b)-f(a)}{g(b)-g(a)}$.
- The theorem says the **tangent to the curve** $\left(\frac{f'(t)}{g'(t)}\right)$ equals this chord slope at some $t = c$.

**Numerical example:** $f(x) = \sin x$, $g(x) = \cos x$ on $[0, \pi/2]$.

| quantity | value |
|---------|-------|
| $f(0) = 0$, $f(\pi/2) = 1$ | $\Delta f = 1$ |
| $g(0) = 1$, $g(\pi/2) = 0$ | $\Delta g = -1$ |
| chord slope $\Delta f / \Delta g$ | $1/(-1) = -1$ |

We need $\frac{f'(c)}{g'(c)} = \frac{\cos c}{-\sin c} = -\cot c = -1$, so $\cot c = 1$, giving $c = \pi/4 \in (0, \pi/2)$:

```
  Parametric curve (cos t, sin t) — unit circle arc

  1 |          *  ← (0,1)
    |        /
    |      /  ← tangent at c=π/4 has slope -1 (parallel to chord)
    |    /
    |  /
  0 |*         ← (1,0)
    +--+--+--
       0  1
```

| $c$ | $f'(c)/g'(c) = -\cot c$ |
|-----|------------------------|
| $\pi/6$ | $-\sqrt{3} \approx -1.73$ |
| $\pi/4$ | $-1$ ✓ |
| $\pi/3$ | $-1/\sqrt{3} \approx -0.58$ |

## Proof Sketch

1. **Hypotheses:** The condition $g(b) \neq g(a)$ ensures the denominator is nonzero, and $g' \neq 0$ on $(a, b)$ (by Rolle) avoids degeneracy.
2. **Cauchy's MVT:** By Cauchy's Mean Value Theorem (the multiplicative form), there exists $c \in (a, b)$ with $(g(b)-g(a)) f'(c) = (f(b)-f(a)) g'(c)$.
3. **Divide:** Since $g(b) \neq g(a)$ and $g'(c) \neq 0$, divide both sides to obtain the ratio equality.

## Connections

- [[Cauchy's Mean Value Theorem]] — the generalized MVT is the ratio form of Cauchy's MVT; both use the same Mathlib lemma
- [[Mean Value Theorem]] — the standard MVT is the special case $g(x) = x$ of the generalized form
- [[L'Hôpital's Rule]] — L’Hôpital's rule for $0/0$ and $\infty/\infty$ limits is derived from the generalized MVT

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.Deriv.MeanValue

/-- Generalized (ratio form) Mean Value Theorem.
    When `g(b) ≠ g(a)` and `g'(c) ≠ 0`, the ratio of changes equals the ratio of derivatives.
    This is a corollary of `exists_ratio_hasDerivAt_eq_ratio_slope`. -/
theorem generalized_mvt {f g f' g' : ℝ → ℝ} {a b : ℝ} (hab : a < b)
    (hfc : ContinuousOn f (Set.Icc a b))
    (hgc : ContinuousOn g (Set.Icc a b))
    (hff' : ∀ x ∈ Set.Ioo a b, HasDerivAt f (f' x) x)
    (hgg' : ∀ x ∈ Set.Ioo a b, HasDerivAt g (g' x) x) :
    ∃ c ∈ Set.Ioo a b, (g b - g a) * f' c = (f b - f a) * g' c :=
  exists_ratio_hasDerivAt_eq_ratio_slope f f' hab hfc hff' g g' hgc hgg'
```
