+++
title = "Rolle's Theorem"
description = "Between two equal values of a differentiable function there exists a point with zero derivative"
weight = 70
tags = ["lean4-proof", "calculus", "visualization"]
latex = "f(a)=f(b)\\implies\\exists\\,c\\in(a,b):\\;f'(c)=0"
prerequisites = ["intermediate-value-theorem", "mean-value-theorem"]
lean4_status = "complete"
+++

## Statement

Let $f : \mathbb{R} \to \mathbb{R}$ be continuous on $[a, b]$ and differentiable on $(a, b)$. If $f(a) = f(b)$, then there exists at least one point $c \in (a, b)$ such that

$$f'(c) = 0$$

In other words, whenever a smooth curve returns to its starting height, it must have had a horizontal tangent somewhere in between.

## Visualization

Consider $f(x) = (x-1)(x-3) = x^2 - 4x + 3$ on $[1, 3]$.

Note $f(1) = 0$ and $f(3) = 0$, so $f(1) = f(3)$.

```
  f(x) = (x-1)(x-3)

  0 |*-----------*   ← f(1)=0, f(3)=0
    | \         /
 -1 |  \       /
    |   ·-----·      ← tangent at c=2 is horizontal
 -1 |    \   /
    |     \ /
 -1 |      *         ← minimum at x=2, f(2)=-1
    +--+---+---+--
       1   2   3
```

**Derivative:** $f'(x) = 2x - 4$, so $f'(c) = 0 \Rightarrow c = 2 \in (1, 3)$.

| $x$ | $f(x)$ | $f'(x)$ |
|-----|--------|---------|
| 1   | 0      | $-2$    |
| 2   | $-1$   | **0** ✓ |
| 3   | 0      | $2$     |

The horizontal tangent at $c = 2$ is guaranteed by Rolle's Theorem.

## Proof Sketch

1. **Trivial case:** If $f$ is constant on $[a, b]$, then $f' \equiv 0$ everywhere and any $c$ works.
2. **Non-trivial case:** Since $f$ is continuous on the compact set $[a, b]$, it attains its maximum and minimum (Extreme Value Theorem).
3. **Interior extremum:** Since $f(a) = f(b)$ and $f$ is not constant, at least one of the extrema is attained at an interior point $c \in (a, b)$.
4. **Zero derivative:** At an interior local extremum of a differentiable function, $f'(c) = 0$ (Fermat's criterion). If $f'(c) > 0$ or $f'(c) < 0$, then $f$ is locally monotone at $c$, contradicting the extremum.

## Connections

- [[Mean Value Theorem]] — the MVT is proved by applying Rolle's Theorem to $h(x) = f(x) - L(x)$ where $L$ is the secant line
- [[Intermediate Value Theorem]] — the existence of an interior extremum uses continuity on a compact interval, the same hypothesis as IVT
- [[Taylor's Theorem]] — Taylor's theorem in Lagrange remainder form uses the MVT repeatedly, which in turn rests on Rolle

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.LocalExtr.Rolle

/-- Rolle's Theorem: wraps Mathlib's `exists_hasDerivAt_eq_zero`. -/
theorem rolles_theorem {f f' : ℝ → ℝ} {a b : ℝ} (hab : a < b)
    (hfc : ContinuousOn f (Set.Icc a b))
    (hfI : f a = f b)
    (hff' : ∀ x ∈ Set.Ioo a b, HasDerivAt f (f' x) x) :
    ∃ c ∈ Set.Ioo a b, f' c = 0 :=
  exists_hasDerivAt_eq_zero hab hfc hfI hff'
```
