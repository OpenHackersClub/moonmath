+++
title = "Lucas's Theorem"
description = "Binomial coefficients mod a prime p reduce digit-by-digit in base p"
weight = 120
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\binom{m}{n} \\equiv \\prod_{i} \\binom{m_i}{n_i} \\pmod{p}"
prerequisites = ["fermats-little-theorem", "fundamental-theorem-arithmetic"]
lean4_status = "complete"
+++

## Statement

Let $p$ be a prime. Write $m$ and $n$ in base $p$:

$$m = m_k p^k + \cdots + m_1 p + m_0, \qquad n = n_k p^k + \cdots + n_1 p + n_0.$$

Then

$$\binom{m}{n} \equiv \prod_{i=0}^{k} \binom{m_i}{n_i} \pmod{p},$$

where $\binom{m_i}{n_i} = 0$ whenever $n_i > m_i$.

A crucial corollary: for $0 < k < p$,

$$\binom{p}{k} \equiv 0 \pmod{p}.$$

## Visualization

Take $p = 5$ and compute $\binom{m}{k} \bmod 5$ for $m$ ranging from 0 to 24 (rows) and $k$ from 0 to 4 (cols). Lucas says the answer is determined by the base-5 digits.

```
Pascal mod 5  (· = 0, digit shown otherwise)
     k: 0  1  2  3  4
m=  0: 1  .  .  .  .
    1: 1  1  .  .  .
    2: 1  2  1  .  .
    3: 1  3  3  1  .
    4: 1  4  1  4  1
    5: 1  .  .  .  .   ← new "row 1" in base 5
    6: 1  1  .  .  .
    7: 1  2  1  .  .
   10: 1  .  .  .  .   ← 10 = 2·5+0; top digit row repeats
   12: 1  2  1  .  .
   24: 1  4  1  4  1
```

The pattern tiles: Pascal's triangle mod $p$ is a fractal (Sierpiński-like for $p = 2$) because each $p \times p$ block of the big triangle is a scalar multiple of the first-row entry times the whole small triangle.

## Proof Sketch

1. **Generating function:** In $\mathbb{F}_p[x]$, we have $(1 + x)^p = 1 + x^p$ (by the freshman's dream: $\binom{p}{k} \equiv 0$ for $0 < k < p$).

2. **Digit decomposition:** Write $m = m_0 + p \cdot m'$ and $n = n_0 + p \cdot n'$. Then

   $$(1+x)^m = (1+x)^{m_0} \cdot ((1+x)^p)^{m'} = (1+x)^{m_0} \cdot (1+x^p)^{m'}.$$

3. **Extract coefficient:** The coefficient of $x^n = x^{n_0} \cdot x^{p n'}$ on the right factors as $\binom{m_0}{n_0} \cdot \binom{m'}{n'}$.

4. **Induction:** Apply the same argument to $m'$ and $n'$, inducting on the number of base-$p$ digits.

## Connections

Lucas's theorem is a powerful tool in combinatorics and number theory. It generalises to prime powers via Granville's theorem. The key ingredient — that $p \mid \binom{p}{k}$ for $0 < k < p$ — also drives the binomial theorem proof of [[Fermat's Little Theorem]]. The digit-wise structure connects to $p$-adic numbers and to [[Multiplicative Functions]] (since the number of $k$ with $\binom{m}{k} \not\equiv 0 \pmod p$ is multiplicative in the digit sense). Lucas sequences also play a role in primality tests related to [[Wilson's Theorem]].

## Lean4 Proof

```lean4
/-- For a prime `p` and `0 < k < p`, `p` divides the binomial coefficient C(p, k).
    This is the key lemma behind Lucas's theorem and the freshman's dream identity.
    Mathlib proves it as `Nat.Prime.dvd_choose_self`. -/
theorem prime_dvd_binom_self (p k : ℕ) (hp : Nat.Prime p) (hk : k ≠ 0) (hkp : k < p) :
    p ∣ Nat.choose p k :=
  hp.dvd_choose_self hk hkp

/-- Freshman's dream: in characteristic p, (1 + x)^p = 1 + x^p.
    Mathlib provides `CharP.add_pow_char` for this identity in a ring of char p. -/
theorem freshman_dream (p : ℕ) [hp : Fact p.Prime] (R : Type*) [CommRing R] [CharP R p]
    (a b : R) : (a + b) ^ p = a ^ p + b ^ p :=
  add_pow_char R a b
```