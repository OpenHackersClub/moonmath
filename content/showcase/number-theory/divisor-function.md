+++
title = "Divisor Function σ(n)"
description = "The sum-of-divisors function σ(n) is multiplicative and characterises perfect numbers"
weight = 290
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\sigma(n) = \\sum_{d \\mid n} d"
prerequisites = ["multiplicative-functions", "fundamental-theorem-arithmetic"]
lean4_status = "complete"
+++

## Statement

The **divisor function** (or sum-of-divisors function) $\sigma(n)$ is defined for positive integers $n$ by

$$\sigma(n) = \sum_{d \mid n} d.$$

More generally, $\sigma_k(n) = \sum_{d \mid n} d^k$; the case $k = 1$ is the most classical.

Key property: $\sigma$ is **multiplicative** — for coprime $m, n$:

$$\gcd(m, n) = 1 \implies \sigma(mn) = \sigma(m)\,\sigma(n).$$

A number $n$ is **perfect** if $\sigma(n) = 2n$ (equivalently, the sum of proper divisors equals $n$).

## Visualization

Values of $\sigma(n)$ for $n = 1$ to $20$. Perfect numbers ($\sigma(n) = 2n$) are starred:

| $n$  | Divisors of $n$       | $\sigma(n)$ | Note          |
|------|-----------------------|-------------|---------------|
| 1    | 1                     | 1           |               |
| 2    | 1, 2                  | 3           |               |
| 3    | 1, 3                  | 4           |               |
| 4    | 1, 2, 4               | 7           |               |
| 5    | 1, 5                  | 6           |               |
| 6    | 1, 2, 3, 6            | 12          | **perfect** $\star$ |
| 7    | 1, 7                  | 8           |               |
| 8    | 1, 2, 4, 8            | 15          |               |
| 9    | 1, 3, 9               | 13          |               |
| 10   | 1, 2, 5, 10           | 18          |               |
| 11   | 1, 11                 | 12          |               |
| 12   | 1, 2, 3, 4, 6, 12     | 28          |               |
| 13   | 1, 13                 | 14          |               |
| 14   | 1, 2, 7, 14           | 24          |               |
| 15   | 1, 3, 5, 15           | 24          |               |
| 16   | 1, 2, 4, 8, 16        | 31          |               |
| 17   | 1, 17                 | 18          |               |
| 18   | 1, 2, 3, 6, 9, 18     | 39          |               |
| 19   | 1, 19                 | 20          |               |
| 20   | 1, 2, 4, 5, 10, 20    | 42          |               |
| 28   | 1, 2, 4, 7, 14, 28    | 56          | **perfect** $\star$ |

Multiplicativity check: $\sigma(4) = 7$, $\sigma(3) = 4$, $\sigma(12) = \sigma(4 \cdot 3) = 7 \cdot 4 = 28$. $\checkmark$

## Proof Sketch

1. **Multiplicativity.** For coprime $m, n$, every divisor of $mn$ factors uniquely as $d_1 d_2$ with $d_1 \mid m$, $d_2 \mid n$. Hence $\sum_{d \mid mn} d = \bigl(\sum_{d_1 \mid m} d_1\bigr)\bigl(\sum_{d_2 \mid n} d_2\bigr) = \sigma(m)\sigma(n)$.
2. **Prime powers.** $\sigma(p^k) = 1 + p + p^2 + \cdots + p^k = (p^{k+1}-1)/(p-1)$.
3. **General formula.** By multiplicativity, for $n = \prod p_i^{e_i}$: $\sigma(n) = \prod_i \sigma(p_i^{e_i}) = \prod_i \frac{p_i^{e_i+1}-1}{p_i-1}$.
4. **Perfect numbers.** Even perfect numbers have the form $2^{p-1}(2^p - 1)$ where $2^p - 1$ is a Mersenne prime (Euler's theorem).

## Connections

$\sigma$ is the canonical example of a [[Multiplicative Functions|multiplicative function]]. The [[Möbius Inversion]] formula expresses $\sigma$ as a Dirichlet convolution $\sigma = \mathrm{id} * \mathbf{1}$, connecting it to the theory of [[Euler's Totient Function]].

## Lean4 Proof

```lean4
import Mathlib.NumberTheory.ArithmeticFunction.Misc

open ArithmeticFunction in
/-- σ is multiplicative: isMultiplicative_sigma from Mathlib. -/
theorem sigma_multiplicative {k : ℕ} :
    IsMultiplicative (σ k) :=
  isMultiplicative_sigma

open ArithmeticFunction in
/-- σ₁(6) = 12: sum of divisors of 6 is 1+2+3+6 = 12.
    We verify via the explicit divisor set and sigma_one_apply. -/
theorem sigma_6 : σ 1 6 = 12 := by
  rw [sigma_one_apply]
  decide

open ArithmeticFunction in
/-- 6 is perfect: σ₁(6) = 2·6. -/
theorem six_is_perfect : σ 1 6 = 2 * 6 := by
  rw [sigma_one_apply]
  decide
```
