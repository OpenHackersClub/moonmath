+++
title = "Intermediate Value Theorem"
description = "A continuous function on a closed interval hits every value between its endpoints"
weight = 60
tags = ["lean4-proof", "calculus", "visualization"]
latex = "f(a) \\le y \\le f(b) \\Rightarrow \\exists\\, c \\in [a,b]:\\; f(c) = y"
prerequisites = []
lean4_status = "complete"
+++

## Statement

If $f$ is continuous on the closed interval $[a, b]$ and $y$ is any value strictly between $f(a)$ and $f(b)$, then there exists at least one $c \in [a, b]$ with $f(c) = y$.

Equivalently, the image $f([a, b])$ contains the interval $[\min(f(a), f(b)),\, \max(f(a), f(b))]$.

## Visualization

**Sign-change detection** — the most common application:

| $x$ | $p(x) = x^3 - x - 1$ | Sign |
|-----|----------------------|------|
| 0   | $-1$                 | $-$  |
| 1   | $-1$                 | $-$  |
| 2   | $5$                  | $+$  |
| 1.3 | $0.197$              | $+$  |
| 1.2 | $-0.272$             | $-$  |
| 1.32| $0.297...$           | $+$  |
| 1.32| $\approx 0.035$      | $+$  |

Sign changes from $-$ to $+$ on $[1, 2]$, so by IVT a root $c \in (1, 2)$ exists (the actual root is $c \approx 1.3247$).

```
p(x) = x³ - x - 1

 5 |                     *  (2, 5)
   |
 0 +······················ ← root c ≈ 1.32
   |              *
-1 | * ···· *
   +--+--+--+--+--+--+--
      0  0.5  1  1.5  2
```

Bisection algorithm exploits IVT: repeatedly halve $[a, b]$ at the sign-change bracket to converge on $c$.

## Proof Sketch

1. **WLOG $f(a) \le y \le f(b)$** (the reversed case is symmetric).
2. **Define $S = \{x \in [a, b] : f(x) \le y\}$.** $S$ is non-empty (contains $a$) and bounded above by $b$.
3. **Let $c = \sup S$.** By definition $a \le c \le b$.
4. **Show $f(c) = y$ by contradiction:** if $f(c) < y$, continuity gives a neighbourhood where $f < y$, contradicting $c = \sup S$. If $f(c) > y$, similarly contradicts the definition of $c$ as a supremum.
5. **Conclude** $f(c) = y$ and $c \in [a, b]$.

## Connections

- [[Mean Value Theorem]] — Rolle's Theorem (used in MVT's proof) relies on IVT to locate extrema
- [[Fundamental Theorem of Calculus]] — continuity hypotheses in FTC are validated via IVT arguments
- [[L’Hôpital's Rule]] — the squeeze step in L'Hôpital's proof uses IVT to trap $c_x$ between $a$ and $x$
- [[Taylor's Theorem]] — existence of the Lagrange remainder point $c$ follows from IVT applied to an auxiliary function
- [[Chain Rule]] — continuity (a prerequisite for the chain rule) is IVT's core hypothesis

## Lean4 Proof

```lean4
import Mathlib.Topology.Order.IntermediateValue

open Set

/-- Intermediate Value Theorem: a continuous function on `[a, b]` takes every value
    between `f a` and `f b`. Wraps Mathlib's `intermediate_value_Icc`. -/
theorem ivt {f : ℝ → ℝ} {a b : ℝ} (hab : a ≤ b)
    (hf : ContinuousOn f (Icc a b)) :
    Icc (f a) (f b) ⊆ f '' Icc a b :=
  intermediate_value_Icc hab hf
```
