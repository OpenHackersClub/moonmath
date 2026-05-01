+++
title = "Connected Components"
description = "Every topological space partitions into maximal connected subsets, each of which is closed"
weight = 70
tags = ["lean4-proof", "topology", "visualization"]
latex = "X = \\bigsqcup_{x} C_x, \\quad C_x \\text{ closed}"
prerequisites = ["path-connected"]
lean4_status = "complete"
+++

## Statement

For any topological space $X$ and any point $x \in X$, the **connected component** of $x$ is the union of all connected subsets containing $x$:

$$C_x = \bigcup \{S \subseteq X : x \in S,\, S \text{ connected}\}.$$

Three key facts hold simultaneously:

1. Each $C_x$ is connected (the union of connected sets sharing a point is connected).
2. The components partition $X$: distinct components are disjoint, and every point belongs to exactly one component.
3. Each $C_x$ is **closed** in $X$.

A space is **totally disconnected** if every component is a singleton (e.g. $\mathbb{Q}$).

## Visualization

**Example 1 — $\mathbb{R} \setminus \{0\}$ has two components:**

```
     C₋ = (-∞, 0)          C₊ = (0, +∞)
  ←──────────────────) 0 (──────────────────→
        component            component
        of -1                of 1

  Any path from -1 to 1 must cross 0, which is missing.
  So no connected set straddles the gap.
```

**Example 2 — $\mathbb{Q}$ is totally disconnected:**

| Rational $q$ | Component $C_q$ | Size |
|---|---|---|
| $0$ | $\{0\}$ | singleton |
| $1/2$ | $\{1/2\}$ | singleton |
| $\sqrt{2}$ | (irrational, not in $\mathbb{Q}$) | — |

Between any two rationals $p < q$ lies an irrational $r$. The open sets $(-\infty, r) \cap \mathbb{Q}$ and $(r, \infty) \cap \mathbb{Q}$ disconnect any interval, so no two rationals share a component.

**Closure:** In $\mathbb{R}$, the component $(-\infty, 0)$ is open, and also closed (its complement $(0, \infty)$ is open). In general, components need not be open — but they are always closed.

## Proof Sketch

1. **Connectedness of $C_x$:** Write $C_x = \bigcup_{\alpha} S_\alpha$ where each $S_\alpha$ contains $x$. Any two points of $C_x$ lie in some $S_\alpha$ and $S_\beta$ both containing $x$; the union $S_\alpha \cup S_\beta$ is connected (overlapping connected sets sharing a point). A union of connected sets with a common point is connected by induction.

2. **Partition:** If $C_x \cap C_y \ne \emptyset$, then $C_x \cup C_y$ is a connected set containing both $x$ and $y$, so it is contained in $C_x$ and in $C_y$, forcing $C_x = C_y$.

3. **Closedness of $C_x$:** The closure $\overline{C_x}$ is connected (closure of a connected set is connected in any topological space). Since $C_x$ is the maximal connected set containing $x$, and $\overline{C_x}$ contains $x$ and is connected, we must have $\overline{C_x} \subseteq C_x$, i.e. $C_x$ is closed.

## Connections

- **[[Heine–Borel Theorem]]** — in $\mathbb{R}^n$ the connected components of an open set are open; Heine–Borel implies each compact connected component is closed and bounded.
- **[[Tychonoff's Theorem]]** — the product of connected spaces is connected; equivalently, products of spaces with one component each have one component.
- **[[Bolzano–Weierstrass Theorem]]** — the real line is connected precisely because every bounded sequence has a convergent subsequence; total disconnectedness of $\mathbb{Q}$ is the obstruction Bolzano–Weierstrass cures by passing to $\mathbb{R}$.

## Lean4 Proof

```lean4
import Mathlib.Topology.Connected.Basic

/-- Each connected component is a closed set. -/
theorem connectedComponent_is_closed
    {X : Type*} [TopologicalSpace X] (x : X) :
    IsClosed (connectedComponent x) :=
  isClosed_connectedComponent

/-- Two points share a component iff one is in the component of the other. -/
theorem same_component_iff
    {X : Type*} [TopologicalSpace X] {x y : X} :
    connectedComponent x = connectedComponent y ↔ y ∈ connectedComponent x :=
  connectedComponent_eq_iff_mem
```
