+++
title = "Pascal's Identity"
description = "The binomial coefficient recurrence: C(n,k) = C(n-1,k-1) + C(n-1,k), the engine of Pascal's triangle."
weight = 60
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "\\binom{n}{k} = \\binom{n-1}{k-1} + \\binom{n-1}{k}"
prerequisites = ["inclusion-exclusion"]
lean4_status = "complete"
+++

## Statement

For any natural numbers $n$ and $k$ with $k \le n$:

$$\binom{n}{k} = \binom{n-1}{k-1} + \binom{n-1}{k}$$

This recurrence, together with the boundary conditions $\binom{n}{0} = \binom{n}{n} = 1$, completely determines all binomial coefficients.

## Visualization

Pascal's triangle rows 0–6, with each interior entry the sum of the two above it:

```
Row 0:                 1
Row 1:               1   1
Row 2:             1   2   1
Row 3:           1   3   3   1
Row 4:         1   4   6   4   1
Row 5:       1   5  10  10   5   1
Row 6:     1   6  15  20  15   6   1
```

The identity says: entry $(n,k)$ = entry $(n-1,k-1)$ + entry $(n-1,k)$.

Example at row 4, column 2: $\binom{4}{2} = 6 = \binom{3}{1} + \binom{3}{2} = 3 + 3$.

Arrow diagram for $\binom{4}{2}$:
```
        3   3        <- row 3, positions 1 and 2
         \ /
          6          <- row 4, position 2
```

## Proof Sketch

1. **Combinatorial argument.** $\binom{n}{k}$ counts $k$-element subsets of $\{1,\ldots,n\}$.
2. **Split on element $n$.** Either $n$ is in the chosen subset or not.
3. **Case $n \in S$:** must choose $k-1$ elements from $\{1,\ldots,n-1\}$ — gives $\binom{n-1}{k-1}$ choices.
4. **Case $n \notin S$:** must choose $k$ elements from $\{1,\ldots,n-1\}$ — gives $\binom{n-1}{k}$ choices.
5. **Sum:** the two cases are disjoint and exhaustive, so $\binom{n}{k} = \binom{n-1}{k-1} + \binom{n-1}{k}$.

## Connections

Pascal's identity is the combinatorial core behind [[Binomial Theorem]] (the expansion of $(x+y)^n$ sums over rows of Pascal's triangle) and is the key recurrence for [[Stirling Numbers]].

## Lean4 Proof

```lean4
import Mathlib.Data.Nat.Choose.Basic

/-- Pascal's identity: C(n+1, k+1) = C(n, k) + C(n, k+1).
    Mathlib's `Nat.choose_succ_succ` states this directly. -/
theorem pascal_identity (n k : ℕ) :
    Nat.choose (n + 1) (k + 1) = Nat.choose n k + Nat.choose n (k + 1) :=
  Nat.choose_succ_succ' n k
```
