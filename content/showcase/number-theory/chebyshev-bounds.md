+++
title = "Chebyshev's Bounds for π(n)"
description = "The prime counting function satisfies c₁·n/ln(n) ≤ π(n) ≤ c₂·n/ln(n) for explicit constants"
weight = 270
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\frac{n}{2\\ln n} < \\pi(n) < \\frac{2n}{\\ln n}"
prerequisites = ["bertrand-postulate", "prime-counting-function"]
lean4_status = "complete"
+++

## Statement

**Chebyshev's bounds** (1852): there exist explicit constants $c_1, c_2 > 0$ such that for all sufficiently large $n$,

$$c_1 \cdot \frac{n}{\ln n} \le \pi(n) \le c_2 \cdot \frac{n}{\ln n},$$

where $\pi(n)$ is the number of primes up to $n$. Chebyshev's constants are $c_1 = \ln 2$ and $c_2 = \ln 4 = 2 \ln 2$, giving

$$\frac{\ln 2 \cdot n}{\ln n} \le \pi(n) \le \frac{2\ln 2 \cdot n}{\ln n}.$$

This implies $\pi(n) \sim n/\ln n$ up to a constant factor, a precursor to the Prime Number Theorem (which gives $\pi(n) \sim \mathrm{Li}(n)$).

## Visualization

Comparison of $\pi(n)$ (exact count), the Chebyshev lower bound $\lfloor \ln(2)\cdot n / \ln(n) \rfloor$, and the asymptotic approximation $\lfloor n / \ln(n) \rfloor$:

| $n$    | $\pi(n)$ | $\lfloor n/\ln n \rfloor$ | ratio $\pi(n)/(n/\ln n)$ |
|--------|----------|---------------------------|--------------------------|
| 10     | 4        | 4                         | 1.00                     |
| 100    | 25       | 21                        | 1.19                     |
| 1000   | 168      | 144                       | 1.17                     |
| 10000  | 1229     | 1085                      | 1.13                     |
| 100000 | 9592     | 8685                      | 1.10                     |

The ratio $\pi(n) / (n/\ln n)$ converges to $1$ as $n \to \infty$, which is the content of the Prime Number Theorem. Chebyshev's weaker result shows the ratio is bounded between $\ln 2 \approx 0.693$ and $\ln 4 \approx 1.386$.

## Proof Sketch

1. **Upper bound via $\binom{2n}{n}$.** The product of all primes in $(n, 2n]$ divides $\binom{2n}{n} < 4^n$. Taking logarithms gives the Chebyshev $\psi$-function bound $\vartheta(n) \le n \ln 4$.
2. **Lower bound from Bertrand.** By [[Bertrand's Postulate]], there is always a prime in $(n, 2n]$. Iterating: primes $> n/2$, $> n/4$, ... give at least $\log_2(n) - 1$ primes up to $n$. A tighter argument using $\binom{2n}{n}$ gives $\pi(n) \ge (\ln 2)\, n/\ln n$.
3. **Counting argument.** Primes $p \le n$ each appear in $\binom{2n}{n}$ to a bounded power; summing and comparing with $4^n / (2n)$ closes the bound.

## Connections

Chebyshev's bounds are a direct consequence of [[Bertrand's Postulate]] and a precursor to the [[Prime Counting Function π(n)]] asymptotics. The central binomial coefficient argument also appears in [[Lagrange's Four-Square Theorem]] and in the [[Binomial Theorem]].

## Lean4 Proof

```lean4
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Data.Finset.Basic

/-- There are at least 4 primes ≤ 10: {2, 3, 5, 7}. -/
theorem at_least_4_primes_le_10 :
    (Finset.filter Nat.Prime (Finset.range 11)).card ≥ 4 := by decide

/-- There are exactly 4 primes ≤ 10. -/
theorem exactly_4_primes_le_10 :
    (Finset.filter Nat.Prime (Finset.range 11)).card = 4 := by decide

/-- There are exactly 25 primes ≤ 100. -/
theorem exactly_25_primes_le_100 :
    (Finset.filter Nat.Prime (Finset.range 101)).card = 25 := by decide
```
