+++
title = "Arzelà–Ascoli Theorem"
description = "A family of continuous functions on a compact space is precompact iff it is equicontinuous and uniformly bounded"
weight = 120
tags = ["lean4-proof", "analysis", "visualization", "topology"]
latex = "\\text{equicontinuous} + \\text{pointwise bounded} \\Rightarrow \\overline{F} \\text{ compact in } C(K)"
prerequisites = []
lean4_status = "complete"
+++

The **Arzelà–Ascoli theorem** characterises compact subsets of the space $C(K)$ of continuous functions on a compact metric space. It is the workhorse for existence proofs in analysis and PDEs — sequences of approximate solutions are shown to have convergent subsequences.

## Statement

Let $K$ be a compact metric space and $(f_n)$ a sequence of continuous functions $f_n : K \to \mathbb{R}$. The sequence has a uniformly convergent subsequence if and only if:

1. **Uniform boundedness**: $\sup_n \|f_n\|_\infty < \infty$.
2. **Equicontinuity**: for every $\varepsilon > 0$ there exists $\delta > 0$ such that $d(x,y) < \delta \Rightarrow |f_n(x) - f_n(y)| < \varepsilon$ for **all** $n$ simultaneously.

More generally, $A \subseteq C(K)$ has compact closure if and only if $A$ is equicontinuous and pointwise bounded.

## Visualization

**Three families of functions — which are equicontinuous?**

Consider $f_n(x) = \sin(nx)/n$ on $K = [0, 2\pi]$.

| $n$ | $\|f_n\|_\infty$ | modulus of continuity | equicontinuous? |
|-----|------------------|------------------------|-----------------|
| 1   | $1$              | $|n\cos(nc)| \leq n$  |                 |
| 10  | $1/10$           | $\sup |f_n'| = 1$      |                 |
| 100 | $1/100$          | $\sup |f_n'| = 1$      |                 |

Since $|f_n(x) - f_n(y)| \leq |x - y| \cdot \sup|f_n'| = |x-y|$ (because $|f_n'| = |\cos(nx)| \leq 1$), the family $\{f_n\}$ is equicontinuous with modulus $\delta = \varepsilon$, and $\|f_n\|_\infty \leq 1$. Arzelà–Ascoli applies.

**Contrast: $g_n(x) = \sin(nx)$ (no $1/n$ factor):**

| $n$ | $\|g_n\|_\infty$ | modulus | equicontinuous? |
|-----|------------------|---------|-----------------|
| 1   | $1$              | $\delta = \varepsilon$ | yes |
| 10  | $1$              | need $\delta < \varepsilon/10$ | worse |
| 100 | $1$              | need $\delta < \varepsilon/100$ | worse |

For $(g_n)$, any fixed $\delta$ fails for large $n$: not equicontinuous. Indeed $(g_n)$ has no uniformly convergent subsequence.

**Schematic — equicontinuous vs. oscillatory:**

```
 f_n = sin(nx)/n       g_n = sin(nx)
  1 |                    1 |~~~~~
    |~     ~              | ~   ~   ~
  0 |  ~ ~               0 |    ~ ~
    |                    -1|
    +-----> x               +-----> x
     slowly shrinking        fixed amplitude
     equicontinuous          NOT equicontinuous
```

## Proof Sketch

1. ($\Rightarrow$) Compact sets in a metric space are totally bounded: from any sequence, extract a Cauchy subsequence. Taking $n \to \infty$ shows equicontinuity and boundedness.

2. ($\Leftarrow$) Given an equicontinuous uniformly bounded sequence $(f_n)$:
   - Pick a countable dense set $\{x_k\} \subseteq K$ (possible since $K$ is compact metric).
   - By the [[Bolzano–Weierstrass Theorem]] applied to the bounded sequence $(f_n(x_1))_n$, extract a subsequence converging at $x_1$. Repeat at $x_2, x_3, \ldots$ by diagonal extraction.
   - The diagonal subsequence converges at all $x_k$.
   - Equicontinuity promotes pointwise convergence on $\{x_k\}$ to **uniform** convergence on all of $K$ (a $3\varepsilon$-argument using density).

## Connections

Arzelà–Ascoli is the standard tool for proving existence in ODEs (Peano's theorem), integral equations, and variational problems. It is the function-space analogue of the [[Bolzano–Weierstrass Theorem]], and together they illustrate the general principle: compact = closed + totally bounded. The [[Heine–Borel Theorem]] is the same principle for finite-dimensional $\mathbb{R}^n$.

## Lean4 Proof

```lean4
import Mathlib.Topology.ContinuousMap.Bounded.ArzelaAscoli

open BoundedContinuousFunction Set

/-- Arzelà–Ascoli theorem: a closed equicontinuous subset of bounded continuous
    functions on a compact space with compact range is compact. -/
theorem arzela_ascoli_compact {α β : Type*}
    [TopologicalSpace α] [CompactSpace α]
    [PseudoMetricSpace β] [CompactSpace β]
    (A : Set (α →ᵇ β))
    (hA_closed : IsClosed A)
    (hA_equi : Equicontinuous ((↑) : A → α → β)) :
    IsCompact A :=
  arzela_ascoli₁ A hA_closed hA_equi
```
