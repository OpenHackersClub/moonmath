+++
title = "Euler's Totient Function"
description = "φ(n) counts integers up to n coprime to n, and governs modular exponentiation"
weight = 90
tags = ["lean4-proof", "number-theory", "modular-arithmetic", "group-theory", "visualization"]
latex = "a^{\\varphi(n)} \\equiv 1 \\pmod{n}"
prerequisites = ["fermats-little-theorem", "bezout-identity", "chinese-remainder-theorem"]
lean4_status = "complete"
+++

## Statement

The **Euler totient function** $\varphi(n)$ counts the integers in $\{1, \ldots, n\}$ that are coprime to $n$:

$$\varphi(n) = \#\{k : 1 \le k \le n,\ \gcd(k, n) = 1\}$$

**Euler's theorem** says: for any $a$ with $\gcd(a, n) = 1$,

$$a^{\varphi(n)} \equiv 1 \pmod{n}$$

This generalises [[Fermat's Little Theorem]] (which is the special case $n = p$ prime, $\varphi(p) = p-1$).

## Visualization

**Totient values** for small $n$:

| $n$  | Coprime residues                | $\varphi(n)$ |
|-----:|:--------------------------------|-------------:|
|  1   | $\{1\}$                         | 1            |
|  2   | $\{1\}$                         | 1            |
|  4   | $\{1, 3\}$                      | 2            |
|  6   | $\{1, 5\}$                      | 2            |
|  8   | $\{1, 3, 5, 7\}$                | 4            |
|  9   | $\{1, 2, 4, 5, 7, 8\}$         | 6            |
| 10   | $\{1, 3, 7, 9\}$                | 4            |
| 12   | $\{1, 5, 7, 11\}$              | 4            |

**Key formulas**:

```
φ(p)      = p - 1              (p prime)
φ(p^k)    = p^k - p^(k-1)     = p^(k-1)(p - 1)
φ(mn)     = φ(m)φ(n)          if gcd(m,n) = 1   (multiplicativity)
φ(n)      = n · ∏_{p | n} (1 - 1/p)             (general formula)
```

**Euler's theorem trace** for $a = 3$, $n = 10$, $\varphi(10) = 4$:

```
3^1  =   3  ≡ 3 (mod 10)
3^2  =   9  ≡ 9 (mod 10)
3^3  =  27  ≡ 7 (mod 10)
3^4  =  81  ≡ 1 (mod 10)   ← 3^φ(10) ≡ 1 ✓
```

**Group structure**: the units $(\mathbb{Z}/n\mathbb{Z})^\times$ form a group of order $\varphi(n)$. Euler's theorem is just Lagrange's theorem applied to $a$ in this group.

## Proof Sketch

1. **The units form a group.** $(\mathbb{Z}/n\mathbb{Z})^\times = \{a \bmod n : \gcd(a,n) = 1\}$ is a multiplicative group of order $\varphi(n)$ (closure under multiplication uses [[Bezout's Identity]] to show the product of two coprime-to-$n$ elements stays coprime to $n$).

2. **Lagrange's theorem.** For any finite group $G$ and any $a \in G$, the order of $a$ divides $|G|$. So $a^{|G|} = e$ (identity).

3. **Apply to our group.** $|(\mathbb{Z}/n\mathbb{Z})^\times| = \varphi(n)$, so $a^{\varphi(n)} \equiv 1 \pmod n$ for any unit $a$.

4. **Multiplicativity.** For coprime $m, n$, the [[Chinese Remainder Theorem]] gives a ring isomorphism $\mathbb{Z}/mn\mathbb{Z} \cong \mathbb{Z}/m\mathbb{Z} \times \mathbb{Z}/n\mathbb{Z}$. Passing to units: $\varphi(mn) = \varphi(m)\varphi(n)$.

## Connections

Euler's theorem is the engine of RSA encryption: a message $M$ encrypted as $M^e \bmod n$ (with $\gcd(e, \varphi(n))=1$) is decrypted by $C^d \bmod n$ where $ed \equiv 1 \pmod{\varphi(n)}$ (found via [[Bezout's Identity]]). The special case $n=p$ recovers [[Fermat's Little Theorem]]. The multiplicativity formula uses the [[Chinese Remainder Theorem]]. The totient counts units in $\mathbb{Z}/n\mathbb{Z}$, linking to [[Wilson's Theorem]] which identifies when this group has product $-1$. Prime counting via the [[Fundamental Theorem of Arithmetic]] gives the inclusion-exclusion formula for $\varphi$.

## Lean4 Proof

```lean4
/-- **Euler's theorem**: every unit of `ZMod n` raised to the `n.totient`-th
    power equals 1. Mathlib states this as `ZMod.pow_totient`. -/
theorem euler_theorem {n : ℕ} (x : (ZMod n)ˣ) : x ^ n.totient = 1 :=
  ZMod.pow_totient x
```
