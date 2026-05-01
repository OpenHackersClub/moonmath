+++
title = "Newton's Identities"
description = "Power sums and elementary symmetric polynomials satisfy a mutual recursion that lets each determine the other."
weight = 70
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "p_k = \\sum_{i=1}^{k-1} (-1)^{i-1} e_i p_{k-i} + (-1)^{k-1} k e_k"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $x_1, \ldots, x_n$ be indeterminates over a commutative ring $R$. Define the **elementary symmetric polynomials** $e_k = \sum_{i_1 < \cdots < i_k} x_{i_1} \cdots x_{i_k}$ and the **power sums** $p_k = \sum_{i=1}^n x_i^k$.

Newton's identities state: for $1 \le k \le n$,

$$p_k - e_1 p_{k-1} + e_2 p_{k-2} - \cdots + (-1)^{k-1} e_{k-1} p_1 + (-1)^k k e_k = 0.$$

Equivalently (for $k \le n$):

$$p_k = \sum_{i=1}^{k-1} (-1)^{i-1} e_i p_{k-i} + (-1)^{k-1} k e_k.$$

## Visualization

Roots $\{1, 2, 3\}$ — compute $e_k$ and $p_k$ directly, then verify the identity for $k = 3$:

| Quantity | Formula | Value |
|---------|---------|-------|
| $e_1$ | $1+2+3$ | $6$ |
| $e_2$ | $1\cdot2 + 1\cdot3 + 2\cdot3$ | $11$ |
| $e_3$ | $1\cdot2\cdot3$ | $6$ |
| $p_1$ | $1^1+2^1+3^1$ | $6$ |
| $p_2$ | $1^2+2^2+3^2$ | $14$ |
| $p_3$ | $1^3+2^3+3^3$ | $36$ |

Check $k = 3$: $p_3 - e_1 p_2 + e_2 p_1 - 3e_3 = 36 - 6\cdot14 + 11\cdot6 - 3\cdot6 = 36 - 84 + 66 - 18 = 0$. Confirmed.

## Proof Sketch

1. **Generating function.** The identity follows from comparing coefficients in the logarithmic derivative of $\prod_i (1 - x_i t)$, which simultaneously encodes both $e_k$ (via expansion) and $p_k$ (via its derivative).
2. **Combinatorial proof (Zeilberger).** Define a weight function on pairs $(A, j)$ where $A \subseteq [n]$ and $j \in [n]$, and build an involution on the set of such pairs of weight $k$. The involution shows the alternating sum of weighted counts is zero — this zero-sum is exactly Newton's identity.
3. **Recursive recovery.** The identity lets you recover $e_k$ from $p_1, \ldots, p_k$ and vice versa, establishing that both families generate the same ring of symmetric polynomials.

## Connections

Newton's identities connect to [[Vieta's Formulas]] (which express $e_k$ in terms of roots of a polynomial) and to [[Fundamental Theorem of Algebra]] (where symmetric polynomials arise as invariants under permutation of roots). The combinatorial involution technique also appears in [[Inclusion–Exclusion Principle]].

## Lean4 Proof

```lean4
-- Mathlib: Mathlib.RingTheory.MvPolynomial.Symmetric.NewtonIdentities
-- The main theorem is `MvPolynomial.mul_esymm_eq_sum` (Newton's identity in sum form)
-- and `MvPolynomial.psum_eq_mul_esymm_sub_sum` (solved for p_k).
-- We alias the Mathlib statement directly.
open MvPolynomial in
theorem newton_identities_mul_esymm (σ R : Type*) [CommRing R] [Fintype σ]
    [DecidableEq σ] (k : ℕ) :
    k * esymm σ R k =
      ∑ a ∈ Finset.antidiagonal k with a.1 ∈ Set.Ioo 0 k,
        (-1 : MvPolynomial σ R) ^ (a.fst + 1) * esymm σ R a.fst * psum σ R a.snd +
        (-1) ^ k * psum σ R k :=
  mul_esymm_eq_sum σ R k
```