+++
title = "Stirling's Approximation"
description = "The asymptotic formula n! ~ sqrt(2*pi*n) * (n/e)^n, with the Mathlib proof that n!/stirling(n) tends to 1."
weight = 130
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "n! \\sim \\sqrt{2\\pi n}\\left(\\frac{n}{e}\\right)^n"
prerequisites = ["catalan-numbers"]
lean4_status = "complete"
+++

## Statement

**Stirling's approximation:** As $n \to \infty$,

$$n! \sim \sqrt{2\pi n} \left(\frac{n}{e}\right)^n$$

More precisely, the ratio $\frac{n!}{\sqrt{2\pi n}(n/e)^n} \to 1$ as $n \to \infty$.

The approximation is already tight for moderate $n$ and gives the correct exponential order and polynomial correction for quantities like the central binomial coefficient $\binom{2n}{n} \sim \frac{4^n}{\sqrt{\pi n}}$.

## Visualization

Numerical comparison: exact $n!$ vs Stirling approximation $S(n) = \sqrt{2\pi n}(n/e)^n$ and their ratio.

| $n$ | $n!$ | $S(n)$ (approx) | ratio $n!/S(n)$ |
|---|---|---|---|
| 1 | 1 | 0.9221 | 1.0847 |
| 5 | 120 | 118.02 | 1.0168 |
| 10 | 3628800 | 3598695.6 | 1.0083 |
| 20 | $2.432 \times 10^{18}$ | $2.423 \times 10^{18}$ | 1.0042 |

The ratio decreases monotonically toward 1. Robbins (1955) showed:

$$\sqrt{2\pi n}\left(\frac{n}{e}\right)^n e^{1/(12n+1)} < n! < \sqrt{2\pi n}\left(\frac{n}{e}\right)^n e^{1/(12n)}$$

**Error magnitude.** The multiplicative error is $1 + \frac{1}{12n} + O(1/n^2)$, so for $n = 10$ the relative error is about $0.83\%$.

**Log version** (useful for information theory and combinatorics):

$$\ln(n!) = n\ln n - n + \frac{1}{2}\ln(2\pi n) + O(1/n)$$

This is the starting point for entropy estimates and the analysis of random permutations.

## Proof Sketch

1. **Wallis' product.** Show $\prod_{k=1}^{n} \frac{(2k)(2k)}{(2k-1)(2k+1)} \to \frac{\pi}{2}$ as $n \to \infty$ (Wallis 1655).
2. **Define Stirling sequence.** Let $a_n = \frac{n!}{(n/e)^n \sqrt{n}}$; show it is log-convex and decreasing.
3. **Identify the limit.** The limit $a = \lim a_n$ satisfies $\pi/2 = a^2/2$ via Wallis, giving $a = \sqrt{\pi}$.
4. **Reconstruct.** $\frac{n!}{\sqrt{2\pi n}(n/e)^n} = \frac{a_n}{\sqrt{2\pi}} \to \frac{\sqrt{\pi}}{\sqrt{2\pi}} \cdot \sqrt{2} = 1$.

The Mathlib proof in `Mathlib.Analysis.SpecialFunctions.Stirling` follows exactly this route: `Stirling.tendsto_stirlingSeq_sqrt_pi` proves $a_n \to \sqrt{\pi}$, and `Stirling.factorial_isEquivalent_stirling` packages the asymptotic equivalence.

## Connections

Stirling's approximation is the analytic foundation for estimates on [[Catalan Numbers]] ($C_n \sim \frac{4^n}{n^{3/2}\sqrt{\pi}}$) and [[Bell Numbers]]. It connects to the Wallis product and [[Taylor's Theorem]] (the Stirling series is an asymptotic expansion). The log version is the combinatorial backbone of the [[Inclusion-Exclusion Principle]] applied to derangements.

## Lean4 Proof

```lean4
import Mathlib.Analysis.SpecialFunctions.Stirling

/-- Stirling's formula: n! is asymptotically equivalent to sqrt(2*pi*n)*(n/e)^n.
    Mathlib's `Stirling.factorial_isEquivalent_stirling` states this. -/
theorem stirling_formula_alias :
    (fun n ↦ (n ! : ℝ)) ~[Filter.atTop]
      fun n ↦ Real.sqrt (2 * n * Real.pi) * (n / Real.exp 1) ^ n :=
  Stirling.factorial_isEquivalent_stirling

/-- Numeric sanity check: 10! = 3628800. -/
theorem factorial_10 : Nat.factorial 10 = 3628800 := by decide
```
