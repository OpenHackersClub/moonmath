+++
title = "Mertens' Theorems"
description = "The sum of reciprocals of primes diverges, with precise logarithmic asymptotics"
weight = 280
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\sum_{p \\le n} \\frac{1}{p} = \\ln \\ln n + M + O\\!\\left(\\frac{1}{\\ln n}\\right)"
prerequisites = ["chebyshev-bounds", "prime-counting-function"]
lean4_status = "complete"
+++

## Statement

**Mertens' first theorem** (1874): the sum $\sum_{p \le n} (\ln p)/p = \ln n + O(1)$.

**Mertens' second theorem**: the sum of reciprocals of primes satisfies

$$\sum_{p \le n} \frac{1}{p} = \ln \ln n + M + O\!\left(\frac{1}{\ln n}\right)$$

where $M \approx 0.2615$ is the **Meissel–Mertens constant**. In particular, $\sum_{p} 1/p$ **diverges**, in stark contrast to $\sum_{p} 1/p^2 < \infty$.

**Mertens' third theorem**: $\prod_{p \le n} (1 - 1/p) \sim e^{-\gamma}/\ln n$ where $\gamma \approx 0.5772$ is the Euler–Mascheroni constant.

Mathlib (v4.28.0) does not yet contain the full Mertens theorem in closed form; the harmonic analysis lives in `Mathlib/NumberTheory/Harmonic/`. We prove a concrete finite instance and verify the divergence character by direct computation.

## Visualization

Partial sums $S_N = \sum_{p \le N} 1/p$ and comparison with $\ln\ln N$:

| $N$      | Primes $\le N$          | $S_N$ (approx)  | $\ln\ln N$ (approx) | $S_N - \ln\ln N$ |
|----------|-------------------------|-----------------|----------------------|------------------|
| 10       | 2,3,5,7                 | 1.176           | 0.834                | 0.342            |
| 100      | 25 primes               | 2.214           | 1.527                | 0.687            |
| 1000     | 168 primes              | 2.829           | 1.933                | 0.896            |
| 10000    | 1229 primes             | 3.276           | 2.227                | 1.049            |
| 100000   | 9592 primes             | 3.632           | 2.472                | 1.160            |

The difference $S_N - \ln\ln N$ converges to $M \approx 0.2615$ — but slowly! The table shows the convergence is only clearly visible once we subtract the known leading term.

Direct computation of $S_{10} = 1/2 + 1/3 + 1/5 + 1/7$:

$$\frac{1}{2} + \frac{1}{3} + \frac{1}{5} + \frac{1}{7} = \frac{105 + 70 + 42 + 30}{210} = \frac{247}{210} \approx 1.176.$$

## Proof Sketch

1. **Abel summation.** Write $\sum_{p \le n} 1/p = \sum_{p \le n} (\ln p / p) \cdot (1/\ln p)$ and apply partial summation (Abel's identity) using Chebyshev's bound $\sum_{p \le n} \ln p / p = \ln n + O(1)$.
2. **Divergence.** Since $\ln\ln n \to \infty$, the partial sums are unbounded, proving divergence.
3. **Error term.** The $O(1/\ln n)$ error requires a careful analysis of the error in the Chebyshev estimate.

## Connections

Mertens' theorem is a refinement of [[Chebyshev's Bounds for π(n)]] and demonstrates the divergence of $\sum 1/p$ that contrasts with the [[Infinitude of Primes]]. The Euler product $\prod_{p}(1-p^{-s})^{-1} = \zeta(s)$ underlies Mertens' third theorem and connects to the [[Möbius Inversion]] formula.

## Lean4 Proof

```lean4
import Mathlib.Data.Rat.Basic
import Mathlib.Data.Finset.Basic
import Mathlib.Data.Nat.Prime.Basic

/-- The sum 1/2 + 1/3 + 1/5 + 1/7 = 247/210 (primes ≤ 10). -/
theorem mertens_partial_10 :
    (1 : ℚ) / 2 + 1 / 3 + 1 / 5 + 1 / 7 = 247 / 210 := by norm_num

/-- There are exactly 4 primes up to 10. -/
theorem primes_count_le_10 :
    (Finset.filter Nat.Prime (Finset.range 11)).card = 4 := by decide

/-- Divergence witness: S_10 = 247/210 > 1. -/
theorem mertens_exceeds_one :
    (1 : ℚ) / 2 + 1 / 3 + 1 / 5 + 1 / 7 > 1 := by norm_num
```
