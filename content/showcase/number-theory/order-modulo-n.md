+++
title = "Order Modulo n"
description = "The multiplicative order of a mod n is the smallest positive k with a^k ≡ 1 (mod n), and it always divides phi(n)"
weight = 210
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\text{ord}_n(a) \\mid \\varphi(n)"
prerequisites = ["eulers-totient", "fermats-little-theorem"]
lean4_status = "complete"
+++

## Statement

For $\gcd(a, n) = 1$, the **multiplicative order** of $a$ modulo $n$ is

$$\text{ord}_n(a) = \min\{k \ge 1 : a^k \equiv 1 \pmod{n}\}$$

In Lean this is `orderOf a` for `a : (ZMod n)ˣ`.

**Theorem (Lagrange).** $\text{ord}_n(a) \mid \phi(n)$.

In Mathlib: `ZMod.pow_totient` states $a^{\phi(n)} = 1$ for units, and `orderOf_dvd_card` (from group theory) gives $\text{ord}(a) \mid |G|$. Since $|(ZMod\; n)^\times| = \phi(n)$ by `ZMod.card_units_eq_totient`, we get the divisibility.

## Visualization

**Orders of elements in $(\mathbb{Z}/12\mathbb{Z})^\times$:**

The units mod 12 are $\{1, 5, 7, 11\}$ (those coprime to 12), so $\phi(12) = 4$.

| $a$ | Powers $a^1, a^2, a^3, a^4 \bmod 12$ | $\text{ord}_{12}(a)$ |
|-----|---------------------------------------|----------------------|
| 1   | 1, 1, 1, 1                            | 1                    |
| 5   | 5, 1, 5, 1                            | 2                    |
| 7   | 7, 1, 7, 1                            | 2                    |
| 11  | 11, 1, 11, 1                          | 2                    |

All orders (1, 2, 2, 2) divide $\phi(12) = 4$. Note: no element has order 4, so $(\mathbb{Z}/12\mathbb{Z})^\times$ is **not** cyclic (it is $\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$).

Contrast with $(\mathbb{Z}/7\mathbb{Z})^\times$ (prime modulus): 2 has order 3, while 3 has order 6 = $\phi(7)$, so 3 is a primitive root.

## Proof Sketch

1. The set $\{a, a^2, a^3, \ldots\}$ in the finite group $((\mathbb{Z}/n\mathbb{Z})^\times, \times)$ must eventually repeat; the first repetition gives a period $d = \text{ord}(a)$.
2. The cyclic subgroup $\langle a \rangle = \{1, a, \ldots, a^{d-1}\}$ has order $d$.
3. By Lagrange's theorem, $d \mid |(\mathbb{Z}/n\mathbb{Z})^\times| = \phi(n)$.
4. In particular $a^{\phi(n)} = (a^d)^{\phi(n)/d} = 1$, recovering the Euler–[[Euler's Totient Function]] theorem.

## Connections

The order always divides [[Euler's Totient Function]] $\phi(n)$, with equality iff $a$ is a primitive root (see [[Primitive Roots]]). For prime $n = p$ the divisibility $\text{ord}_p(a) \mid p-1$ is a corollary of [[Fermat's Little Theorem]].

## Lean4 Proof

```lean4
/-- The order of any unit in (ZMod n)ˣ divides the card of the group,
    which equals Euler's totient φ(n). -/
theorem order_dvd_totient (n : ℕ) [NeZero n] (a : (ZMod n)ˣ) :
    orderOf a ∣ n.totient := by
  have h1 : orderOf a ∣ Fintype.card (ZMod n)ˣ := orderOf_dvd_card
  rwa [ZMod.card_units_eq_totient] at h1
```
