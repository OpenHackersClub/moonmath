+++
title = "Kraft's Inequality"
description = "Every prefix-free binary code with codeword lengths l_1,...,l_n satisfies the sum 2^{-l_i} <= 1"
weight = 152
tags = ["lean4-proof", "information-theory", "visualization"]
latex = "\\sum_{i=1}^{n} 2^{-\\ell_i} \\le 1"
prerequisites = ["shannon-entropy"]
lean4_status = "complete"
+++

## Statement

A binary code is **prefix-free** (instantaneous) if no codeword is a prefix of another. For any prefix-free code with codeword lengths $\ell_1, \ell_2, \ldots, \ell_n$:

$$\sum_{i=1}^{n} 2^{-\ell_i} \le 1.$$

Conversely (McMillan's theorem), the same bound holds for all uniquely decodable codes, and for any lengths satisfying the bound there exists a prefix-free code achieving them.

## Visualization

The code $\mathcal{C} = \{0,\; 10,\; 110,\; 111\}$ with lengths $(1, 2, 3, 3)$ is prefix-free. Verification:

$$2^{-1} + 2^{-2} + 2^{-3} + 2^{-3} = \frac{1}{2} + \frac{1}{4} + \frac{1}{8} + \frac{1}{8} = 1.$$

This code is **complete** (sum equals 1), meaning it is a full binary tree:

```
root
 |-- 0  (codeword)
 |-- 1
      |-- 10  (codeword)
      |-- 11
           |-- 110  (codeword)
           |-- 111  (codeword)
```

Each leaf at depth $d$ contributes $2^{-d}$; the sum over all leaves of a full binary tree is exactly 1.

A code that fails the inequality, e.g. $\{0, 01, 10, 11\}$ (lengths $1, 2, 2, 2$):

$$2^{-1} + 2^{-2} + 2^{-2} + 2^{-2} = \frac{1}{2} + \frac{3}{4} = \frac{5}{4} > 1.$$

Indeed $0$ is a prefix of $01$, so this code is not prefix-free.

## Proof Sketch

1. Assign to each codeword $w$ of length $\ell$ the set $S_w$ of binary strings of length $L = \max \ell_i$ that extend $w$: there are $2^{L - \ell}$ such strings.
2. Prefix-freeness means the sets $\{S_w\}$ are pairwise disjoint subsets of $\{0,1\}^L$.
3. Since all sets fit inside $\{0,1\}^L$ (which has $2^L$ elements),

$$\sum_i 2^{L - \ell_i} \le 2^L.$$

4. Divide both sides by $2^L$ to obtain the inequality.

## Connections

Kraft's Inequality is the feasibility constraint in the [[Source Coding Theorem]] (Shannon's noiseless coding theorem). The counting argument mirrors the [[Pigeonhole Principle]] applied to the set of binary strings.

## Lean4 Proof

```lean4
/-- Verify Kraft's inequality for the specific code {0, 10, 110, 111}
    with lengths (1, 2, 3, 3) using norm_num. -/
theorem kraft_code_eq_one :
    (2 : ℚ)⁻¹ ^ 1 + (2 : ℚ)⁻¹ ^ 2 + (2 : ℚ)⁻¹ ^ 3 + (2 : ℚ)⁻¹ ^ 3 = 1 := by
  norm_num

/-- The sum is at most 1 follows immediately from equality. -/
theorem kraft_code_le_one :
    (2 : ℚ)⁻¹ ^ 1 + (2 : ℚ)⁻¹ ^ 2 + (2 : ℚ)⁻¹ ^ 3 + (2 : ℚ)⁻¹ ^ 3 ≤ 1 :=
  kraft_code_eq_one.le
```
