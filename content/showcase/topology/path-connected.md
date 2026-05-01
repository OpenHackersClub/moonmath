+++
title = "Path-Connected Implies Connected"
description = "Every path-connected topological space is connected; continuous paths prevent disconnections"
weight = 60
tags = ["lean4-proof", "topology", "visualization"]
latex = "\\text{path-connected} \\Rightarrow \\text{connected}"
prerequisites = ["heine-borel"]
lean4_status = "complete"
+++

## Statement

A topological space $X$ is **path-connected** if for every two points $x, y \in X$ there exists a continuous map $\gamma : [0,1] \to X$ with $\gamma(0) = x$ and $\gamma(1) = y$. A space is **connected** if it cannot be split into two disjoint nonempty open sets.

$$\text{path-connected} \Rightarrow \text{connected.}$$

The converse fails: the **topologist's sine curve** $\{(0,y) : y \in [-1,1]\} \cup \{(x, \sin(1/x)) : x > 0\}$ is connected but not path-connected.

## Visualization

Consider the annulus $A = \{(x,y) \in \mathbb{R}^2 : 1 \le x^2 + y^2 \le 4\}$. Any two points can be joined by a path staying inside $A$:

```
         *  *
      *        *
    *    path    *
   * ---P→→→→Q-- *
    *            *
      *        *
         *  *

  P = (1, 0),  Q = (-1, 0)
  γ(t) = (cos(πt), sin(πt)) stays on the inner circle.
  Since γ is continuous and [0,1] is connected,
  γ([0,1]) is connected → A itself cannot be split.
```

If $A$ were disconnected, say $A = U \sqcup V$ with $U,V$ open and disjoint, then $\gamma^{-1}(U)$ and $\gamma^{-1}(V)$ would be a disconnection of $[0,1]$ — impossible since $[0,1]$ is connected.

## Proof Sketch

1. Suppose $X$ is path-connected and assume for contradiction $X = U \sqcup V$ with $U, V$ open, nonempty, and disjoint.
2. Pick $x \in U$ and $y \in V$. By path-connectedness there exists $\gamma : [0,1] \to X$ continuous with $\gamma(0) = x$, $\gamma(1) = y$.
3. Then $[0,1] = \gamma^{-1}(U) \sqcup \gamma^{-1}(V)$ is a partition into two disjoint nonempty open sets (preimages of opens under a continuous map are open).
4. This contradicts the connectedness of $[0,1]$ (a fact proven from the intermediate value theorem / order-completeness of $\mathbb{R}$).
5. Therefore $X$ is connected.

## Connections

The path-connectedness implication underpins many classical results:

- **[[Intermediate Value Theorem]]** — the key ingredient: $[0,1]$ is connected because $\mathbb{R}$ satisfies the least upper bound property; the IVT is precisely the statement that continuous images of connected sets are connected.
- **[[Heine–Borel Theorem]]** — closed bounded subsets of $\mathbb{R}^n$ are compact; combined with path-connectedness, this lets us conclude that such sets (like the annulus) are both compact and connected.
- **[[Brouwer Fixed-Point Theorem]]** — the closed unit ball is path-connected (straight-line paths stay inside) and hence connected, a prerequisite for topological degree arguments.
- **[[Urysohn's Lemma]]** — in a normal space, connected and path-connected sets can be separated by continuous real-valued functions, tying separation axioms to path structure.

## Lean4 Proof

```lean4
import Mathlib.Topology.Connected.PathConnected

/-- Path-connected spaces are connected (Mathlib instance). -/
theorem path_connected_implies_connected
    (X : Type*) [TopologicalSpace X] [PathConnectedSpace X] : ConnectedSpace X :=
  inferInstance
```
