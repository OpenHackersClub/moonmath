+++
title = "Heine–Borel Theorem"
description = "In ℝⁿ, a subset is compact if and only if it is closed and bounded"
weight = 10
tags = ["lean4-proof", "topology", "compactness", "visualization"]
latex = "K \\subseteq \\mathbb{R}^n \\;\\text{compact} \\iff K \\;\\text{closed and bounded}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

A subset $K \subseteq \mathbb{R}^n$ is **compact** if and only if it is **closed** and **bounded**:

$$K \subseteq \mathbb{R}^n \text{ compact} \iff K \text{ closed and bounded.}$$

Compactness means every open cover admits a finite subcover. Closed means the set contains all its limit points. Bounded means it fits inside some ball $B_r(0)$.

## Visualization

Consider $\mathbb{R}^1$. Below, a closed bounded set $[1,3]$ versus the open unbounded ray $(0,\infty)$.

```
Number line:
──0────[1═══3]────────5──────────────▶
       ↑     ↑
    closed  closed
    bounded bounded
    compact!

──0════(0═══════════════════════════▶
        ↑
   open, unbounded, not compact
   The cover {(−1, n) : n ∈ ℕ} has no finite subcover.
```

The set $[1,3]$ can be covered by finitely many open intervals no matter how you choose them — any open cover must include intervals that together blanket $[1,3]$, and compactness guarantees a finite selection suffices.

For an open set $(1,3)$: the cover $\{(1+\tfrac{1}{n}, 3) : n \ge 1\}$ is an open cover with no finite subcover (every finite sub-collection misses points near $1$).

## Proof Sketch

**Compact $\Rightarrow$ closed and bounded.** A compact subset of a Hausdorff space is closed. If $K$ were unbounded, the open cover $\{B_n(0) : n \in \mathbb{N}\}$ would have no finite subcover — contradiction.

**Closed and bounded $\Rightarrow$ compact.** A closed bounded set sits inside some $[-R,R]^n$. The box $[-R,R]^n$ is a finite product of compact intervals, hence compact by [[Tychonoff's Theorem]]. A closed subset of a compact set is compact.

## Connections

Heine–Borel is the foundation for a cascade of classical theorems:

- **[[Bolzano–Weierstrass Theorem]]** — a bounded sequence in $\mathbb{R}^n$ has a convergent subsequence; the compact enclosing box does the work.
- **[[Brouwer Fixed-Point Theorem]]** — the closed unit ball is compact (closed and bounded), enabling degree-theory and homological arguments.
- **[[Tychonoff's Theorem]]** — the infinite-dimensional generalisation: arbitrary products of compact spaces are compact. Heine–Borel is the $n=1$ finite-dimensional case.
- **[[Hausdorff Distance]]** — the space of non-empty compact subsets of $\mathbb{R}^n$ is itself complete and compact under the Hausdorff metric, making IFS attractors well-defined.
- **[[Urysohn's Lemma]]** — compact Hausdorff spaces are normal, so Urysohn's construction applies.
- **[[Iterated Function Systems]]** — the Hutchinson operator acts on $\mathcal{K}^*(\mathbb{R}^n)$; Heine–Borel guarantees that iterates stay in a compact ambient space.

## Lean4 Proof

```lean4
import Mathlib.Topology.MetricSpace.Bounded
import Mathlib.Analysis.InnerProductSpace.EuclideanDist

open Bornology

/-- **Heine–Borel theorem** for Euclidean space ℝⁿ.
    Mathlib: `isCompact_iff_isClosed_bounded` in `ProperSpace` instances. -/
theorem heine_borel {n : ℕ} {s : Set (EuclideanSpace ℝ (Fin n))} :
    IsCompact s ↔ IsClosed s ∧ IsBounded s :=
  isCompact_iff_isClosed_bounded
```
