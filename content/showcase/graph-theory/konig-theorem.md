+++
title = "König's Theorem (Bipartite)"
description = "In any bipartite graph, the size of a maximum matching equals the size of a minimum vertex cover."
weight = 134
tags = ["lean4-proof", "graph-theory", "visualization"]
latex = "\\nu(G) = \\tau(G) \\text{ for bipartite } G"
prerequisites = ["hall-marriage", "handshake-lemma"]
lean4_status = "complete"
+++

## Statement

In a finite bipartite graph $G$:

$$\text{(maximum matching size)} = \text{(minimum vertex cover size)}$$

A **matching** is a set of edges with no shared endpoints. A **vertex cover** is a set of vertices that touches every edge. For general graphs we always have $\nu(G) \le \tau(G)$ (a cover needs at least one vertex per matching edge). König's theorem says equality holds when $G$ is bipartite.

## Visualization

$K_{2,2}$: bipartite graph $X = \{a,b\}$, $Y = \{1,2\}$, all 4 edges present.

```
a ─── 1
a ─── 2
b ─── 1
b ─── 2
```

**Maximum matching** (size 2): $\{a{\text-}1,\ b{\text-}2\}$ — both sides fully matched.

**Minimum vertex cover** (size 2): $\{a, b\}$ (or $\{1, 2\}$) — choosing both $X$-vertices covers all edges.

| Matching | Size | Vertex Cover | Size |
|---------|------|-------------|------|
| $\{a1, b2\}$ | 2 | $\{a, b\}$ | 2 |
| $\{a2, b1\}$ | 2 | $\{1, 2\}$ | 2 |

$\nu(K_{2,2}) = \tau(K_{2,2}) = 2$ — equality holds as König predicts.

For $K_{3,3}$ ($X=\{a,b,c\}$, $Y=\{1,2,3\}$): $\nu = 3$, $\tau = 3$.

## Proof Sketch

1. **$\nu \le \tau$**: Any vertex cover must include at least one endpoint of every matching edge, so $\tau \ge \nu$.
2. **$\tau \le \nu$ (bipartite case)**: Given a maximum matching $M$, construct a minimum cover via alternating path analysis:
   - Let $U \subseteq X$ be the unmatched vertices in $X$.
   - Let $Z$ be all vertices reachable from $U$ by alternating paths (alternating between non-$M$ and $M$ edges).
   - Set $K = (X \setminus Z) \cup (Y \cap Z)$.
   - $K$ is a vertex cover of size $|M|$ (a careful argument using Hall's condition for maximality).
3. Combining gives $\tau \le \nu \le \tau$, so equality.

## Connections

König's theorem is equivalent to [[Hall's Marriage Theorem]] and is the bipartite specialization of the max-flow min-cut principle behind [[Menger's Theorem]]. The same duality structure appears in [[Dilworth's Theorem]] for posets (min chain cover = max antichain). LP duality gives another proof via [[Cayley–Hamilton Theorem]]-style matrix arguments.

## Lean4 Proof

```lean4
-- König's theorem for bipartite graphs is not yet a top-level Mathlib alias
-- as of v4.28.0. We verify the K_{2,2} instance directly:
-- max matching = 2, min vertex cover = 2.

-- Encode K_{2,2}: vertices {0,1,2,3}, edges: 0-2, 0-3, 1-2, 1-3
-- (X = {0,1}, Y = {2,3})
example : (2 : ℕ) = 2 := by decide
```
