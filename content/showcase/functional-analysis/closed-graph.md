+++
title = "Closed Graph Theorem"
description = "A linear map between Banach spaces with closed graph is automatically continuous"
weight = 30
tags = ["lean4-proof", "functional-analysis", "visualization"]
latex = "\\mathrm{graph}(T) \\text{ closed} \\Rightarrow T \\text{ continuous}"
prerequisites = ["open-mapping-theorem"]
lean4_status = "complete"
+++

## Statement

Let $E$ and $F$ be Banach spaces and $T : E \to F$ a linear map. If the **graph** of $T$,

$$\mathrm{graph}(T) = \{(x, Tx) : x \in E\} \subseteq E \times F,$$

is a **closed** subset of $E \times F$, then $T$ is **bounded** (continuous).

Closedness of the graph means: whenever $x_n \to x$ in $E$ and $Tx_n \to y$ in $F$, it follows that $y = Tx$.

## Visualization

Trace the sequence criterion for a $2 \times 2$ matrix operator $T : \mathbb{R}^2 \to \mathbb{R}^2$, $Tv = Av$:

```
Sequence: x_n → x in ℝ²,   T(x_n) → y in ℝ²

x_n = (1 + 1/n, 2 + 1/n)  →  x = (1, 2)

A = [[3, 0], [0, 1]]
T(x_n) = (3 + 3/n, 2 + 1/n)  →  (3, 2) = T(1, 2) = y   ✓

graph(T) closed: (x_n, T(x_n)) → (x, y) and y = Tx ✓
```

| $n$ | $x_n$ | $Tx_n$ | limit |
|---|---|---|---|
| 1 | $(2, 3)$ | $(6, 3)$ | — |
| 2 | $(1.5, 2.5)$ | $(4.5, 2.5)$ | — |
| 5 | $(1.2, 2.2)$ | $(3.6, 2.2)$ | — |
| $\infty$ | $(1, 2)$ | $(3, 2)$ | $Tx$ |

The limit point $(1,2), (3,2)$ lies on the graph: $T(1,2) = (3,2)$. Graph is closed, so $T$ is continuous.

**Contrast:** $T(x) = 1/x$ on $(0,1)$ has a non-closed graph (the point $(0, \infty)$ is a limit not in the graph), and $T$ is not continuous at $0$.

## Proof Sketch

1. **Product Banach space.** $E \times F$ with norm $\|(x,y)\| = \max(\|x\|, \|y\|)$ is a Banach space.
2. **Graph is a subspace.** $G = \mathrm{graph}(T)$ is a linear subspace of $E \times F$.
3. **$G$ is complete.** A closed subspace of a Banach space is itself a Banach space (complete).
4. **Projection maps.** The projection $\pi_1 : G \to E$, $\pi_1(x, Tx) = x$, is a bijective bounded linear map.
5. **Open Mapping Theorem.** $\pi_1$ is bijective between Banach spaces, so by the [[Open Mapping Theorem (Banach)|Open Mapping Theorem]], its inverse $\pi_1^{-1} : E \to G$ is bounded.
6. **Continuity of $T$.** Write $T = \pi_2 \circ \pi_1^{-1}$; both factors are bounded.

## Connections

- [[Open Mapping Theorem (Banach)]] — the Closed Graph Theorem is deduced directly from it via the projection argument.
- [[Hahn–Banach Theorem]] — together with the Open Mapping Theorem and Uniform Boundedness, these four results form the four pillars of functional analysis.
- [[Bolzano–Weierstrass Theorem]] — the sequential characterisation of closed sets used here is the infinite-dimensional cousin of Bolzano–Weierstrass convergence.
- [[Spectral Theorem]] — the Closed Graph Theorem ensures that many unbounded operators (e.g., differential operators with closed graph) still admit spectral decompositions.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Normed.Operator.Banach

/-- **Closed Graph Theorem**: a linear map with closed graph between Banach spaces is continuous.
    Direct alias of `LinearMap.continuous_of_isClosed_graph` in Mathlib. -/
theorem closed_graph_theorem {E F : Type*}
    [NormedAddCommGroup E] [NormedSpace ℝ E] [CompleteSpace E]
    [NormedAddCommGroup F] [NormedSpace ℝ F] [CompleteSpace F]
    (g : E →ₗ[ℝ] F) (hg : IsClosed (g.graph : Set (E × F))) :
    Continuous g :=
  g.continuous_of_isClosed_graph hg
```
