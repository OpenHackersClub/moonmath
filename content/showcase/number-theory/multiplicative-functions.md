+++
title = "Multiplicative Functions"
description = "Arithmetic functions that split over coprime inputs: f(mn) = f(m)f(n) when gcd(m,n) = 1"
weight = 140
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\gcd(m,n)=1 \\Rightarrow f(mn)=f(m)f(n)"
prerequisites = ["fundamental-theorem-arithmetic", "euler-totient", "mobius-inversion"]
lean4_status = "complete"
+++

## Statement

An arithmetic function $f : \mathbb{N} \to R$ is **multiplicative** if:

1. $f(1) = 1$, and
2. $f(mn) = f(m) f(n)$ whenever $\gcd(m, n) = 1$.

It is **completely multiplicative** if condition 2 holds for *all* $m, n$ (not just coprime pairs).

A multiplicative function is completely determined by its values on prime powers, since every positive integer factors uniquely as $n = p_1^{a_1} \cdots p_k^{a_k}$ and then

$$f(n) = f(p_1^{a_1}) \cdots f(p_k^{a_k}).$$

## Visualization

**Key examples and their values:**

```
Function        f(1)  f(p)      f(p²)       f(mn) coprime rule
----------------+-----+---------+-----------+--------------------
φ (totient)      1    p-1       p(p-1)      φ(mn)=φ(m)φ(n)
μ (Möbius)       1    -1        0           μ(mn)=μ(m)μ(n) if sqfree
σ₁ (sum divs)   1    p+1       p²+p+1      σ₁(mn)=σ₁(m)σ₁(n)
τ (# divisors)  1    2         3           τ(mn)=τ(m)τ(n)
λ (Liouville)   1    -1        1           λ(mn)=λ(m)λ(n) (completely)
id (identity)   1    p         p²          id(mn)=id(m)id(n) (completely)
ε (unit)         1    0         0           ε(mn)=ε(m)ε(n)
```

**Table of values for small $n$:**

```
n    φ(n)  μ(n)  τ(n)  σ₁(n)
----+------+------+------+------
1     1     1     1     1
2     1    -1     2     3
3     2    -1     2     4
4     2     0     3     7
5     4    -1     2     6
6     2     1     4    12   ← 6=2·3, coprime: φ(6)=φ(2)φ(3)=1·2=2  ✓
8     4     0     4    15
12    4     0     6    28   ← 12=4·3: φ(12)=φ(4)φ(3)=2·2=4  ✓
30   8    -1     8    72   ← 30=2·3·5: τ(30)=2·2·2=8  ✓
```

## Proof Sketch

1. **Values at prime powers:** By the multiplicative property and the [[Fundamental Theorem of Arithmetic]], it suffices to know $f(p^k)$ for all primes $p$ and $k \geq 1$.

2. **Dirichlet convolution:** If $f$ and $g$ are multiplicative, so is their convolution $h(n) = \sum_{d \mid n} f(d) g(n/d)$. This is proved by grouping divisors of $mn$ (with $\gcd(m,n)=1$) as pairs $(d_1, d_2)$ with $d_1 \mid m$ and $d_2 \mid n$.

3. **Totient formula:** From $\sum_{d \mid n} \phi(d) = n$ (a completely multiplicative identity) and [[Möbius Inversion]], one recovers $\phi(n) = n \prod_{p \mid n}(1-1/p)$.

4. **Dirichlet series:** A multiplicative $f$ has an Euler product $\sum_n f(n) n^{-s} = \prod_p \sum_{k \geq 0} f(p^k) p^{-ks}$, a key tool in analytic number theory.

## Connections

Multiplicative functions form the backbone of elementary number theory. [[Möbius Inversion]] is entirely about the convolution algebra of multiplicative functions. [[Euler's Totient]] $\phi$ is the canonical example. The [[Möbius Inversion|Möbius function]] $\mu$ is the convolution inverse of the constant-$1$ function. The structure also appears in the proof of [[Quadratic Reciprocity]] via Gauss sums (which are "twisted" multiplicative functions), and in the analysis of the [[Euclidean Algorithm]] through $\gcd$-related identities. Dirichlet characters (used in proving primes in arithmetic progressions) are completely multiplicative functions on $(\mathbb{Z}/n\mathbb{Z})^\times$.

## Lean4 Proof

```lean4
/-- Euler's totient is multiplicative: φ(m·n) = φ(m)·φ(n) when gcd(m,n) = 1.
    Mathlib proves this as `Nat.totient_mul`. -/
theorem totient_multiplicative {m n : ℕ} (h : Nat.Coprime m n) :
    (m * n).totient = m.totient * n.totient :=
  Nat.totient_mul h

/-- The number-of-divisors function τ is multiplicative.
    In Mathlib's `ArithmeticFunction` framework, `ArithmeticFunction.sigma 0`
    counts divisors (since σ₀(n) = #{d : d ∣ n}), and is `IsMultiplicative`. -/
theorem sigma_zero_multiplicative :
    (ArithmeticFunction.sigma 0).IsMultiplicative :=
  ArithmeticFunction.isMultiplicative_sigma 0
```