+++
title = "Alexandrov One-Point Compactification"
description = "Adding a single point at infinity to a locally compact Hausdorff space yields a compact space"
weight = 200
tags = ["lean4-proof", "topology", "visualization"]
latex = "X\\text{ locally compact Hausdorff} \\Rightarrow X^+ = X\\cup\\{\\infty\\}\\text{ compact}"
prerequisites = ["heine-borel"]
lean4_status = "complete"
+++

## Statement

Let $X$ be a locally compact Hausdorff space. Form $X^+ = X \cup \{\infty\}$ (the **one-point compactification** or **Alexandrov compactification**) with the topology:

- Every open set of $X$ is open in $X^+$.
- A set $U \ni \infty$ is open in $X^+$ if and only if $X \setminus U$ is compact and closed in $X$.

Then $X^+$ is **compact**. Moreover, if $X$ is locally compact and $T_2$, then $X^+$ is $T_2$ as well.

In Mathlib, this is `OnePoint X` with the instance `CompactSpace (OnePoint X)`.

## Visualization

$\mathbb{R} \cup \{\infty\} \cong S^1$ via stereographic projection:

```
Stereographic projection: ℝ → S¹ \ {N}  (N = north pole)

  S¹ (unit circle in ℝ²):

              N = ∞ (north pole)
              ●
             /|\
            / | \
           /  |  \
     ─────●───┼───●──────  ← ℝ (number line, south equator)
          -1  0   1
              │
     Line through N and x ∈ ℝ hits S¹ at unique point φ(x)

  As x → ±∞ along ℝ,  φ(x) → N = ∞

  φ: ℝ → S¹\{N},  φ(x) = ( 2x/(x²+1),  (x²-1)/(x²+1) )

  OnePoint ℝ ≅ S¹  (compact — the circle wraps around)
```

Neighborhoods of $\infty$:

```
Compact subsets of ℝ:   K = [−R, R] for large R
Open neighborhood of ∞: U_R = ℝ\[−R,R] ∪ {∞} = (−∞,−R) ∪ (R,∞) ∪ {∞}

  ──────)─────────────────────────(────────●
      -R                          R        ∞
       ↑                          ↑
  ←────────────── U_R ────────────────────→ (open in ℝ⁺)
```

## Proof Sketch

1. **Topology is well-defined:** The two types of open sets (open in $X$, and complements of compact closed sets plus $\infty$) are closed under finite intersections and arbitrary unions.
2. **Compactness:** Given an open cover $\mathcal{U}$ of $X^+$, some $U_0 \in \mathcal{U}$ contains $\infty$. By definition $K = X^+ \setminus U_0$ is compact in $X$. The remaining members of $\mathcal{U}$ cover $K$, yielding a finite subcover $\{U_1, \ldots, U_n\}$. Then $\{U_0, U_1, \ldots, U_n\}$ is a finite subcover of $X^+$.
3. **Hausdorff:** For $x, y \in X$ use local compactness; for $x \in X$ and $\infty$, take a compact neighborhood $K \ni x$ and open $U \supseteq K^c \cup \{\infty\}$.

## Connections

- [[Heine–Borel Theorem]] — Heine–Borel characterizes compact subsets of $\mathbb{R}^n$; the one-point compactification makes these the "finite" part of $X^+$, with neighborhoods of $\infty$ being complements of compact sets.
- [[Tychonoff's Theorem]] — Tychonoff gives compactness of arbitrary products; one-point compactification gives compactness by adding a single point. Both are tools for producing compact spaces.
- [[Continuous Bijection on Compact-Hausdorff is Homeo]] — the homeomorphism $\mathbb{R}^+ \cong S^1$ is an instance: the stereographic projection is a continuous bijection from a compact space to a Hausdorff space.
- [[Urysohn's Lemma]] — $X^+$ is normal when $X$ is locally compact $T_2$ (Mathlib: `OnePoint.instNormalSpace`); Urysohn's lemma then applies to $X^+$.

## Lean4 Proof

```lean4
import Mathlib.Topology.Compactification.OnePoint.Basic

/-- **Alexandrov one-point compactification** is compact.
    `OnePoint X` is Mathlib's name for X ∪ {∞};
    `CompactSpace (OnePoint X)` is a direct instance. -/
theorem onePoint_compact (X : Type*) [TopologicalSpace X] :
    CompactSpace (OnePoint X) :=
  inferInstance
```
