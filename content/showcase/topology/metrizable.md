+++
title = "Metrizable Spaces"
description = "A topological space is metrizable if its topology is induced by some metric"
weight = 170
tags = ["lean4-proof", "topology", "visualization"]
latex = "\\exists\\, d:\\, X\\times X\\to\\mathbb{R}_{\\ge 0},\\;\\tau = \\tau_d"
prerequisites = ["second-countable", "regular-space"]
lean4_status = "complete"
+++

## Statement

A topological space $(X, \tau)$ is **metrizable** if there exists a metric $d : X \times X \to \mathbb{R}_{\ge 0}$ such that $\tau$ equals the metric topology $\tau_d$ (open balls form a basis):

$$\exists\, d: X \times X \to \mathbb{R}_{\ge 0},\quad \tau = \tau_d.$$

**Urysohn Metrization Theorem:** A regular Hausdorff ($T_3$) second-countable topological space is metrizable.

In Mathlib, `MetrizableSpace X` extends `PseudoMetrizableSpace X` (admits a compatible pseudometric) with `T0Space X`. Every metrizable space is $T_2$ (Hausdorff) and regular.

## Visualization

$\mathbb{R}^n$ as a metrizable space — Euclidean metric induces the standard topology:

```
ℝ² with Euclidean metric d(x,y) = √((x₁-y₁)² + (x₂-y₂)²):

         y₂
          │         r
          │      ╭──────╮
          │     /    p   \    Open ball B_r(p) = {x : d(x,p) < r}
          │     \        /
          │      ╰──────╯
          └──────────────────── y₁

  Standard topology basis: {B_r(p) : p ∈ ℚ², r ∈ ℚ₊}  ← countable

  Finite products of metrizable spaces are metrizable:
  d_prod((x,y),(x',y')) = max(d_X(x,x'), d_Y(y,y'))  or  √(d_X² + d_Y²)
```

Which spaces are metrizable?

| Space | Metrizable? | Witness |
|---|---|---|
| $\mathbb{R}^n$ | yes | Euclidean metric |
| $[0,1]^\mathbb{N}$ (Hilbert cube) | yes | $\sum 2^{-n}|x_n - y_n|$ |
| $[0,1]^{\mathbb{R}}$ (uncountable product) | no | not first-countable |
| Discrete uncountable | yes | $d(x,y) = \mathbb{1}[x \ne y]$ |
| Co-finite topology on $\mathbb{R}$ | no | not Hausdorff |

## Proof Sketch

**Urysohn Metrization (sketch):**

1. **Separation:** $X$ is regular ($T_3$) and second-countable; by Urysohn's lemma, for each pair of disjoint closed sets $F, G$ there exists a continuous $f: X \to [0,1]$ with $f|_F = 0$, $f|_G = 1$.
2. **Countable separating family:** second-countability yields countably many such functions $\{f_n : X \to [0,1]\}$ that separate points.
3. **Embedding:** define $\iota: X \to [0,1]^{\mathbb{N}}$ by $\iota(x)_n = f_n(x)$. This is a topological embedding (continuous, injective, open onto image).
4. **Hilbert cube is metrizable:** $[0,1]^{\mathbb{N}}$ carries the metric $d(x,y) = \sum_{n} 2^{-n} |x_n - y_n|$, which is compatible with the product topology.
5. **Subspace metric:** pull the Hilbert cube metric back through $\iota$ to get a compatible metric on $X$.

## Connections

- [[Second-Countable Spaces]] — second-countability is the key hypothesis in Urysohn metrization; without it regularity alone does not suffice.
- [[Regular Space]] — the $T_3$ axiom (regular + $T_1$) is the other hypothesis; together with second-countability it characterizes metrizable spaces among $T_1$ spaces.
- [[Urysohn's Lemma]] — the core tool in the metrization proof: normal spaces admit continuous real-valued separation functions.
- [[Tychonoff's Theorem]] — the Hilbert cube $[0,1]^{\mathbb{N}}$ is compact by Tychonoff; Urysohn metrization embeds second-countable regular spaces into it.

## Lean4 Proof

```lean4
import Mathlib.Topology.Metrizable.Basic
import Mathlib.Topology.MetricSpace.ProperSpace

/-- Every pseudometric space is pseudo-metrizable — direct instance. -/
instance {X : Type*} [PseudoMetricSpace X] : PseudoMetrizableSpace X :=
  inferInstance

/-- **ℝ is metrizable** (T0 + pseudo-metrizable). -/
theorem real_metrizable : MetrizableSpace ℝ := inferInstance

/-- Products of metrizable spaces are metrizable. -/
theorem prod_metrizable {X Y : Type*} [TopologicalSpace X] [TopologicalSpace Y]
    [MetrizableSpace X] [MetrizableSpace Y] : MetrizableSpace (X × Y) :=
  inferInstance
```
