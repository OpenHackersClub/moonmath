+++
title = "Regular Space"
description = "A topological space is regular if a closed set and a disjoint point can be separated by open neighborhoods"
weight = 190
tags = ["lean4-proof", "topology", "visualization"]
latex = "\\forall\\,C\\text{ closed},\\, p\\notin C:\\;\\exists\\,U\\supseteq C,\\,V\\ni p\\text{ open},\\;U\\cap V=\\emptyset"
prerequisites = []
lean4_status = "complete"
+++

## Statement

A topological space $X$ is **regular** if for every closed set $C$ and every point $p \notin C$ there exist disjoint open sets $U \supseteq C$ and $V \ni p$:

$$\forall\, C \text{ closed},\; p \notin C \;\Longrightarrow\; \exists\, U \supseteq C,\; V \ni p \text{ open},\; U \cap V = \emptyset.$$

A $T_3$ space is regular + $T_1$ (points are closed). Every metric space is regular: the distance function $d(p, C) > 0$ provides explicit separating balls.

## Visualization

Point $p$ outside closed set $C$ — disjoint open neighborhoods:

```
Topological space X:

   C (closed)           p (point not in C)
   ╭──────────╮                ●
  /            \
 │  ████████   │             / ← V open, contains p
 │  ████████   │           ╭───╮
 │  ████████   │           │ p │
  \            /           ╰───╯
   ╰──────────╯
    ↑
    U open, contains C
    U ∩ V = ∅

In ℝ (metric space):  C = [2, 5],  p = 0
  d(p, C) = 2
  U = (1, 6)   ← contains C
  V = (−1, 1)  ← contains p
  U ∩ V = ∅ ✓
```

Separation hierarchy:

```
T₀ (Kolmogorov)
  ↓
T₁ (Fréchet) — points are closed
  ↓
T₂ (Hausdorff) — distinct points have disjoint neighborhoods
  ↓
T₃ = Regular + T₁ — point vs. closed set separated
  ↓
T₄ = Normal + T₁ — closed set vs. closed set separated
  ↓
T₆ = Perfectly normal + T₁
```

## Proof Sketch

**Uniform spaces (hence metric spaces) are regular:**

1. Let $C \subseteq X$ be closed and $p \notin C$, so $p \in X \setminus C$ (open).
2. By the uniform space axioms there exists an entourage $E$ with $E[p] \cap C = \emptyset$ (since $C^c$ is open in the uniform topology).
3. Set $V = \text{int}(E[p])$ and $U = X \setminus \overline{E^{-1}[p]^c}$. Symmetry of $E$ ensures $U \supseteq C$ and $U \cap V = \emptyset$.
4. In metric spaces: choose $r = d(p, C)/2$; then $V = B_r(p)$ and $U = \{x : d(x, C) < r\}$ work directly.

Mathlib formalizes this as: `UniformSpace.to_regularSpace`, promoted automatically whenever a `UniformSpace` instance exists.

## Connections

- [[Normal Space]] — every normal space ($T_4$) is regular ($T_3$); regularity is the weaker point-vs-set separation while normality handles set-vs-set.
- [[Heine–Borel Theorem]] — compact Hausdorff spaces are regular (in fact normal); Heine–Borel's compact sets in $\mathbb{R}^n$ inherit regularity from the Euclidean metric.
- [[Urysohn's Lemma]] — Urysohn's lemma applies to normal spaces; regular + second-countable implies normal (by Urysohn's metrization argument), so the two separation axioms interact closely.
- [[Metrizable Spaces]] — metrizable spaces are exactly those that are regular, Hausdorff, and second-countable (Urysohn metrization theorem); regularity is one of the three equivalent conditions.

## Lean4 Proof

```lean4
import Mathlib.Topology.UniformSpace.Separation

/-- Every uniform space is regular.
    Mathlib instance: `UniformSpace.to_regularSpace`. -/
theorem uniformSpace_regular
    {X : Type*} [UniformSpace X] : RegularSpace X :=
  inferInstance

/-- In particular, every (pseudo-)metric space is regular. -/
example : RegularSpace ℝ := inferInstance
```
