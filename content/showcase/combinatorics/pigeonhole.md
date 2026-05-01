+++
title = "Pigeonhole Principle"
description = "If n+1 objects are placed into n boxes, some box contains at least two objects"
weight = 10
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "|f^{-1}(\\{y\\})| \\geq 2 \\;\\text{for some}\\; y \\in B \\;\\text{when}\\; |A| > |B|"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $A$ and $B$ be finite sets with $|A| > |B|$. Then for every function $f : A \to B$, there exist distinct $x, y \in A$ such that $f(x) = f(y)$.

$$|A| > |B| \implies \exists\, x \neq y \in A,\; f(x) = f(y)$$

Equivalently: no injective function exists from a larger finite set to a smaller one.

## Visualization

**13 birthdays, 12 months — a collision is unavoidable.**

People:  Alice Bob Carol Dave Eve Frank Grace Heidi Ivan Judy Karl Liam Mia
Months:  Jan  Feb  Mar  Apr  May  Jun  Jul  Aug  Sep  Oct  Nov  Dec

```
Month | People assigned
------+----------------
Jan   | Alice
Feb   | Bob
Mar   | Carol
Apr   | Dave
May   | Eve
Jun   | Frank
Jul   | Grace
Aug   | Heidi
Sep   | Ivan
Oct   | Judy
Nov   | Karl
Dec   | Liam  ← 12 slots filled, Mia has nowhere new to go
```

Mia must share a month with one of the twelve above. No arrangement escapes this — it is a consequence of arithmetic, not bad luck.

**Counting argument:** Suppose every month held at most 1 person. Then at most $12 \times 1 = 12$ people could be accommodated. But we have 13 people. Contradiction. $\square$

## Proof Sketch

Assume for contradiction that $f$ is injective, i.e., $f(x) \neq f(y)$ whenever $x \neq y$. Then $f$ induces an injection from $A$ into $B$, so $|A| \leq |B|$, contradicting $|A| > |B|$. Therefore $f$ cannot be injective, and some value in $B$ is hit at least twice.

The proof is purely combinatorial: no induction is needed. The key lemma in Mathlib is `Fintype.exists_ne_map_eq_of_card_lt`.

## Connections

The Pigeonhole Principle powers a surprising range of results:

- [[Inclusion-Exclusion Principle]] — counts collisions precisely rather than just proving they exist
- [[Catalan Numbers]] — Dyck paths use a pigeonhole-style argument to establish the ballot problem
- [[Stirling Numbers]] — partition counting requires knowing when forced merges occur
- [[Bell Numbers]] — the exponential growth of $B_n$ implies pigeonhole collisions among partitions
- [[Fermat's Little Theorem]] — the necklace-counting proof implicitly uses pigeonhole via residues
- [[Binomial Theorem]] — multinomial counting relies on distinguishing collision-free arrangements

## Lean4 Proof

```lean4
import Mathlib.Combinatorics.Pigeonhole

/-- The Pigeonhole Principle: any function from a larger finite type to
    a smaller one must identify two distinct inputs.
    This is `Fintype.exists_ne_map_eq_of_card_lt` from Mathlib. -/
theorem pigeonhole {α β : Type*} [Fintype α] [Fintype β]
    (h : Fintype.card β < Fintype.card α) (f : α → β) :
    ∃ x y : α, x ≠ y ∧ f x = f y :=
  Fintype.exists_ne_map_eq_of_card_lt f h

/-- Concrete instance: 13 elements → 12 elements forces a collision.
    `Fin 13` has cardinality 13, `Fin 12` has cardinality 12. -/
theorem pigeonhole_13_12 (f : Fin 13 → Fin 12) :
    ∃ x y : Fin 13, x ≠ y ∧ f x = f y :=
  Fintype.exists_ne_map_eq_of_card_lt f (by decide)
```
