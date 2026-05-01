+++
title = "Hausdorff Distance"
description = "A metric on non-empty compact sets, foundation for the IFS attractor theorem"
weight = 25
tags = ["lean4-proof", "fractal-geometry", "metric", "visualization"]
latex = "d_H(A, B) = \\max\\left(\\sup_{a \\in A} d(a, B),\\ \\sup_{b \\in B} d(b, A)\\right)"
prerequisites = []
lean4_status = "sorry"
+++

## Definition

Let $(X, d)$ be a metric space and $A, B \subseteq X$ non-empty subsets. The **Hausdorff distance** between $A$ and $B$ is

$$d_H(A, B) = \max\left(\sup_{a \in A} \inf_{b \in B} d(a, b),\ \sup_{b \in B} \inf_{a \in A} d(a, b)\right).$$

An equivalent characterisation uses $\varepsilon$-thickenings $A_\varepsilon = \{x \in X : \inf_{a \in A} d(x, a) \leq \varepsilon\}$:

$$d_H(A, B) = \inf\{\varepsilon \geq 0 : A \subseteq B_\varepsilon \text{ and } B \subseteq A_\varepsilon\}.$$

Intuitively, $d_H(A, B) \leq \varepsilon$ means every point of $A$ has a partner in $B$ within $\varepsilon$, and vice versa.

## Examples

**Two points.** $d_H(\{p\}, \{q\}) = d(p, q)$ — the Hausdorff distance reduces to the underlying metric on singletons.

**Concentric circles.** Let $C_r$ and $C_R$ be circles of radii $r < R$ centred at the origin in $\mathbb{R}^2$. Then $d_H(C_r, C_R) = R - r$.

**A point and a set.** $d_H(\{p\}, A) = \sup_{a \in A} d(p, a)$ — the diameter of $A$ as seen from $p$.

**Nested squares.** A square $S \subset [0,1]^2$ of side $1/2$ centred in the unit square has $d_H(S, [0,1]^2) = \tfrac{\sqrt{2}}{4}$ — the distance from the centre of $S$ to a corner of $[0,1]^2$ minus the distance to a corner of $S$.

**Cantor approximations.** Let $C_n$ be the $n$-th iterate of the middle-thirds Cantor construction (so $C_0 = [0,1]$, $C_1 = [0, 1/3] \cup [2/3, 1]$, …). Then $d_H(C_n, C) \to 0$ as $n \to \infty$, where $C$ is the Cantor set itself. This is the Hausdorff-metric statement that the [[Iterated Function Systems|IFS]] iteration converges.

## The Space of Compact Sets

Let $\mathcal{K}^*(X)$ denote the set of non-empty compact subsets of $X$. The fundamental result is:

**Theorem (Hausdorff completeness).** *If $(X, d)$ is complete, then $(\mathcal{K}^*(X), d_H)$ is a complete metric space. If $X$ is compact, so is $\mathcal{K}^*(X)$.*

This is the platform on which the [[Iterated Function Systems]] attractor theorem is built: the Hutchinson operator $F(K) = \bigcup_i f_i(K)$ acts on $\mathcal{K}^*(X)$, and the contraction ratios of the $f_i$ transfer to a contraction ratio for $F$ in the Hausdorff metric.

## Why Hausdorff Distance for Fractals

Pointwise convergence is too weak for fractal sets — the Cantor set has Lebesgue measure zero, so any "$L^p$" sense of convergence collapses. The Hausdorff metric instead measures *shape* convergence: $K_n \to K$ in $d_H$ iff every point of $K$ is approximated by points of $K_n$ and vice versa, uniformly. This is exactly the notion needed to make sense of "the attractor is the limit of $F^n(K_0)$ for any starting compact $K_0$".

## Connections

The Hausdorff metric makes the [[Iterated Function Systems]] attractor a Banach fixed point. Beyond fractals, $d_H$ is the standard tool for shape comparison in image processing, computational geometry (e.g. mesh similarity), and optimal transport relaxations.

## Lean4 Proof

```lean4
import Mathlib.Topology.MetricSpace.HausdorffDistance
import Mathlib.Topology.MetricSpace.Closeds

open Metric Set TopologicalSpace

/-- Hausdorff distance between two non-empty bounded sets in a metric space. -/
noncomputable def hausdorffDist {X : Type*} [PseudoMetricSpace X] (A B : Set X) : ℝ :=
  Metric.hausdorffDist A B

/-- Singleton case: d_H({p}, {q}) = d(p, q). -/
example {X : Type*} [MetricSpace X] (p q : X) :
    Metric.hausdorffDist ({p} : Set X) {q} = dist p q := by
  simp [hausdorffDist_singleton]

/-- The space of non-empty compact subsets of a complete metric space is complete
    under the Hausdorff distance. (Mathlib: `NonemptyCompacts.completeSpace`.) -/
example {X : Type*} [MetricSpace X] [CompleteSpace X] :
    CompleteSpace (NonemptyCompacts X) := inferInstance
```
