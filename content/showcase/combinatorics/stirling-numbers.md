+++
title = "Stirling Numbers"
description = "Counting partitions of n elements into k non-empty subsets"
weight = 40
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "S(n,k) = k \\cdot S(n-1,k) + S(n-1,k-1)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

The **Stirling number of the second kind** $S(n, k)$ counts the number of ways to partition a set of $n$ distinct elements into exactly $k$ non-empty subsets.

They satisfy:

$$S(n, k) = k \cdot S(n-1, k) + S(n-1, k-1)$$

with boundary conditions $S(0, 0) = 1$, $S(n, 0) = 0$ for $n \geq 1$, and $S(0, k) = 0$ for $k \geq 1$.

The explicit formula is:

$$S(n, k) = \frac{1}{k!} \sum_{j=0}^{k} (-1)^{k-j} \binom{k}{j} j^n$$

## Visualization

**Triangle $S(n, k)$ for $n, k \leq 5$:**

```
n\k |  0    1    2    3    4    5
----+------------------------------
 0  |  1    0    0    0    0    0
 1  |  0    1    0    0    0    0
 2  |  0    1    1    0    0    0
 3  |  0    1    3    1    0    0
 4  |  0    1    7    6    1    0
 5  |  0    1   15   25   10    1
```

**Reading the recursion — $S(3, 2) = 3$:**

Partitions of $\{1, 2, 3\}$ into 2 non-empty subsets:

```
{ {1}, {2,3} }    { {2}, {1,3} }    { {3}, {1,2} }
```

Three ways. The recursion $S(3,2) = 2 \cdot S(2,2) + S(2,1) = 2 \cdot 1 + 1 = 3$ confirms this.

**Why the factor $k$?** To build a partition of $\{1, \ldots, n\}$ into $k$ blocks, either:
- Element $n$ forms a singleton block: arrange the rest into $k-1$ blocks — $S(n-1, k-1)$ ways.
- Element $n$ joins one of the $k$ existing blocks: choose which block — $k \cdot S(n-1, k)$ ways.

**$S(4, 2) = 7$:** Partitions of $\{1,2,3,4\}$ into 2 non-empty subsets:

```
{1}, {2,3,4}    {2}, {1,3,4}    {3}, {1,2,4}    {4}, {1,2,3}
{1,2}, {3,4}    {1,3}, {2,4}    {1,4}, {2,3}
```

Seven. Verified.

## Proof Sketch

The recursion $S(n+1, k+1) = (k+1) \cdot S(n, k+1) + S(n, k)$ follows directly from the case analysis on whether element $n+1$ is alone in its block or joined to one of $k+1$ existing blocks. The boundary cases $S(0,0)=1$ and $S(n,0)=0$ for $n>0$ encode the empty-partition convention.

Mathlib's `Nat.stirlingSecond` is defined by this exact recursion, making `stirlingSecond_succ_succ` hold by `rfl`.

## Connections

- [[Inclusion-Exclusion Principle]] — the explicit formula $S(n,k) = \frac{1}{k!}\sum_j (-1)^{k-j}\binom{k}{j}j^n$ is an inclusion-exclusion sum
- [[Bell Numbers]] — $B_n = \sum_{k=0}^n S(n,k)$; Bell numbers are row sums of the Stirling triangle
- [[Catalan Numbers]] — both count tree-like structures; $C_n$ counts binary trees, $S(n,k)$ counts set-forests
- [[Pigeonhole Principle]] — $S(n,k) = 0$ when $k > n$, reflecting that you cannot form more blocks than elements
- [[Binomial Theorem]] — the explicit formula uses binomial coefficients; the falling factorial $x^{\underline{n}} = \sum_k S(n,k)(-1)^{n-k}x^k$
- [[Möbius Inversion]] — Stirling numbers of both kinds are related by Möbius inversion on the partition lattice

## Lean4 Proof

```lean4
import Mathlib.Combinatorics.Enumerative.Stirling

open Nat

/-- The Stirling recursion S(n+1, k+1) = (k+1)·S(n,k+1) + S(n,k).
    Mathlib's definition makes this hold by `rfl`. -/
theorem stirling_recursion (n k : ℕ) :
    stirlingSecond (n + 1) (k + 1) =
      (k + 1) * stirlingSecond n (k + 1) + stirlingSecond n k :=
  rfl

/-- Verify the triangle entries by computation. -/
theorem stirling_vals :
    stirlingSecond 3 2 = 3 ∧
    stirlingSecond 4 2 = 7 ∧
    stirlingSecond 5 3 = 25 := by decide

/-- S(n, n) = 1 for all n: only one way to partition n elements into n singletons.
    This is `stirlingSecond_self` in Mathlib. -/
theorem stirling_diagonal (n : ℕ) : stirlingSecond n n = 1 :=
  stirlingSecond_self n

/-- S(n, k) = 0 when k > n: cannot have more blocks than elements.
    This is `stirlingSecond_eq_zero_of_lt` in Mathlib. -/
theorem stirling_zero_of_lt {n k : ℕ} (h : n < k) : stirlingSecond n k = 0 :=
  stirlingSecond_eq_zero_of_lt h
```
