+++
title = "Open Mapping Theorem (Complex)"
description = "A non-constant holomorphic function on a connected open set maps open sets to open sets."
weight = 70
tags = ["lean4-proof", "complex-analysis", "visualization"]
latex = "f \\text{ holomorphic, non-constant} \\implies f(U) \\text{ open for every open } U"
prerequisites = ["maximum-modulus", "cauchy-integral-formula"]
lean4_status = "complete"
+++

## Statement

Let $U \subseteq \mathbb{C}$ be a connected open set and $f : U \to \mathbb{C}$ holomorphic and non-constant. Then $f$ is an **open map**: for every open set $V \subseteq U$, the image $f(V)$ is open in $\mathbb{C}$.

## Visualization

**$f(z) = z^2$ on the unit disk $\mathbb{D} = \{|z| < 1\}$.**

```
Domain (open disk):                Image f(D):

  Imaginary                          Imaginary
      |                                  |
  -1--+--1   open disk                -1--+--1   open disk
      |       |z| < 1                     |       |w| < 1
      |                                   |
```

The map $z \mapsto z^2$ takes every open subset of $\mathbb{D}$ to an open subset. For example, the upper half-disk $\{|z| < 1, \text{Im}(z) > 0\}$ maps to $\{|w| < 1\} \setminus [0, 1)$ — which is open.

**Contrast with real analysis:** The real map $x \mapsto x^2$ on $(-1, 1)$ maps to $[0, 1)$, which is NOT open. Complex holomorphicity is far more rigid.

```
Real analogy (fails openness):
  (-1, 1) --[x -> x^2]--> [0, 1)   NOT open!
  
Complex version (works):
  D (open) --[z -> z^2]--> D (open)  Open!
```

**Why the difference?** The map $z \mapsto z^2$ wraps angles by $2\times$, so neighborhoods of $z_0$ map surjectively onto neighborhoods of $z_0^2$. At $z_0 = 0$ the map is two-to-one but still locally surjective onto a neighborhood of $0$.

## Proof Sketch

1. Suppose $f$ is non-constant and holomorphic near $w_0 = f(z_0)$.
2. Write $f(z) - w_0 = (z - z_0)^m h(z)$ where $h(z_0) \ne 0$ and $m \ge 1$.
3. By continuity, $h$ is non-vanishing near $z_0$, and $(z - z_0)^m$ wraps a small circle around $z_0$ around $0$ exactly $m$ times (winding number $m$).
4. Rouché's theorem shows: for $|w - w_0|$ small enough, $f(z) - w$ has exactly $m$ roots near $z_0$. So every $w$ near $w_0$ is in $f(U)$.
5. Hence $f(U)$ contains a neighborhood of $w_0$, so $f(U)$ is open.

## Connections

The Open Mapping Theorem immediately gives the [[Maximum Modulus Principle]]: if $|f|$ achieves an interior maximum, then $f$ maps a neighborhood of that point onto an open set containing $f(z_0)$ — but open sets have points with larger modulus, contradiction. The theorem also implies the [[Fundamental Theorem of Algebra]] (polynomials are open maps on $\mathbb{C}$, and $\mathbb{C}$ is connected, so the image is all of $\mathbb{C}$).

## Lean4 Proof

```lean4
import Mathlib.Analysis.Complex.OpenMapping

open AnalyticOnNhd

/-- **Open Mapping Theorem**: a non-constant analytic function on a preconnected
    open set is either constant or an open map.
    Uses `AnalyticOnNhd.is_constant_or_isOpen` from Mathlib. -/
theorem open_mapping_complex {U : Set ℂ} {g : ℂ → ℂ}
    (hg : AnalyticOnNhd ℂ g U)
    (hU : IsPreconnected U) :
    (∃ c, ∀ z ∈ U, g z = c) ∨ IsOpenMap (U.restrict g) :=
  hg.is_constant_or_isOpen hU
```
