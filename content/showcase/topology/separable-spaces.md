+++
title = "Separable Spaces"
description = "A topological space is separable if it contains a countable dense subset"
weight = 150
tags = ["lean4-proof", "topology", "visualization"]
latex = "\\exists\\, D \\subseteq X,\\; |D| \\le \\aleph_0 \\text{ and } \\overline{D} = X"
prerequisites = []
lean4_status = "complete"
+++

## Statement

A topological space $X$ is **separable** if it contains a countable dense subset $D \subseteq X$:

$$\exists\, D \subseteq X,\quad |D| \le \aleph_0 \quad \text{and} \quad \overline{D} = X.$$

The real line $\mathbb{R}$ is separable: $\mathbb{Q}$ is countable and dense in $\mathbb{R}$ (between any two reals lies a rational). By contrast, an uncountable discrete space (every singleton open) is **not** separable: any dense set must contain every point.

## Visualization

Dense rational approximation to arbitrary reals:

```
Real line ℝ:
  ...─────[0]─────[0.3]──[0.33]─[0.333]──···──[1/3]──[0.5]─────[1]─────...
                   │      │       │              │
                  3/10  33/100  333/1000       exact  ← rationals approaching 1/3

Dense subset D = ℚ:
  Every open interval (a, b) ⊆ ℝ contains a rational:
  a < p/q < b  for some integers p, q  (Archimedean property)

                    a         p/q        b
  ──────────────────(──────────•──────────)──────────────▶
                    ↑                     ↑
               any open ball     contains a rational
```

Non-separable example (co-countable topology comparison):

| Space | Dense subset | Countable? | Separable? |
|---|---|---|---|
| $\mathbb{R}$ (standard) | $\mathbb{Q}$ | yes | yes |
| $\mathbb{R}$ (discrete) | $\mathbb{R}$ itself | no | no |
| $\mathbb{R}^n$ | $\mathbb{Q}^n$ | yes | yes |
| $\ell^\infty$ (bounded sequences) | none countable | no | no |

## Proof Sketch

**$\mathbb{R}$ is separable:**

1. **Countability:** $\mathbb{Q}$ is countable — it bijects with $\mathbb{Z} \times \mathbb{Z}_{>0}$, which is countable.
2. **Density:** For any $x \in \mathbb{R}$ and $\varepsilon > 0$, we need a rational in $(x - \varepsilon, x + \varepsilon)$.
3. By the Archimedean property, pick $n \in \mathbb{N}$ with $n > 1/\varepsilon$. Then $\lfloor nx \rfloor / n$ is rational and within $1/n < \varepsilon$ of $x$.
4. Hence $\mathbb{Q}$ is dense, so $\overline{\mathbb{Q}} = \mathbb{R}$ and $\mathbb{R}$ is separable.

**General principle:** a metrizable separable space has a countable basis, hence is second-countable (the converse direction requires metric structure).

## Connections

- [[Heine–Borel Theorem]] — compact metric spaces are separable (a compact metric space has a finite $\varepsilon$-net for every $\varepsilon > 0$, yielding a countable dense set).
- [[Baire Category Theorem (Topology)]] — in a complete metric space, separability is equivalent to second-countability via `IsSeparable.secondCountableTopology`.
- [[Bolzano–Weierstrass Theorem]] — a separable metric space satisfies a sequential compactness criterion: bounded sequences have convergent subsequences (Bolzano–Weierstrass gives this for $\mathbb{R}^n$).

## Lean4 Proof

```lean4
import Mathlib.Topology.Bases
import Mathlib.Topology.Algebra.Order.Archimedean

/-- **ℝ is separable**: the rationals are countable and dense in ℝ.
    Mathlib's `Rat.denseRange_cast` gives density of ℚ → ℝ;
    `SeparableSpace` bundles the countable dense-subset witness. -/
theorem real_isSeparable : SeparableSpace ℝ := inferInstance
```
