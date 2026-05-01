+++
title = "Möbius Inversion"
description = "If g equals the Dirichlet convolution of f with the constant 1, then f recovers via the Möbius function"
weight = 130
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "f(n) = \\sum_{d \\mid n} \\mu(d)\\, g\\!\\left(\\frac{n}{d}\\right)"
prerequisites = ["fundamental-theorem-arithmetic", "multiplicative-functions", "euler-totient"]
lean4_status = "complete"
+++

## Statement

The **Möbius function** $\mu : \mathbb{N} \to \{-1, 0, 1\}$ is defined by

$$\mu(n) = \begin{cases} 1 & \text{if } n = 1 \\ (-1)^k & \text{if } n = p_1 p_2 \cdots p_k \text{ is squarefree with } k \text{ distinct primes} \\ 0 & \text{if } n \text{ has a squared prime factor.} \end{cases}$$

**Möbius Inversion Formula:** If $g(n) = \sum_{d \mid n} f(d)$ for all $n$, then

$$f(n) = \sum_{d \mid n} \mu(d)\, g\!\left(\frac{n}{d}\right).$$

This is the statement that $\mu$ is the Dirichlet-convolution inverse of the constant function $\mathbf{1}$:

$$\mu * \mathbf{1} = \varepsilon, \qquad \text{where } \varepsilon(n) = [n = 1].$$

## Visualization

**Values of $\mu(n)$ for small $n$:**

```
n   Factorization     μ(n)
----+----------------+------
1   1                 1
2   2                -1
3   3                -1
4   2²                0   ← squared factor
5   5                -1
6   2·3               1   ← 2 prime factors, squarefree
7   7                -1
8   2³                0
9   3²                0
10  2·5               1
11  11               -1
12  2²·3              0
30  2·3·5            -1   ← 3 prime factors, squarefree
```

**Inversion in action:** Take $f(n) = \phi(n)$ (Euler's totient). Then $g(n) = \sum_{d\mid n} \phi(d) = n$ (a classical identity). Möbius inversion recovers $\phi(n) = \sum_{d \mid n} \mu(d) \cdot (n/d) = n \prod_{p \mid n}(1 - 1/p)$.

## Proof Sketch

1. **Key identity:** Show $\sum_{d \mid n} \mu(d) = [n = 1]$. For $n = 1$ this is trivial. For $n > 1$, write $n = p_1^{a_1} \cdots p_k^{a_k}$. The sum over divisors $d$ of $n$ with $\mu(d) \neq 0$ is just the sum over squarefree divisors, which equals $\sum_{j=0}^{k} \binom{k}{j}(-1)^j = (1-1)^k = 0$.

2. **Substitution:** Expand $\sum_{d \mid n} \mu(d)\, g(n/d) = \sum_{d \mid n} \mu(d) \sum_{e \mid (n/d)} f(e)$.

3. **Reindex:** Switch the order: for each divisor $e$ of $n$, collect all $d$ dividing $n/e$:

   $$= \sum_{e \mid n} f(e) \sum_{d \mid (n/e)} \mu(d) = \sum_{e \mid n} f(e) \cdot [n/e = 1] = f(n).$$

4. **Dirichlet series view:** The generating Dirichlet series for $\mathbf{1}$ is $\zeta(s)$ and for $\mu$ is $1/\zeta(s)$, so their product is $1$, which is the Dirichlet series for $\varepsilon$.

## Connections

Möbius inversion is the cornerstone of analytic and algebraic number theory. It immediately yields the formula for [[Euler's Totient]] $\phi(n) = n \prod_{p \mid n}(1 - 1/p)$. It underpins the Riemann zeta function and prime-counting via $\Lambda(n) = -\sum_{d \mid n} \mu(d) \log d$. The formula generalises to any locally finite partially ordered set (Rota's generalisation). The Möbius function is itself a [[Multiplicative Functions|multiplicative function]], and the inversion formula preserves multiplicativity. See also [[Fundamental Theorem of Arithmetic]] for the squarefree factorization structure.

## Lean4 Proof

```lean4
open ArithmeticFunction in
/-- The Möbius function is the Dirichlet-convolution inverse of the constant-1
    arithmetic function (the zeta function). In Mathlib this is stated as
    `moebius_mul_coe_zeta : (μ * ζ : ArithmeticFunction ℤ) = 1`. -/
theorem moebius_zeta_inverse :
    (ArithmeticFunction.moebius * ArithmeticFunction.zeta : ArithmeticFunction ℤ) = 1 :=
  ArithmeticFunction.moebius_mul_coe_zeta
```