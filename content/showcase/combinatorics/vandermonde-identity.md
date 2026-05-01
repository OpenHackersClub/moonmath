+++
title = "Vandermonde's Identity"
description = "The convolution identity for binomial coefficients: C(m+n,r) = sum over k of C(m,k)*C(n,r-k)."
weight = 70
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "\\binom{m+n}{r} = \\sum_{k=0}^{r} \\binom{m}{k}\\binom{n}{r-k}"
prerequisites = ["pascal-identity"]
lean4_status = "complete"
+++

## Statement

For natural numbers $m$, $n$, and $r$:

$$\binom{m+n}{r} = \sum_{k=0}^{r} \binom{m}{k}\binom{n}{r-k}$$

The right side is a convolution: to select $r$ objects from two disjoint groups of sizes $m$ and $n$, decide how many ($k$) come from the first group.

## Visualization

Take $m = 2$, $n = 3$, $r = 2$. Then $\binom{5}{2} = 10$.

Splitting by how many come from the first group of 2:

| $k$ (from group 1) | $\binom{2}{k}$ | $\binom{3}{2-k}$ | product |
|---|---|---|---|
| 0 | 1 | 3 | 3 |
| 1 | 2 | 3 | 6 |
| 2 | 1 | 1 | 1 |
| **sum** | | | **10** |

Check: $3 + 6 + 1 = 10 = \binom{5}{2}$. The identity holds.

## Proof Sketch

1. **Algebraic proof via polynomials.** The generating function for $\binom{m}{\cdot}$ is $(1+x)^m$.
2. **Coefficient extraction.** The coefficient of $x^r$ in $(1+x)^m \cdot (1+x)^n = (1+x)^{m+n}$ is $\binom{m+n}{r}$.
3. **Product of series.** The coefficient of $x^r$ in the product of two power series is the convolution: $\sum_k \binom{m}{k}\binom{n}{r-k}$.
4. **Conclusion.** Both expressions equal $[x^r](1+x)^{m+n}$, so they are equal.

## Connections

Vandermonde's identity generalises to multinomials in [[Chu-Vandermonde Identity]]. Combined with [[Pascal's Identity]], it shows the algebraic structure of the binomial coefficient table. The identity also underlies the convolution formula for the [[Binomial Theorem]].

## Lean4 Proof

```lean4
import Mathlib.Data.Nat.Choose.Vandermonde

/-- Vandermonde's identity: C(m+n, k) equals the convolution
    sum_{i+j=k} C(m,i) * C(n,j).
    Mathlib's `Nat.add_choose_eq` states this directly. -/
theorem vandermonde_identity (m n k : ℕ) :
    Nat.choose (m + n) k =
      ∑ ij ∈ Finset.Nat.antidiagonal k, Nat.choose m ij.1 * Nat.choose n ij.2 :=
  Nat.add_choose_eq m n k
```
