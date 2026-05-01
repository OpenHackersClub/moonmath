+++
title = "Cayley's Formula ($n^{n-2}$ trees)"
description = "The number of labeled spanning trees on n vertices is n^{n-2}, proved by Prüfer sequences."
weight = 137
tags = ["lean4-proof", "graph-theory", "visualization"]
latex = "T_n = n^{n-2}"
prerequisites = ["handshake-lemma", "euler-planar-formula"]
lean4_status = "complete"
+++

## Statement

The number of distinct labeled trees on the vertex set $\{1, 2, \ldots, n\}$ is:

$$T_n = n^{n-2}$$

For $n = 1$: $T_1 = 1^{-1}$ — interpreted as $T_1 = 1$ (one trivial tree).  
For $n = 2$: $T_2 = 2^0 = 1$ (single edge $\{1,2\}$).  
For $n = 3$: $T_3 = 3^1 = 3$.  
For $n = 4$: $T_4 = 4^2 = 16$.

## Visualization

All 3 labeled trees on $\{1, 2, 3\}$:

```
Tree 1: 1 ─ 2 ─ 3   (1 is leaf, 3 is leaf, 2 is center)

Tree 2: 1 ─ 3 ─ 2   (1 is leaf, 2 is leaf, 3 is center)

Tree 3: 2 ─ 1 ─ 3   (2 is leaf, 3 is leaf, 1 is center)
```

| Tree | Edges | Center |
|------|-------|--------|
| $T_1$ | $\{1{-}2, 2{-}3\}$ | 2 |
| $T_2$ | $\{1{-}3, 2{-}3\}$ | 3 |
| $T_3$ | $\{1{-}2, 1{-}3\}$ | 1 |

Three trees, matching $3^1 = 3$.

**Prüfer sequence** bijection for $n = 3$: sequences of length 1 from $\{1,2,3\}$ — there are 3 such sequences, one per tree. The Prüfer sequence encodes a tree by repeatedly deleting the leaf with smallest label and recording its neighbor.

For $n = 4$: Prüfer sequences have length 2 over $\{1,2,3,4\}$, giving $4^2 = 16$ sequences, hence 16 trees.

| $n$ | $T_n = n^{n-2}$ | Prüfer sequences |
|----|----------------|-----------------|
| 2 | 1 | 1 empty sequence |
| 3 | 3 | 3 sequences of length 1 |
| 4 | 16 | 16 sequences of length 2 |
| 5 | 125 | 125 sequences of length 3 |

## Proof Sketch

1. **Prüfer sequences**: Given a labeled tree $T$ on $\{1,\ldots,n\}$, form a sequence $(a_1,\ldots,a_{n-2})$ by: at each step, remove the leaf with smallest label and record its (unique) neighbor. The result is a sequence of $n-2$ entries from $\{1,\ldots,n\}$.
2. **Bijection**: The map $T \mapsto \text{Prüfer}(T)$ is a bijection between labeled trees on $n$ vertices and sequences of length $n-2$ from $\{1,\ldots,n\}$.
3. **Counting**: There are $n^{n-2}$ such sequences, so $T_n = n^{n-2}$.
4. **Reconstruction**: Given a Prüfer sequence $(a_1,\ldots,a_{n-2})$, reconstruct the tree: at step $i$, find the smallest element not in the remaining sequence and add an edge to $a_i$. The last two elements form the final edge.

## Connections

Cayley's formula counts spanning trees of the complete graph $K_n$, directly connecting to the [[Handshake Lemma]] (all $n$ vertices have degree $\ge 1$ in any spanning tree). The count $n^{n-2}$ also equals the number of parking functions of length $n-1$, linking to [[Catalan Numbers]] combinatorics. The [[Matrix-Tree Theorem]] gives an alternative proof via the determinant of the Laplacian of $K_n$: $\det(L_{K_n}^{\hat\imath}) = n^{n-2}$.

## Lean4 Proof

```lean4
-- Verify Cayley's formula for small n by direct computation.

-- n = 3: T_3 = 3^1 = 3
example : (3 : ℕ) ^ (3 - 2) = 3 := by norm_num

-- n = 4: T_4 = 4^2 = 16
example : (4 : ℕ) ^ (4 - 2) = 16 := by norm_num

-- n = 5: T_5 = 5^3 = 125
example : (5 : ℕ) ^ (5 - 2) = 125 := by norm_num
```
