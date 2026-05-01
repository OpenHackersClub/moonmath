+++
title = "Tychonoff Theorem"
description = "An arbitrary product of compact spaces is compact in the product topology"
weight = 50
tags = ["lean4-proof", "topology", "compactness", "product"]
latex = "\\prod_{i \\in I} X_i \\text{ compact} \\iff \\forall i,\\; X_i \\text{ compact}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $\{X_i\}_{i \in I}$ be an arbitrary (possibly uncountable) family of topological spaces. The **product space** $\prod_{i \in I} X_i$, equipped with the product topology (the coarsest topology making all projections continuous), is **compact** if and only if each factor $X_i$ is compact:

$$\prod_{i \in I} X_i \text{ compact} \iff \forall i \in I,\; X_i \text{ compact.}$$

## Visualization

For two compact intervals $[0,1]$ and $[0,1]$, their product is the closed unit square — compact.

```
[0,1] × [0,1]:

1 ┌─────────────────┐
  │                 │
  │   Compact grid  │
  │   of [0,1]²     │
  │                 │
  │  Each row [0,1] │ ← compact
  │  Each col [0,1] │ ← compact
0 └─────────────────┘
  0                 1
  ↑_________________↑
     product is compact (= closed bounded square, Heine–Borel)

For infinitely many factors [0,1] × [0,1] × ··· (countably or uncountably many):
  The product topology is coarser than the norm topology.
  Tychonoff: still compact! (Alexander subbase theorem + Axiom of Choice)
```

Each horizontal slice is a copy of a compact interval; the infinite product assembles them all into one compact space. This is remarkable: no single bounded-metric characterisation works in infinite dimensions, yet the purely topological notion of compactness is preserved.

## Proof Sketch

The standard proof uses the **Alexander subbase theorem**: a topological space is compact iff every open cover by subbase elements (preimages of open sets in each factor) has a finite subcover.

Given an open cover by subbase elements $\pi_i^{-1}(U_i)$, if for every $i$ the sets $\{U : \pi_i^{-1}(U) \text{ in cover}\}$ covered $X_i$, compactness of each $X_i$ would give a finite subcover. A contrapositive argument (using the **Axiom of Choice**, e.g. via Zorn's lemma) shows the cover must already contain a finite subcover.

The proof is equivalent to the Axiom of Choice: for infinite products, neither direction is provable in ZF without AC.

## Connections

- **[[Heine–Borel Theorem]]** — the $n$-dimensional case: $[0,1]^n$ is compact as a finite product of compact intervals. Tychonoff is the infinite-dimensional extension.
- **[[Bolzano–Weierstrass Theorem]]** — in the product (weak) topology on infinite-dimensional spaces, Tychonoff's theorem recovers sequential compactness results (e.g. Banach–Alaoglu theorem for weak-* topology).
- **[[Brouwer Fixed-Point Theorem]]** — Schauder's fixed-point theorem in Banach spaces uses weak compactness (Tychonoff + Banach–Alaoglu) to generalise Brouwer to infinite dimensions.
- **[[Urysohn Lemma]]** — a product of completely regular (Tychonoff) spaces is completely regular; Urysohn-type separation passes to products.
- **[[Hausdorff Distance]]** — the hyperspace of compact subsets inherits compactness from an ambient compact space; Tychonoff underlies this transfer.
- **[[Iterated Function Systems]]** — the space of probability measures on a compact space (used in IFS analysis) is compact in the weak-* topology by Tychonoff + Riesz representation.

## Lean4 Proof

```lean4
import Mathlib.Topology.Compactness.Compact

open Set Topology

/-- **Tychonoff's theorem**: a product of compact sets is compact (Set.pi version).
    Mathlib: `isCompact_univ_pi` in `Mathlib.Topology.Compactness.Compact`. -/
theorem tychonoff {ι : Type*} {X : ι → Type*} [∀ i, TopologicalSpace (X i)]
    {s : ∀ i, Set (X i)} (h : ∀ i, IsCompact (s i)) :
    IsCompact (pi univ s) :=
  isCompact_univ_pi h

/-- **Tychonoff's theorem** (instance form): a product of compact spaces is compact. -/
example {ι : Type*} {X : ι → Type*} [∀ i, TopologicalSpace (X i)]
    [∀ i, CompactSpace (X i)] : CompactSpace (∀ i, X i) :=
  inferInstance
```
