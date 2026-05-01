+++
title = "Chinese Remainder Theorem"
description = "Simultaneous congruences with coprime moduli have a unique solution mod their product"
weight = 70
tags = ["lean4-proof", "number-theory", "modular-arithmetic"]
latex = "\\mathbb{Z}/mn\\mathbb{Z} \\cong \\mathbb{Z}/m\\mathbb{Z} \\times \\mathbb{Z}/n\\mathbb{Z}"
prerequisites = ["bezout-identity", "euclidean-algorithm"]
lean4_status = "complete"
+++

## Statement

Let $m$ and $n$ be coprime natural numbers (i.e. $\gcd(m,n) = 1$). Then for any integers $a$ and $b$ there exists a unique $x$ modulo $mn$ such that:

$$x \equiv a \pmod{m} \qquad \text{and} \qquad x \equiv b \pmod{n}$$

In algebraic terms this is a **ring isomorphism**:

$$\mathbb{Z}/mn\mathbb{Z} \;\cong\; \mathbb{Z}/m\mathbb{Z} \;\times\; \mathbb{Z}/n\mathbb{Z}$$

## Visualization

**Worked example**: solve $x \equiv 2 \pmod{3}$ and $x \equiv 3 \pmod{5}$.

```
mod 3:  ...,  2,  5,  8, 11, 14, 17, 20, 23, ...   (2 mod 3)
mod 5:  ...,  3,  8, 13, 18, 23, 28, 33, ...        (3 mod 5)
                   ↑                  ↑
                   8 ≡ 2 (mod 3) and 8 ≡ 3 (mod 5)   ✓
```

Unique solution: $x \equiv 8 \pmod{15}$.

**Construction via Bezout**:

| Step | Action                                  | Value           |
|-----:|:----------------------------------------|----------------:|
| 1    | Find $u, v$ with $3u + 5v = 1$         | $u=2,\ v=-1$   |
| 2    | $e_1 = 5v \cdot a = 5(-1)(2) = -10$   | $\equiv 2 \pmod 3$ |
| 3    | $e_2 = 3u \cdot b = 3(2)(3) = 18$     | $\equiv 3 \pmod 5$ |
| 4    | $x = e_1 + e_2 = -10 + 18 = 8$        | $8 \bmod 15 = 8$ |

**Bijection table** for $m=2, n=3$ (all residues mod 6):

| $x \bmod 6$ | $x \bmod 2$ | $x \bmod 3$ |
|:-----------:|:-----------:|:-----------:|
| 0           | 0           | 0           |
| 1           | 1           | 1           |
| 2           | 0           | 2           |
| 3           | 1           | 0           |
| 4           | 0           | 1           |
| 5           | 1           | 2           |

Every pair $(\text{mod }2, \text{mod }3)$ appears exactly once — the isomorphism is a bijection.

## Proof Sketch

1. **Existence.** By [[Bezout's Identity]] there exist $u, v$ with $mu + nv = 1$. Set $x = a \cdot nv + b \cdot mu$. Then $x \bmod m = a \cdot nv \bmod m = a(1 - mu)v \bmod m = a \bmod m = a$ (and similarly for $b$).

2. **Uniqueness.** If $x$ and $x'$ both satisfy both congruences then $m \mid (x - x')$ and $n \mid (x - x')$. Since $\gcd(m,n)=1$, we have $mn \mid (x - x')$, so $x \equiv x' \pmod{mn}$.

3. **Ring isomorphism.** The map $\phi: \mathbb{Z}/mn\mathbb{Z} \to \mathbb{Z}/m\mathbb{Z} \times \mathbb{Z}/n\mathbb{Z}$ sending $x$ to $(x \bmod m, x \bmod n)$ is a ring homomorphism. Existence and uniqueness show it is bijective.

## Connections

CRT is the modular-arithmetic analogue of product decomposition. It generalises to any finite collection of pairwise coprime moduli. It is the basis for fast arithmetic in cryptography (RSA uses it for efficient decryption) and connects to [[Euler's Totient Function]] via the multiplicativity $\phi(mn) = \phi(m)\phi(n)$ for coprime $m,n$. The structure mirrors [[Bezout's Identity]] in action, and the coprimality assumption echoes the role of primes in the [[Fundamental Theorem of Arithmetic]].

## Lean4 Proof

```lean4
/-- **Chinese Remainder Theorem**: for coprime moduli `m` and `n`, the ring
    `ZMod (m * n)` is isomorphic to `ZMod m × ZMod n`.
    Mathlib provides the isomorphism as `ZMod.chineseRemainder`. -/
theorem crt {m n : ℕ} (h : Nat.Coprime m n) :
    ZMod (m * n) ≃+* ZMod m × ZMod n :=
  ZMod.chineseRemainder h
```
