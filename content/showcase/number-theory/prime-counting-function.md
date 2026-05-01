+++
title = "Prime Counting Function π(n)"
description = "π(n) counts primes up to n and satisfies π(n) ~ n/ln(n) by the Prime Number Theorem"
weight = 300
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\pi(n) = \\#\\{p \\le n : p \\text{ prime}\\}"
prerequisites = ["fundamental-theorem-arithmetic", "bertrand-postulate"]
lean4_status = "complete"
+++

## Statement

The **prime counting function** $\pi(n)$ counts the number of primes up to $n$:

$$\pi(n) = \#\{p \le n : p \text{ is prime}\}.$$

The **Prime Number Theorem** (Hadamard and de la Vallée-Poussin, 1896) gives the asymptotic:

$$\pi(n) \sim \frac{n}{\ln n} \quad (n \to \infty),$$

meaning $\pi(n) / (n / \ln n) \to 1$. Equivalently, $\pi(n) \sim \mathrm{Li}(n)$ where $\mathrm{Li}(n) = \int_2^n dt/\ln t$ is the logarithmic integral.

Mathlib defines `Nat.primeCounting n` as the number of primes $\le n$, with notation $\pi$ (scoped to `Nat.Prime`).

## Visualization

Exact values of $\pi(n)$ and comparison with the approximation $n/\ln n$:

| $n$    | $\pi(n)$ | $\lfloor n/\ln n \rfloor$ | $\pi(n) - \lfloor n/\ln n \rfloor$ |
|--------|----------|---------------------------|-------------------------------------|
| 10     | 4        | 4                         | 0                                   |
| 50     | 15       | 11                        | 4                                   |
| 100    | 25       | 21                        | 4                                   |
| 500    | 95       | 80                        | 15                                  |
| 1000   | 168      | 144                       | 24                                  |
| 5000   | 669      | 591                       | 78                                  |
| 10000  | 1229     | 1085                      | 144                                 |

The logarithmic integral $\mathrm{Li}(n)$ is a better approximation (within $O(\sqrt{n}\ln n)$ assuming the Riemann Hypothesis):

| $n$    | $\pi(n)$ | $\mathrm{Li}(n)$ (approx) | error      |
|--------|----------|---------------------------|------------|
| 1000   | 168      | 178                       | $-10$      |
| 10000  | 1229     | 1246                      | $-17$      |
| 100000 | 9592     | 9630                      | $-38$      |

Note $\mathrm{Li}(n) > \pi(n)$ for all computed values, though Skewes showed the inequality reverses for some astronomically large $n$.

## Proof Sketch

1. **Elementary lower bound.** By [[Bertrand's Postulate]], $\pi(2n) \ge \pi(n) + 1$, giving $\pi(n) \ge \lfloor \log_2 n \rfloor$.
2. **Chebyshev bounds.** $(\ln 2)\, n/\ln n \le \pi(n) \le (2\ln 2)\, n/\ln n$ for large $n$ (see [[Chebyshev's Bounds for π(n)]]).
3. **Prime Number Theorem.** The full asymptotics use complex analysis: $\zeta(s) \ne 0$ on $\Re(s) = 1$ (zero-free region) implies $\pi(n) \sim n/\ln n$.
4. **Equivalent forms.** $\pi(n) \sim n/\ln n \iff \psi(n) \sim n$ (von Mangoldt function) $\iff$ $\vartheta(n) \sim n$ (Chebyshev $\vartheta$).

## Connections

$\pi(n)$ is the primary object of study in analytic number theory, shaped by [[Chebyshev's Bounds for π(n)]] and [[Mertens' Theorems]]. The [[Infinitude of Primes]] shows $\pi(n) \to \infty$; the Prime Number Theorem quantifies the rate. The [[Multiplicative Functions]] Dirichlet series generating function is $-\zeta'(s)/\zeta(s) = \sum_{n\ge1} \Lambda(n) n^{-s}$, linking $\pi$ to the Riemann zeta function.

## Lean4 Proof

```lean4
import Mathlib.NumberTheory.PrimeCounting
import Mathlib.Data.Nat.Prime.Basic

/-- π(10) = 4: there are exactly 4 primes ≤ 10 (namely 2, 3, 5, 7). -/
theorem pi_10 : Nat.primeCounting 10 = 4 := by decide

/-- π is monotone: if m ≤ n then π(m) ≤ π(n). -/
theorem pi_monotone : Monotone Nat.primeCounting :=
  Nat.monotone_primeCounting

/-- π(n) is unbounded: Nat.tendsto_primeCounting' shows it diverges to infinity. -/
theorem pi_tendsto : Filter.Tendsto Nat.primeCounting' Filter.atTop Filter.atTop :=
  Nat.tendsto_primeCounting'
```
