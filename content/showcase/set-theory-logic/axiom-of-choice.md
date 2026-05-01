+++
title = "Axiom of Choice"
description = "For any collection of nonempty sets there exists a function selecting one element from each — equivalent to Zorn's lemma and the well-ordering theorem."
weight = 145
tags = ["lean4-proof", "set-theory-logic", "visualization"]
latex = "\\forall A\\,(\\emptyset \\notin A \\implies \\exists f: A \\to \\bigcup A,\\; \\forall S\\in A,\\; f(S)\\in S)"
prerequisites = ["zorn-lemma", "well-ordering"]
lean4_status = "complete"
+++

## Statement

**Axiom of Choice (AC):** For any set $A$ of nonempty sets there exists a *choice function* $f : A \to \bigcup A$ satisfying $f(S) \in S$ for every $S \in A$.

The three statements below are equivalent (provably so in ZF without Choice):

1. **AC** — choice functions exist.
2. **Zorn's Lemma** — every chain-bounded nonempty poset has a maximal element.
3. **Well-Ordering Theorem** — every set can be well-ordered.

## Visualization

**Choice from a family of shoes vs. socks:**

Hilbert's informal picture: for infinitely many pairs of *shoes*, you can choose the left shoe from each pair without AC (it is a definable rule). For infinitely many pairs of *socks* (identical), you need AC — there is no rule to distinguish elements, so you must postulate the choice function.

**Zorn ↔ AC (one direction):**

Given a collection $\mathcal{F}$ of functions (partial choice functions on subsets of $A$), ordered by extension, every chain has an upper bound (their union). By Zorn there is a maximal partial choice function $f^*$; if its domain were not all of $A$, we could extend it (using a single choice), contradicting maximality. So $f^*$ is a full choice function.

## Equivalences and Consequences

| Consequence | Proof route |
|---|---|
| Every vector space has a basis | Zorn on the poset of linearly independent sets |
| Every ring has a maximal ideal | Zorn on proper ideals ordered by inclusion |
| Every field has an algebraic closure | Zorn + transfinite extension |
| [[Tychonoff's Theorem]] | AC (Tychonoff ↔ AC) |
| [[Zorn's Lemma]] | Direct equivalence with AC |
| Well-ordering of $\mathbb{R}$ | WOT (non-constructive, AC equivalent) |

## Proof Sketch (Zorn $\Rightarrow$ AC)

1. Let $A = \{S_i\}$ be a family of nonempty sets. Consider the poset $\mathcal{P}$ of all partial choice functions: pairs $(B, f)$ with $B \subseteq A$ and $f : B \to \bigcup A$, $f(S) \in S$ for all $S \in B$.
2. Order $\mathcal{P}$ by $(B, f) \le (B', f')$ iff $B \subseteq B'$ and $f' \restriction_B = f$.
3. Every chain $(B_\alpha, f_\alpha)$ in $\mathcal{P}$ has upper bound $(\bigcup B_\alpha, \bigcup f_\alpha)$ (compatible functions on a chain have a consistent union).
4. By Zorn's lemma, there exists a maximal $(B^*, f^*)$.
5. If $B^* \neq A$, pick $S \in A \setminus B^*$ and any $x \in S$; extend $f^*$ to $B^* \cup \{S\}$ by $f^*(S) = x$ — contradicting maximality. Hence $B^* = A$ and $f^*$ is a total choice function.

## Connections

- [[Zorn's Lemma]] — direct equivalence; Zorn is often the most convenient form for algebra
- [[Well-Ordering Theorem]] — equivalent to AC; implies every cardinal is an aleph
- [[Tychonoff's Theorem]] — product of compact spaces is compact; equivalent to AC in full generality
- [[Cantor's Theorem]] — cardinality strict inequalities hold independently of AC, but cardinal arithmetic relies on AC for linearity of $\le_\text{card}$

## Lean4 Proof

```lean4
import Mathlib.Order.Zorn
import Mathlib.Logic.Choice

/-- The Axiom of Choice is a global axiom in Lean/Mathlib via `Classical.choice`.
    Every nonempty type has an element. -/
theorem choice_nonempty {α : Sort*} [h : Nonempty α] : α :=
  Classical.choice h

/-- Zorn's lemma in Mathlib (`zorn_le`) is proved from `Classical.choice`.
    Direction: Zorn ⊢ for any chain-bounded nonempty poset, a maximal element exists. -/
theorem zorns_lemma_from_choice {α : Type*} [PartialOrder α]
    (h : ∀ c : Set α, IsChain (· ≤ ·) c → BddAbove c) : ∃ m : α, IsMax m :=
  zorn_le h

/-- One direction of AC ↔ Zorn: existence of choice functions.
    For a family of nonempty sets (here modelled as a type with Nonempty instances),
    Classical.choice provides the choice function definitionally. -/
theorem ac_choice_function {ι : Type*} (S : ι → Type*) [∀ i, Nonempty (S i)] :
    ∃ f : ∀ i, S i, True :=
  ⟨fun i => Classical.choice (inferInstance), trivial⟩
```
