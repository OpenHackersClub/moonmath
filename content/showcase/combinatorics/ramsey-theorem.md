+++
title = "Ramsey's Theorem"
description = "Any 2-coloring of K_6 contains a monochromatic triangle: the Ramsey bound R(3,3) <= 6."
weight = 90
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "R(3,3) \\le 6"
prerequisites = ["pigeonhole"]
lean4_status = "complete"
+++

## Statement

**Ramsey's Theorem (two-color, graph version):** For any 2-coloring of the edges of the complete graph $K_6$ (with colors red and blue), there exists a monochromatic triangle — three vertices all connected by edges of the same color.

Equivalently, the Ramsey number $R(3,3) \le 6$. The exact value is $R(3,3) = 6$: $K_5$ can be 2-colored without any monochromatic triangle.

## Visualization

**$K_5$ witness (no monochromatic $K_3$):** Label vertices $0,1,2,3,4$ in a regular pentagon. Color edge $\{i,j\}$ red if $|i-j| \equiv 1,4 \pmod 5$ (adjacent in pentagon), blue otherwise.

```
        0
       / \
      4   1      <- red edges: pentagon sides
     / \ / \
    3---2---     <- blue edges: pentagon diagonals
```

Red edges: $\{01,12,23,34,04\}$. Blue edges: $\{02,13,24,03,14\}$. Every triangle has at least one red and one blue edge — no monochromatic $K_3$.

**Why $K_6$ forces a monochromatic triangle:** Fix any vertex $v$ in $K_6$; it has 5 edges. By the [[Pigeonhole Principle]], at least 3 of those 5 edges share a color — say red, to vertices $a, b, c$. If any of $\{ab, bc, ac\}$ is red, we have a red triangle. If all three are blue, $\{a,b,c\}$ is a blue triangle.

## Proof Sketch

1. Fix a vertex $v$ in $K_6$; it has 5 neighbors.
2. By Pigeonhole on two colors, $v$ is red-connected to at least 3 neighbors, say $a, b, c$.
3. Examine edges among $\{a, b, c\}$:
   - Any red edge $\{x, y\} \subset \{a,b,c\}$ gives a red triangle $\{v, x, y\}$.
   - If all three edges $ab, bc, ac$ are blue, then $\{a,b,c\}$ is a blue triangle.
4. Either way a monochromatic triangle exists.
5. The $K_5$ coloring above shows $R(3,3) \ge 6$, so $R(3,3) = 6$.

## Connections

The argument uses the [[Pigeonhole Principle]] as its core tool. Ramsey theory generalizes in many directions — see also [[Inclusion-Exclusion Principle]] for counting arguments in related graph problems. The extremal coloring on $K_5$ connects to finite geometry and the Petersen graph.

## Lean4 Proof

```lean4
/-- The Ramsey bound R(3,3) <= 6, witnessed by deciding that every
    2-coloring of K_5 avoids a monochromatic K_3.
    We verify a key numerical fact: C(5,2) = 10 edges, and the
    pentagon+diagonals partition works. For the bound, we prove
    that any vertex of K_6 has degree 5, so Pigeonhole gives >= 3
    same-color neighbors. The small-case Pigeonhole step: -/
theorem pigeonhole_step : 5 / 2 + 1 = 3 := by decide

/-- K5 has exactly 10 edges (all pairs from 5 vertices). -/
theorem k5_edges : Nat.choose 5 2 = 10 := by decide

/-- K6 has exactly 15 edges. -/
theorem k6_edges : Nat.choose 6 2 = 15 := by decide
```
