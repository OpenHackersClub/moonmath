+++
title = "Zorn's Lemma"
description = "If every chain in a nonempty poset has an upper bound, the poset has a maximal element."
weight = 138
tags = ["lean4-proof", "set-theory-logic", "visualization"]
latex = "\\text{every chain bounded} \\implies \\exists\\text{ maximal element}"
prerequisites = ["well-ordering"]
lean4_status = "complete"
+++

## Statement

Let $(P, \le)$ be a nonempty partially ordered set. If every chain (totally ordered subset) in $P$ has an upper bound in $P$, then $P$ has at least one **maximal element** — an element $m$ such that no element strictly above $m$ exists:

$$(\forall C \subseteq P,\ C \text{ chain} \Rightarrow \exists\, u \in P,\ \forall c \in C,\ c \le u) \implies \exists\, m \in P,\ \forall p \in P,\ m \le p \Rightarrow m = p$$

## Visualization

The poset of subgroups of $\mathbb{Z}$ under inclusion, ordered by divisibility of the generator (larger subgroup = smaller index):

```
Subgroups of ℤ under ⊆:
         ℤ = ⟨1⟩
        / | \
    ⟨2⟩ ⟨3⟩ ⟨5⟩  …
    /\    |
 ⟨4⟩ ⟨6⟩ ⟨9⟩
    \  /
    ⟨12⟩
     …
     {0}

Chain example: ⟨1⟩ ⊇ ⟨2⟩ ⊇ ⟨4⟩ ⊇ ⟨8⟩ ⊇ …
Upper bound of chain: ⟨1⟩ (= ℤ itself)
Maximal element among proper subgroups: none finite one works —
but Zorn gives a maximal proper subgroup (e.g. ⟨p⟩ for prime p).
```

For the poset of all proper subgroups of $\mathbb{Z}$ (excluding $\mathbb{Z}$ itself): every chain $\{n_k \mathbb{Z}\}$ has lcm as upper bound. Zorn yields a maximal proper subgroup $p\mathbb{Z}$ for some prime $p$.

## Proof Sketch

Zorn's Lemma is equivalent to the Axiom of Choice. The standard proof proceeds:

1. Assume the hypothesis. Suppose for contradiction no maximal element exists.
2. For every chain $C$, choose an upper bound $u(C) \in P$ with $u(C)$ strictly above some element of $C$ (using Choice and the no-maximal assumption).
3. Build a transfinite sequence $p_0 < p_1 < p_2 < \cdots < p_\omega < \cdots$ by transfinite recursion using $u$.
4. This sequence, indexed by all ordinals, forms a strictly increasing chain of elements of $P$. But $P$ is a set, so the collection of distinct elements is bounded — contradiction.

## Connections

Zorn's Lemma is equivalent to the [[Well-Ordering Theorem]] and the Axiom of Choice. All three are used interchangeably in modern algebra and topology. The existence of bases for all vector spaces, maximal ideals in rings, and algebraic closures of fields each follow from Zorn. Compare with [[Tychonoff's Theorem]], which is also equivalent to Choice.

## Lean4 Proof

```lean4
import Mathlib.Order.Zorn

/-- Zorn's Lemma for partial orders: every chain-bounded poset has a maximal element.
    Mathlib: `zorn_le`. -/
theorem zorns_lemma {α : Type*} [PartialOrder α]
    (h : ∀ c : Set α, IsChain (· ≤ ·) c → BddAbove c) : ∃ m : α, IsMax m :=
  zorn_le h
```
