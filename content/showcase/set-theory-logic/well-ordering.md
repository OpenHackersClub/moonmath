+++
title = "Well-Ordering Theorem"
description = "Every set can be well-ordered: given AC, every set admits a relation under which every nonempty subset has a minimum."
weight = 139
tags = ["lean4-proof", "set-theory-logic", "visualization"]
latex = "\\forall X,\\ \\exists\\, \\le_X,\\ (X, \\le_X) \\text{ is well-ordered}"
prerequisites = ["zorn-lemma"]
lean4_status = "complete"
+++

## Statement

The **Well-Ordering Theorem** (equivalent to the Axiom of Choice): every set $X$ admits a total order $\le_X$ under which every nonempty subset $S \subseteq X$ has a least element:

$$\forall X,\; \exists\, \le_X,\; \forall S \subseteq X,\; S \neq \emptyset \Rightarrow \exists\, m \in S,\; \forall s \in S,\; m \le_X s$$

## Visualization

$\mathbb{N}$ is well-ordered by the standard $\le$: every nonempty subset has a minimum.

```
ℕ with standard order:
0 < 1 < 2 < 3 < 4 < 5 < …

Subset {3, 7, 2, 11}: minimum is 2.
Subset {5, 5, 8}:     minimum is 5.
Subset {0}:           minimum is 0.

Compare ℝ with standard order — NOT well-ordered:
  Subset (0, 1) = {x | 0 < x < 1} has no minimum element.
  For any candidate m > 0, the element m/2 is smaller and still in (0,1).

With AC, we can well-order ℝ — but no explicit well-order is constructible.
```

$\mathbb{N}$ is the canonical example: any decreasing sequence $n_0 > n_1 > n_2 > \cdots$ in $\mathbb{N}$ must terminate.

## Proof Sketch

The Well-Ordering Theorem follows from Zorn's Lemma (and conversely):

1. Consider the poset of all well-orderings $(S, \le_S)$ where $S \subseteq X$, ordered by end-extension: $(S, \le_S) \preceq (T, \le_T)$ if $S \subseteq T$, $\le_T$ extends $\le_S$, and every element of $T \setminus S$ is above all of $S$.
2. Every chain in this poset has an upper bound (the union of all well-orderings in the chain).
3. By Zorn's Lemma, there is a maximal well-ordering $(M, \le_M)$.
4. If $M \neq X$, pick any $x \in X \setminus M$ and extend $\le_M$ by placing $x$ at the top — contradicting maximality. So $M = X$.

## Connections

Well-ordering is equivalent to both [[Zorn's Lemma]] and the Axiom of Choice. The ordinals provide canonical representatives of well-ordered sets: every well-order is isomorphic to a unique ordinal. The [[Cantor's Theorem|diagonal argument]] uses well-ordering implicitly when constructing new cardinals.

## Lean4 Proof

```lean4
import Mathlib.SetTheory.Cardinal.Order

/-- Every type admits a well-ordering relation.
    Mathlib: `WellOrderingRel.isWellOrder` — the canonical instance built via AC. -/
example (α : Type*) : IsWellOrder α WellOrderingRel :=
  WellOrderingRel.isWellOrder
```
