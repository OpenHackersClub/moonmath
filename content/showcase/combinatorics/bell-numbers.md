+++
title = "Bell Numbers"
description = "The total number of partitions of a set of n elements"
weight = 50
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "B_n = \\sum_{k=0}^{n} S(n, k)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

The $n$-th Bell number $B_n$ counts the total number of partitions of a set of $n$ elements. It satisfies:

$$B_n = \sum_{k=0}^{n} S(n, k)$$

where $S(n, k)$ are the [[Stirling Numbers]] of the second kind. Equivalently, by the Bell triangle recursion:

$$B_{n+1} = \sum_{k=0}^{n} \binom{n}{k} B_k$$

## Visualization

**Values $B_0$ through $B_6$:**

| $n$   | 0 | 1 | 2 | 3 | 4  | 5   | 6   |
|-------|---|---|---|---|----|-----|-----|
| $B_n$ | 1 | 1 | 2 | 5 | 15 | 52  | 203 |

**All partitions of $\{1, 2, 3\}$ — verifying $B_3 = 5$:**

```
1 block (k=1):
  { {1, 2, 3} }

2 blocks (k=2):
  { {1}, {2, 3} }
  { {2}, {1, 3} }
  { {3}, {1, 2} }

3 blocks (k=3):
  { {1}, {2}, {3} }
```

Five partitions total. $B_3 = S(3,1) + S(3,2) + S(3,3) = 1 + 3 + 1 = 5$.

**Bell triangle** (each row gives the next Bell number and recursion weights):

```
Row 0:  1
Row 1:  1   2
Row 2:  2   3   5
Row 3:  5   7  10  15
Row 4: 15  20  27  37  52
```

Each row starts with the last element of the previous row; each subsequent element is the sum of the element to its left and the element directly above it. The first element of each row is $B_n$.

**Recursion unrolled for $B_4 = 15$:**

$$B_4 = \binom{3}{0}B_0 + \binom{3}{1}B_1 + \binom{3}{2}B_2 + \binom{3}{3}B_3 = 1\cdot1 + 3\cdot1 + 3\cdot2 + 1\cdot5 = 15$$

## Proof Sketch

The recursion $B_{n+1} = \sum_{k=0}^n \binom{n}{k} B_k$ follows from conditioning on the block containing element $n+1$: if that block has size $j+1$, choose the remaining $j$ members from $\{1, \ldots, n\}$ in $\binom{n}{j}$ ways, then partition the leftover $n-j$ elements in $B_{n-j}$ ways. Summing over $j = 0, \ldots, n$ gives the formula (after re-indexing with $k = n-j$).

Mathlib's `Nat.bell` uses the Finset-indexed form `∑ i : Fin n.succ, choose n i * Nat.bell (n - i)`, proved as `Nat.bell_succ`.

## Connections

- [[Stirling Numbers]] — $B_n = \sum_k S(n,k)$; Bell numbers are horizontal sums of the Stirling triangle
- [[Inclusion-Exclusion Principle]] — each $S(n,k)$ in the sum is itself an inclusion-exclusion formula
- [[Catalan Numbers]] — both count partition-like objects; $C_n < B_n$ for $n \geq 3$ since Catalan only counts non-crossing partitions
- [[Pigeonhole Principle]] — the rapid growth of $B_n$ means that any assignment of $n$ objects to $B_{n-1}$ partition-types must repeat a type for large $n$
- [[Binomial Theorem]] — the binomial coefficients $\binom{n}{k}$ in the recursion come from choosing block members
- [[Möbius Inversion]] — the exponential generating function $\sum_n B_n x^n/n! = e^{e^x - 1}$ is proved via Möbius inversion on the partition lattice

## Lean4 Proof

```lean4
import Mathlib.Combinatorics.Enumerative.Bell

open Nat

/-- The Bell number recursion: B(n+1) = ∑ᵢ C(n,i) * B(n-i).
    This is `Nat.bell_succ` in Mathlib. -/
theorem bell_recursion (n : ℕ) :
    Nat.bell (n + 1) = ∑ i : Fin n.succ, Nat.choose n i * Nat.bell (n - i) :=
  Nat.bell_succ n

/-- Verify small Bell numbers by computation. -/
theorem bell_vals :
    Nat.bell 0 = 1 ∧ Nat.bell 1 = 1 ∧ Nat.bell 2 = 2 ∧
    Nat.bell 3 = 5 ∧ Nat.bell 4 = 15 ∧ Nat.bell 5 = 52 := by decide

/-- B(0) = 1: the empty set has exactly one partition (the empty partition). -/
theorem bell_zero : Nat.bell 0 = 1 :=
  Nat.bell_zero

/-- B(5) = 52, verifying the sixth Bell number. -/
theorem bell_five : Nat.bell 5 = 52 := by decide
```
