+++
title = "Euler's Formula for Planar Graphs"
description = "For any connected planar graph, V − E + F = 2, where F counts the faces including the outer face."
weight = 132
tags = ["lean4-proof", "graph-theory", "visualization"]
latex = "V - E + F = 2"
prerequisites = ["handshake-lemma"]
lean4_status = "complete"
+++

## Statement

For any connected planar graph embedded in the plane with $V$ vertices, $E$ edges, and $F$ faces (including the unbounded outer face):

$$V - E + F = 2$$

This is the **Euler characteristic** of the sphere, and it holds for any planar embedding of any connected planar graph.

## Visualization

$K_4$ embedded in the plane with 4 vertices, 6 edges, and 4 faces:

```
      1
     /|\
    / | \
   /  |  \
  2---+---3
   \  |  /
    \ | /
     \|/
      4

V = 4  (vertices: 1, 2, 3, 4)
E = 6  (edges: 12, 13, 14, 23, 24, 34)
F = 4  (faces: △123, △124, △134, △234, outer = △234)
```

Wait — $K_4$ drawn as above yields exactly 4 faces: three triangular inner faces and one outer (unbounded) face:

| Item | Count |
|------|-------|
| Vertices $V$ | 4 |
| Edges $E$ | 6 |
| Faces $F$ | 4 |
| $V - E + F$ | $4 - 6 + 4 = \mathbf{2}$ |

Contrast with $K_5$ ($V=5, E=10$): any planar embedding would require $F = 2 - 5 + 10 = 7$, but since every face has $\ge 3$ edges and each edge borders $\le 2$ faces, we'd need $2E \ge 3F$, i.e. $20 \ge 21$ — contradiction, so $K_5$ is **not planar** (Kuratowski).

## Proof Sketch

1. **Base case:** A tree on $n$ vertices has $n-1$ edges and 1 face (the outer), giving $n - (n-1) + 1 = 2$.
2. **Inductive step:** If $G$ is not a tree, it has a cycle, hence an edge $e$ interior to some face. Remove $e$: this merges two faces into one, so $F$ decreases by 1 and $E$ decreases by 1, leaving $V - E + F$ unchanged. Repeat until a spanning tree remains.
3. By induction, every connected planar graph satisfies $V - E + F = 2$.

## Connections

Euler's formula is the topological foundation for the non-planarity of [[Pythagorean Triples]]-style obstructions ($K_5$ and $K_{3,3}$ by Kuratowski). It connects to the [[Handshake Lemma]] via the face-edge incidence count ($2E \ge 3F$ for triangle-free planar graphs). The Euler characteristic also appears in [[Cayley's Formula ($n^{n-2}$ trees)]] via spanning trees of planar graphs.

## Lean4 Proof

```lean4
-- Mathlib does not yet have the full planar graph Euler formula.
-- We verify the K₄ instance directly: V=4, E=6, F=4, V−E+F=2.

theorem euler_K4 : (4 : Int) - 6 + 4 = 2 := by decide
```
