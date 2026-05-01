+++
title = "Handshake Lemma"
description = "The sum of all vertex degrees in a finite graph equals twice the number of edges."
weight = 131
tags = ["lean4-proof", "graph-theory", "visualization"]
latex = "\\sum_{v \\in V} \\deg(v) = 2|E|"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For any finite simple graph $G = (V, E)$:

$$\sum_{v \in V} \deg(v) = 2|E|$$

Every edge $\{u, v\}$ contributes exactly 1 to $\deg(u)$ and 1 to $\deg(v)$, so counting degrees counts each edge twice. As a corollary, the number of vertices with odd degree is always even.

## Visualization

$K_4$: the complete graph on 4 vertices, every vertex has degree 3.

```
    1
   /|\
  / | \
 2--+--3
  \ | /
   \|/
    4

Vertices: {1, 2, 3, 4}
Edges:    {12, 13, 14, 23, 24, 34}  →  |E| = 6
Degrees:  deg(1) = 3, deg(2) = 3, deg(3) = 3, deg(4) = 3
Sum:      3 + 3 + 3 + 3 = 12 = 2 × 6  ✓
```

| Vertex | Neighbors | Degree |
|--------|-----------|--------|
| 1 | 2, 3, 4 | 3 |
| 2 | 1, 3, 4 | 3 |
| 3 | 1, 2, 4 | 3 |
| 4 | 1, 2, 3 | 3 |
| **Sum** | | **12 = 2 × 6** |

## Proof Sketch

1. Form the set of **darts** (directed edge endpoints): for each undirected edge $\{u,v\}$ create two darts $(u,v)$ and $(v,u)$.
2. Count darts two ways. Grouping by tail vertex: each $v$ contributes $\deg(v)$ darts, total $\sum_v \deg(v)$.
3. Grouping by underlying edge: each edge contributes exactly 2 darts, total $2|E|$.
4. Equating the two counts gives $\sum_v \deg(v) = 2|E|$.

## Connections

The degree-sum formula is the combinatorial shadow of [[Binomial Theorem]]-style double-counting arguments. The corollary that every graph has an even number of odd-degree vertices is used in [[Inclusion–Exclusion Principle]] combinatorics and underlies Euler circuit conditions. The formula is also the starting point for spectral graph theory and the [[Matrix-Tree Theorem]].

## Lean4 Proof

```lean4
import Mathlib.Combinatorics.SimpleGraph.DegreeSum

/-- The Handshake Lemma: sum of all degrees equals twice the edge count.
    Mathlib: `SimpleGraph.sum_degrees_eq_twice_card_edges`. -/
theorem handshake_lemma {V : Type*} [Fintype V] [DecidableEq V]
    (G : SimpleGraph V) [DecidableRel G.Adj] :
    ∑ v, G.degree v = 2 * #G.edgeFinset :=
  G.sum_degrees_eq_twice_card_edges
```
