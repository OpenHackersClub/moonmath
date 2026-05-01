+++
title = "Bertrand's Postulate"
description = "For every positive integer n there is always a prime p with n < p ≤ 2n"
weight = 260
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\forall n \\ge 1,\\; \\exists p \\text{ prime},\\; n < p \\le 2n"
prerequisites = ["fundamental-theorem-arithmetic"]
lean4_status = "complete"
+++

## Statement

**Bertrand's postulate** (proved by Chebyshev in 1852): for every positive integer $n$ there exists a prime $p$ satisfying

$$n < p \le 2n.$$

Equivalently, the gap between consecutive primes never exceeds the smaller prime: $p_{k+1} < 2p_k$ for all $k$.

## Visualization

For each $n$ from $1$ to $20$, the smallest prime $p$ in the interval $(n, 2n]$:

| $n$  | Interval     | Prime $p$ |
|------|-------------|-----------|
| 1    | $(1,2]$     | 2         |
| 2    | $(2,4]$     | 3         |
| 3    | $(3,6]$     | 5         |
| 4    | $(4,8]$     | 5         |
| 5    | $(5,10]$    | 7         |
| 6    | $(6,12]$    | 7         |
| 7    | $(7,14]$    | 11        |
| 8    | $(8,16]$    | 11        |
| 9    | $(9,18]$    | 11        |
| 10   | $(10,20]$   | 11        |
| 11   | $(11,22]$   | 13        |
| 12   | $(12,24]$   | 13        |
| 13   | $(13,26]$   | 17        |
| 14   | $(14,28]$   | 17        |
| 15   | $(15,30]$   | 17        |
| 16   | $(16,32]$   | 17        |
| 17   | $(17,34]$   | 19        |
| 18   | $(18,36]$   | 19        |
| 19   | $(19,38]$   | 23        |
| 20   | $(20,40]$   | 23        |

## Proof Sketch

Chebyshev's proof uses the **central binomial coefficient** $\binom{2n}{n}$.

1. **Lower bound.** $\binom{2n}{n} \ge 4^n / (2n+1)$ grows exponentially.
2. **Upper bound without Bertrand.** If no prime lies in $(n, 2n]$, every prime power in the factorisation of $\binom{2n}{n}$ is at most $n$. Using estimates on how many times each prime $p \le n$ divides $\binom{2n}{n}$ (each at most $\log_p(2n)$ times, each prime $> 2n/3$ divides exactly once, and so on), one obtains an upper bound.
3. **Contradiction.** For large $n$, the lower bound exceeds the upper bound. Small cases ($n \le 511$) are verified by exhibiting explicit primes: $2, 3, 5, 7, 13, 23, 43, 83, 163, 317$ each lie in the required interval.

## Connections

The postulate gives a lower bound on the [[Prime Counting Function π(n)]]: $\pi(2n) - \pi(n) \ge 1$ for all $n \ge 1$. It is also used to prove [[Chebyshev's Bounds for π(n)]]: iterating Bertrand gives $\pi(n) \ge \log_2(\log_2 n)$.

## Lean4 Proof

```lean4
import Mathlib.NumberTheory.Bertrand

/-- Bertrand's postulate: for every positive n, there is a prime p with n < p ≤ 2n.
    Direct alias of `Nat.exists_prime_lt_and_le_two_mul` in Mathlib. -/
theorem bertrand (n : ℕ) (hn : n ≠ 0) :
    ∃ p, Nat.Prime p ∧ n < p ∧ p ≤ 2 * n :=
  Nat.exists_prime_lt_and_le_two_mul n hn

/-- Concrete instance: there is a prime p with 10 < p ≤ 20. -/
theorem bertrand_10 : ∃ p, Nat.Prime p ∧ 10 < p ∧ p ≤ 20 :=
  ⟨11, by norm_num, by norm_num, by norm_num⟩
```
