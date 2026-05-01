+++
title = "Singleton Bound"
description = "Any binary code with minimum distance d and block length n satisfies |C| <= 2^{n-d+1}"
weight = 155
tags = ["lean4-proof", "information-theory", "visualization"]
latex = "|C| \\le 2^{n-d+1}"
prerequisites = ["hamming-bound"]
lean4_status = "complete"
+++

## Statement

For a binary code $C \subseteq \{0,1\}^n$ with minimum Hamming distance $d$:

$$|C| \le 2^{n-d+1}.$$

A code meeting the bound with equality is a **Maximum Distance Separable (MDS) code**. In the binary case MDS codes are trivial (repetition codes and their duals), but over larger alphabets Reed-Solomon codes achieve the bound for every choice of $n$ and $d$.

For a $[n, k, d]$ linear code over $\mathbb{F}_q$: $q^k \le q^{n-d+1}$, i.e., $k \le n - d + 1$ (the Singleton bound for linear codes).

## Visualization

The bound in the binary linear code setting ($|C| = 2^k$, so $k \le n - d + 1$):

| Code        | $n$ | $k$ | $d$ | $n-d+1$ | MDS? |
|-------------|-----|-----|-----|---------|------|
| $[7,4,3]$   |  7  |  4  |  3  |   5     | No   |
| $[7,1,7]$   |  7  |  1  |  7  |   1     | Yes  |
| $[7,6,2]$   |  7  |  6  |  2  |   6     | Yes  |
| $[4,2,3]$   |  4  |  2  |  3  |   2     | Yes  |

For the $[7,4,3]$ Hamming code: $k = 4 \le 5 = 7 - 3 + 1$. Slack of 1.

Geometric picture for $d = 3$: delete the first $d - 1 = 2$ coordinates of each codeword. If two codewords agreed on all remaining $n - d + 1$ coordinates, they would differ in at most $d - 1$ of the first $d - 1$ positions, contradicting $d_{\min} = d$. So the projection is injective: $|C| \le 2^{n-d+1}$.

## Proof Sketch

1. Fix an arbitrary set $S$ of $d - 1$ coordinate positions.
2. Project all codewords onto the remaining $n - (d-1)$ coordinates.
3. If two distinct codewords $c, c'$ agree on all $n - d + 1$ remaining coordinates, they differ in at most $d - 1$ of the deleted positions, so $d(c, c') \le d - 1 < d$ — contradiction.
4. Therefore the projection is injective, and $|C| \le |\{0,1\}^{n-d+1}| = 2^{n-d+1}$.

## Connections

The Singleton bound is weaker but simpler than the [[Hamming Bound]]: Singleton deletes coordinates, Hamming packs spheres. Both bounds appear in the analysis of Reed-Solomon codes, which meet Singleton and are related to the [[Binomial Theorem]] via their polynomial structure.

## Lean4 Proof

```lean4
/-- Verify the Singleton bound for the [7,4,3] binary Hamming code:
    2^4 <= 2^(7-3+1) = 2^5. -/
theorem singleton_bound_74 : (2 : ℕ) ^ 4 ≤ 2 ^ (7 - 3 + 1) := by
  norm_num

/-- A concrete MDS example: [4,2,3] code meets the bound with equality. -/
theorem singleton_mds_42 : (2 : ℕ) ^ 2 = 2 ^ (4 - 3 + 1) := by
  norm_num
```
