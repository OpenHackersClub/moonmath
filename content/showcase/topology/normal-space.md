+++
title = "Normal Space"
description = "A topological space is normal if disjoint closed sets can be separated by disjoint open neighborhoods"
weight = 180
tags = ["lean4-proof", "topology", "visualization"]
latex = "\\forall\\, F,G\\text{ closed}, F\\cap G=\\emptyset:\\;\\exists\\,U\\supseteq F,\\,V\\supseteq G\\text{ open},\\;U\\cap V=\\emptyset"
prerequisites = ["regular-space"]
lean4_status = "complete"
+++

## Statement

A topological space $X$ is **normal** (a $T_4$ space when also $T_1$) if for every pair of disjoint closed sets $F$ and $G$ there exist disjoint open sets $U \supseteq F$ and $V \supseteq G$:

$$\forall\, F, G \text{ closed},\; F \cap G = \emptyset \;\Longrightarrow\; \exists\, U \supseteq F,\; V \supseteq G \text{ open},\; U \cap V = \emptyset.$$

Every metric space is normal — the distance function $d(x, F)$ provides explicit separating open sets.

## Visualization

Two disjoint closed disks in $\mathbb{R}^2$ — explicit separating opens:

```
ℝ² plane (normal space via Euclidean metric):

   F = closed disk of radius 1 at (-2, 0)
   G = closed disk of radius 1 at (+2, 0)

   distance from F:  d(x, F) = max(0, |x - (-2,0)| - 1)
   distance from G:  d(x, G) = max(0, |x - (+2,0)| - 1)

        U (open, contains F)       V (open, contains G)
          ╭──────────╮           ╭──────────╮
         /    ╭────╮  \         /  ╭────╮    \
        │    │  F  │   │       │  │  G  │    │
        │    │  ●  │   │       │  │  ●  │    │
         \    ╰────╯  /         \  ╰────╯    /
          ╰──────────╯           ╰──────────╯
               │ ← gap ─────────────────────────
               U = {x : d(x,F) < 1/2}       V = {x : d(x,G) < 1/2}

  U ∩ V = ∅  since  d(F,G) = 2 > 1/2 + 1/2 = 1
```

Normal vs. not normal:

| Space | Normal? | Reason |
|---|---|---|
| $\mathbb{R}^n$ (metric) | yes | metric separation |
| $[0,1]$ | yes | compact Hausdorff |
| $\mathbb{Q}$ (subspace of $\mathbb{R}$) | yes | metrizable |
| Long line | no | not normal ($T_3$ but not $T_4$) |
| Niemytzki plane | no | normal but not perfectly normal |

## Proof Sketch

**Metric spaces are normal:**

1. For disjoint closed sets $F, G \subseteq X$ (metric space), define $f(x) = d(x,F)/(d(x,F) + d(x,G))$.
2. The denominator is positive since $F \cap G = \emptyset$: if $d(x,F) = 0$ then $x \in F$ (closed), so $d(x,G) > 0$.
3. $f$ is continuous, $f|_F = 0$, $f|_G = 1$.
4. Set $U = f^{-1}([0, 1/2))$ and $V = f^{-1}((1/2, 1])$; both open, disjoint, and contain $F$, $G$ respectively.

This is precisely what Urysohn's lemma formalizes, and it shows metrizable $\Rightarrow$ normal.

## Connections

- [[Urysohn's Lemma]] — Urysohn's lemma is the precise statement that in a normal space one can find a continuous real-valued function separating any two disjoint closed sets; normality is the exact hypothesis needed.
- [[Regular Space]] — normality strengthens regularity: regular spaces separate a point from a closed set; normal spaces separate two closed sets. Every metric space satisfies both.
- [[Heine–Borel Theorem]] — compact Hausdorff spaces are normal ($T_4$); Heine–Borel identifies the compact sets in $\mathbb{R}^n$ where normality applies.
- [[Tychonoff's Theorem]] — a product of normal spaces need not be normal in general; Tychonoff's theorem compactifies, and compact Hausdorff spaces are always normal.

## Lean4 Proof

```lean4
import Mathlib.Topology.GDelta.MetrizableSpace

/-- Every pseudo-metrizable space is normal.
    Mathlib: the instance `[PseudoMetrizableSpace X] : NormalSpace X`
    lives in `Mathlib.Topology.GDelta.MetrizableSpace`. -/
theorem pseudoMetrizable_normal
    {X : Type*} [TopologicalSpace X] [PseudoMetrizableSpace X] : NormalSpace X :=
  inferInstance
```
