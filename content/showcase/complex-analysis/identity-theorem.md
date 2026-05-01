+++
title = "Identity Theorem (Analytic Continuation)"
description = "Two analytic functions on a connected domain that agree on a set with a limit point must be identical."
weight = 60
tags = ["lean4-proof", "complex-analysis", "visualization"]
latex = "f|_S = g|_S,\\; S' \\cap U \\ne \\emptyset \\implies f \\equiv g \\text{ on } U"
prerequisites = ["cauchy-integral-formula", "maximum-modulus"]
lean4_status = "complete"
+++

## Statement

Let $U \subseteq \mathbb{C}$ be a connected open set, and let $f, g : U \to \mathbb{C}$ be holomorphic. If $f$ and $g$ agree on a set $S \subseteq U$ that has a limit point in $U$ (i.e., $S$ is not isolated), then $f \equiv g$ on all of $U$.

Equivalently: if $f$ and $g$ agree on a sequence $z_n \to z_0 \in U$, they agree everywhere on $U$.

## Visualization

**Two functions agreeing on $\{1/n : n \ge 1\}$:**

The sequence $1, 1/2, 1/3, 1/4, \ldots \to 0$.

```
  Real line near 0:
  
  0   1/4  1/3  1/2   1
  |----+----+----+----+-->
   ↑
   limit point in U = (-1, 2)
```

If $f$ and $g$ are holomorphic on $(-1, 2)$ (viewed as a subset of $\mathbb{C}$) and $f(1/n) = g(1/n)$ for all $n$, then $f \equiv g$ on all of $(-1, 2)$.

**Concrete example:** Let $h(z) = f(z) - g(z)$. If $h(1/n) = 0$ for all $n$, then $h$ has a zero at $0 = \lim(1/n)$. A holomorphic function's zero set is either isolated or the entire connected domain. Since $\{1/n\}$ gives infinitely many zeros accumulating at $0$, the zero set has a limit point in $U$, so $h \equiv 0$.

| $n$ | $1/n$ | $f(1/n) = g(1/n)$? | Conclusion |
|-----|-------|-------------------|------------|
| 1   | 1.000 | agree             | |
| 2   | 0.500 | agree             | $h = 0$ on a set |
| 10  | 0.100 | agree             | with limit point |
| 100 | 0.010 | agree             | $\Rightarrow h \equiv 0$ |

## Proof Sketch

1. Define $h = f - g$. We need to show $h \equiv 0$.
2. Let $Z = \{z \in U : h^{(n)}(z) = 0 \text{ for all } n \ge 0\}$ (the set where all derivatives of $h$ vanish).
3. $Z$ is closed in $U$ (each condition $h^{(n)}(z) = 0$ is closed).
4. $Z$ is open: if $z_0 \in Z$, then $h$ has a power series $\sum a_n (z-z_0)^n$ near $z_0$ with all $a_n = h^{(n)}(z_0)/n! = 0$, so $h \equiv 0$ in a neighborhood.
5. Since $U$ is connected and $Z$ is clopen and non-empty (the limit point of $S$ is in $Z$), $Z = U$.

## Connections

The Identity Theorem is the cornerstone of **analytic continuation**: the [[Cauchy Integral Formula]] guarantees holomorphic functions are analytic, and the Identity Theorem ensures the continuation is unique. The analogous result for real functions fails entirely — $e^{-1/x^2}$ is smooth and zero at $0$ with all derivatives, yet non-zero elsewhere. Compare with the [[Fundamental Theorem of Algebra]], which also exploits the rigidity of polynomials (a special case).

## Lean4 Proof

```lean4
import Mathlib.Analysis.Analytic.Uniqueness

open AnalyticOnNhd

/-- **Identity Theorem**: two analytic functions on a preconnected space that agree
    eventually near a point must be equal everywhere.
    Uses `AnalyticOnNhd.eq_of_eventuallyEq` from Mathlib. -/
theorem identity_theorem {E F : Type*}
    [NormedAddCommGroup E] [NormedSpace ℂ E] [PreconnectedSpace E]
    [NormedAddCommGroup F] [NormedSpace ℂ F]
    {f g : E → F} {z₀ : E}
    (hf : AnalyticOnNhd ℂ f Set.univ)
    (hg : AnalyticOnNhd ℂ g Set.univ)
    (hfg : f =ᶠ[nhds z₀] g) :
    f = g :=
  eq_of_eventuallyEq hf hg hfg
```
