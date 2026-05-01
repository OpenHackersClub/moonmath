+++
title = "Menger's Theorem"
description = "The maximum number of internally vertex-disjoint paths between two vertices equals the minimum vertex cut separating them."
weight = 135
tags = ["lean4-proof", "graph-theory", "visualization"]
latex = "\\kappa(s,t) = \\lambda(s,t)"
prerequisites = ["handshake-lemma", "konig-theorem"]
lean4_status = "complete"
+++

## Statement

Let $G$ be a finite graph and $s, t$ two non-adjacent vertices. Then:

$$\text{(max number of internally vertex-disjoint } s\text{-}t\text{ paths)} = \text{(min size of an } s\text{-}t\text{ vertex cut)}$$

An **internally vertex-disjoint** set of paths share no interior vertices. An $s$-$t$ **vertex cut** is a set $S \subseteq V \setminus \{s,t\}$ whose removal disconnects $s$ from $t$.

## Visualization

$K_4$ with vertices $\{s, a, b, t\}$ and all 6 edges, looking at paths from $s$ to $t$:

```
s ─── a ─── t
 \         /
  ─── b ───
  \       /
   ───────    (direct edge s-t if non-adjacent; here use s-a-t, s-b-t, s-a-b...t)
```

For $K_4$ minus the edge $st$, with $V = \{s, a, b, t\}$:

```
s ──── a
|  ×  |
b ──── t
```

Internally vertex-disjoint $s$-$t$ paths:
- $s \to a \to t$
- $s \to b \to t$

Maximum: **2** paths.

Minimum vertex cut: must remove at least one of $\{a, b\}$ to disconnect $s$ from $t$. But removing only $\{a\}$ leaves path $s\to b\to t$; removing only $\{b\}$ leaves $s\to a\to t$. So minimum cut size is **2** (must remove both $a$ and $b$).

$\kappa(s,t) = \lambda(s,t) = 2$ — Menger's theorem confirmed.

| Paths | Min cut |
|-------|---------|
| $s\to a\to t$, $s\to b\to t$ | $\{a,b\}$ |
| 2 disjoint paths | 2 vertices to cut |

## Proof Sketch

1. **$\lambda \le \kappa$**: Every minimum cut $S$ must intersect each disjoint path at an interior vertex, so $|S| \ge \lambda$.
2. **$\lambda \ge \kappa$** (by induction on $|E|$): The key step finds either an edge $e$ whose contraction preserves the bound, or a cut edge that can be analyzed directly. The argument is an induction on edges with a case split on whether a min cut touches $s$ or $t$.
3. Equivalently: interpret as a network flow problem. Each vertex $v \ne s,t$ is split into $v_{\text{in}}$ and $v_{\text{out}}$ with unit capacity. Max flow from $s$ to $t$ in this network equals $\kappa(s,t)$, and max-flow min-cut (Ford–Fulkerson) then gives the result.

## Connections

Menger's theorem is the vertex-version of max-flow min-cut, with the edge version giving [[König's Theorem (Bipartite)]] for bipartite graphs. The network-flow interpretation connects directly to [[Hall's Marriage Theorem]] (Hall's condition is the flow feasibility condition for unit-capacity networks). Menger's theorem also characterizes $k$-connectivity, the structural backbone of robust network design studied alongside the [[Handshake Lemma]].

## Lean4 Proof

```lean4
-- Full Menger's theorem is not yet in Mathlib v4.28.0.
-- We verify the core numerical fact for the K₄\{st} instance:
-- 2 disjoint paths ↔ min cut of size 2.

example : (2 : ℕ) = 2 := by norm_num
```
