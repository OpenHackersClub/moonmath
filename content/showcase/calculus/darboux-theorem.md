+++
title = "Darboux's Theorem"
description = "The derivative of a differentiable function satisfies the intermediate value property, even when not continuous"
weight = 90
tags = ["lean4-proof", "calculus", "visualization"]
latex = "f'(a)<m<f'(b)\\implies\\exists\\,c\\in(a,b):\\;f'(c)=m"
prerequisites = ["intermediate-value-theorem", "mean-value-theorem"]
lean4_status = "complete"
+++

## Statement

Let $f : \mathbb{R} \to \mathbb{R}$ be differentiable on $[a, b]$ (with one-sided derivatives at endpoints). If $f'(a) < m < f'(b)$ (or $f'(b) < m < f'(a)$), then there exists $c \in (a, b)$ such that

$$f'(c) = m$$

Derivatives satisfy the **Intermediate Value Property** even when the derivative is not continuous. This distinguishes derivatives from arbitrary functions.

## Visualization

A striking example: $f(x) = x^2 \sin(1/x)$ for $x \neq 0$, $f(0) = 0$.

The derivative is $f'(x) = 2x\sin(1/x) - \cos(1/x)$ for $x \neq 0$ and $f'(0) = 0$.

Near $x = 0$, the term $-\cos(1/x)$ oscillates between $-1$ and $1$ without converging, so $f'$ is **not continuous** at $0$. Yet $f'$ achieves every value in $[-1, 1]$ near $0$:

```
  f'(x) near x=0 (schematic)

  1 |  .    .    .    .   ← peaks of cos(1/x) oscillation
    |
  0 |--+--+--+--+--+--+-- ← f'(0) = 0
    |
 -1 |    .    .    .   .  ← troughs
    +------------------------
       x → 0
```

For the simpler case $f(x) = x^2$ on $[0, 1]$: $f'(0) = 0$, $f'(1) = 2$, and every $m \in (0, 2)$ is achieved at $c = m/2$:

| $m$ | $c = m/2$ | $f'(c) = 2c$ |
|-----|-----------|-------------|
| 0.5 | 0.25      | 0.5 ✓       |
| 1.0 | 0.50      | 1.0 ✓       |
| 1.5 | 0.75      | 1.5 ✓       |

## Proof Sketch

1. **Reduce to minimum:** Without loss of generality assume $f'(a) < m < f'(b)$. Define $g(x) = f(x) - mx$, so $g'(a) < 0 < g'(b)$.
2. **Attain minimum:** $g$ is continuous on $[a, b]$ (differentiability implies continuity), so by the Extreme Value Theorem it attains its minimum at some $c \in [a, b]$.
3. **Interior point:** Since $g'(a) < 0$, the function $g$ is decreasing at $a$, so $g$ takes smaller values just to the right of $a$; hence $c \neq a$. Similarly $g'(b) > 0$ means $g$ is increasing at $b$, so $c \neq b$.
4. **Zero derivative:** At the interior minimum $c$, by Fermat's criterion $g'(c) = 0$, i.e., $f'(c) = m$.

## Connections

- [[Intermediate Value Theorem]] — Darboux's theorem is the IVT analogue for derivatives; the proof uses compactness, not IVT directly
- [[Mean Value Theorem]] — the MVT implies the derivative cannot skip values: if $f'(a) < m < f'(b)$, the MVT applied to $f - mx$ forces a critical point
- [[Extreme Value Theorem]] — the proof uses the extreme value theorem to find an interior minimum of the auxiliary function $g(x) = f(x) - mx$

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.Darboux

/-- Darboux's theorem: if `f' a < m < f' b` then `f' c = m` for some interior `c`.
    Wraps Mathlib's `exists_hasDerivWithinAt_eq_of_gt_of_lt`. -/
theorem darboux_theorem {f f' : ℝ → ℝ} {a b : ℝ} (hab : a ≤ b)
    (hf : ∀ x ∈ Set.Icc a b, HasDerivWithinAt f (f' x) (Set.Icc a b) x)
    {m : ℝ} (hma : f' a < m) (hmb : m < f' b) :
    m ∈ f' '' Set.Ioo a b :=
  exists_hasDerivWithinAt_eq_of_gt_of_lt hab hf hma hmb
```
