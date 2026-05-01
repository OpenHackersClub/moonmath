+++
title = "Convex Function"
description = "A function is convex when the chord between any two points lies above the graph"
weight = 20
tags = ["lean4-proof", "optimization", "visualization"]
latex = "f(\\lambda x + (1-\\lambda)y) \\le \\lambda f(x) + (1-\\lambda)f(y)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

A function $f : \mathbb{R} \to \mathbb{R}$ is **convex** if for all $x, y$ and $\lambda \in [0,1]$:

$$f(\lambda x + (1-\lambda)y) \le \lambda f(x) + (1-\lambda)f(y).$$

Geometrically: the chord joining $(x, f(x))$ to $(y, f(y))$ lies on or above the graph of $f$.

**Key example:** $f(x) = x^2$ is convex on all of $\mathbb{R}$ because $2 = \text{even}$. Mathlib records this as `Even.convexOn_pow`: for any even $n$, $x \mapsto x^n$ is convex on $\mathbb{R}$ (univ).

## Visualization

**Chord above curve** for $f(x) = x^2$ between $x = 0$ and $x = 2$:

```
  f(x) = x²
  4 |        * (2, 4)
    |       /|
  3 |      / |  chord from (0,0) to (2,4)
    |     /  |  slope = 2, chord value at x=1: 2
  2 |    /   |
    |   *    |  graph value at x=1: 1
  1 |  /·    |  chord (2) ≥ graph (1) ✓
    | / ·    |
  0 *--------+---
    0    1   2
```

| $x$ | $f(x) = x^2$ | chord at $x$ (from $0$ to $2$) | chord $\ge$ graph? |
|-----|-------------|-------------------------------|-------------------|
| 0   | 0           | 0                             | yes               |
| 1   | 1           | 2                             | yes               |
| 2   | 4           | 4                             | yes (equal)       |

At $\lambda = 1/2$: $f(1) = 1 \le \frac{1}{2} \cdot 0 + \frac{1}{2} \cdot 4 = 2$. Convexity holds.

**Second-derivative test:** $f''(x) = 2 > 0$ everywhere, so $f$ is strictly convex.

## Proof Sketch

1. **Expand the convexity inequality** for $f(x) = x^2$: need $(\lambda x + (1-\lambda)y)^2 \le \lambda x^2 + (1-\lambda)y^2$.
2. **Rearrange:** $\lambda(1-\lambda)(x-y)^2 \ge 0$, which holds since $\lambda \in [0,1]$.
3. **Mathlib path:** `Even.convexOn_pow` (for $n$ even, applied with $n = 2$, `hn : Even 2`) proves `ConvexOn ℝ Set.univ (fun x ↦ x ^ 2)`. The lemma is in `Mathlib.Analysis.Convex.Mul`.

## Connections

- [[Jensen's Inequality (Convex)]] — Jensen's inequality is the direct extension of convexity to weighted sums and expectations
- [[KKT Conditions]] — KKT conditions are sufficient for constrained optimality when $f$ is convex
- [[AM–GM Inequality]] — AM–GM follows from the convexity of $x \mapsto -\log x$
- [[Cauchy–Schwarz Inequality]] — Cauchy–Schwarz can be proved using the convexity of $x \mapsto x^2$

## Lean4 Proof

```lean4
import Mathlib.Analysis.Convex.Mul

/-- x² is convex on all of ℝ, via Even.convexOn_pow with n = 2. -/
theorem sq_convexOn : ConvexOn ℝ Set.univ (fun x : ℝ ↦ x ^ 2) :=
  even_two.convexOn_pow
```
