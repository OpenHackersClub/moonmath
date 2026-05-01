+++
title = "Inclusion-Exclusion Principle"
description = "The size of a union is the alternating sum of intersection sizes"
weight = 20
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "|A \\cup B| = |A| + |B| - |A \\cap B|"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For finite sets $A_1, \ldots, A_n$:

$$\left|\bigcup_{i=1}^n A_i\right| = \sum_{k=1}^n (-1)^{k+1} \sum_{1 \le i_1 < \cdots < i_k \le n} |A_{i_1} \cap \cdots \cap A_{i_k}|$$

For the two-set case this simplifies to:

$$|A \cup B| = |A| + |B| - |A \cap B|$$

## Visualization

**Venn diagram — two overlapping sets of integers:**

```
    Set A                Set B
  ┌──────────┐        ┌──────────┐
  │  2  4  6 │  8 10  │ 10 15 20 │
  │          │◄──────►│          │
  │          │ 8, 10  │          │
  └──────────┘        └──────────┘
```

Let $A = \{2, 4, 6, 8, 10\}$ and $B = \{8, 10, 15, 20\}$.

| Quantity     | Value                          | Count |
|--------------|--------------------------------|-------|
| $A$          | $\{2, 4, 6, 8, 10\}$          | 5     |
| $B$          | $\{8, 10, 15, 20\}$           | 4     |
| $A \cap B$   | $\{8, 10\}$                   | 2     |
| $A \cup B$   | $\{2, 4, 6, 8, 10, 15, 20\}$ | 7     |

Formula check: $5 + 4 - 2 = 7$. Correct.

**Three-set example** — students taking courses:

```
         Math (M)
        ┌───────────────┐
        │   5           │
        │  ┌──────┐     │
        │  │ 3    │  2  │
        │  │  ┌───┼──┐  │
        │  │  │ 1 │  │  │
        │  └──┼───┘  │  │
        └─────┼──────┘  │
   CS (C)     │  Physics (P)
              └──────────┘
```

$|M|=12,\; |C|=10,\; |P|=8,\; |M\cap C|=4,\; |M\cap P|=3,\; |C\cap P|=2,\; |M\cap C\cap P|=1$.

$$|M \cup C \cup P| = 12 + 10 + 8 - 4 - 3 - 2 + 1 = 22$$

## Proof Sketch

The key observation: each element $x \in A_{i_1} \cap \cdots \cap A_{i_k}$ (belonging to exactly $k$ of the sets) is counted $\binom{k}{1} - \binom{k}{2} + \cdots + (-1)^{k+1}\binom{k}{k}$ times in the alternating sum. By the binomial theorem, this equals $1 - (1-1)^k = 1$. So every element is counted exactly once. $\square$

For the two-set case: draw the Venn diagram, observe elements in $A \cap B$ are double-counted in $|A| + |B|$, and subtract once.

## Connections

- [[Pigeonhole Principle]] — a crude cousin: forces existence of a collision without counting precisely
- [[Catalan Numbers]] — the ballot problem uses complementary counting, an inclusion-exclusion variant
- [[Stirling Numbers]] — the explicit formula for $S(n,k)$ involves an inclusion-exclusion sum over $k$
- [[Bell Numbers]] — $B_n = \sum_k S(n,k)$ accumulates Stirling numbers, each built on inclusion-exclusion
- [[Möbius Inversion]] — the Möbius function on a poset is the categorical generalization of inclusion-exclusion
- [[Fermat's Little Theorem]] — Euler's totient $\phi(n)$ is computed via inclusion-exclusion on prime factors

## Lean4 Proof

```lean4
import Mathlib.Data.Finset.Basic

/-- Two-set inclusion-exclusion: |A ∪ B| = |A| + |B| - |A ∩ B|.
    Proved from `Finset.card_union_add_card_inter` using `omega`. -/
theorem inc_exc_two {α : Type*} [DecidableEq α] (s t : Finset α) :
    (s ∪ t).card = s.card + t.card - (s ∩ t).card := by
  have h := Finset.card_union_add_card_inter s t
  omega

/-- The additive form is already in Mathlib as `Finset.card_union_add_card_inter`:
    |(s ∪ t)| + |s ∩ t| = |s| + |t|. -/
theorem inc_exc_additive {α : Type*} [DecidableEq α] (s t : Finset α) :
    (s ∪ t).card + (s ∩ t).card = s.card + t.card :=
  Finset.card_union_add_card_inter s t

/-- Concrete check: {1,2,3} ∪ {2,3,4} has cardinality 4. -/
theorem inc_exc_example :
    ({1, 2, 3} ∪ {2, 3, 4} : Finset ℕ).card = 4 := by decide
```
