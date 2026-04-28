+++
title = "Fermat's Little Theorem"
description = "For prime p, a^p is congruent to a mod p"
weight = 30
difficulty = "intermediate"
tags = ["lean4-proof", "number-theory", "modular-arithmetic"]
latex = "a^p \\equiv a \\pmod{p}"
prerequisites = ["fundamental-theorem-arithmetic"]
lean4_status = "sorry"
+++

## Statement

If $p$ is a prime number, then for any integer $a$:

$$a^p \equiv a \pmod{p}$$

Equivalently, if $\gcd(a, p) = 1$:

$$a^{p-1} \equiv 1 \pmod{p}$$

## Proof via Necklace Counting

Consider the set of all strings of length $p$ over an alphabet of $a$ symbols. There are $a^p$ such strings total. Exactly $a$ of them are constant (all symbols the same). The remaining $a^p - a$ strings can be grouped into equivalence classes under cyclic rotation, each of size exactly $p$ (since $p$ is prime and the string is non-constant). Therefore $p \mid (a^p - a)$, giving us $a^p \equiv a \pmod{p}$.

## Applications

- **Primality testing:** Fermat's test checks whether $a^{n-1} \equiv 1 \pmod{n}$
- **RSA cryptography:** The RSA algorithm relies on Euler's generalization of this theorem
- **Modular inverses:** When $\gcd(a, p) = 1$, the inverse of $a$ modulo $p$ is $a^{p-2}$

## Connections

This theorem builds on the [[Fundamental Theorem of Arithmetic]] and is a key ingredient in [[Quadratic Reciprocity]].

## Lean4 Proof

```lean4
/-- The number of length-p strings over a symbols is a^p.
    Constant strings account for a of them; the rest form
    orbits of size p under cyclic rotation (since p is prime). -/
theorem necklace_count (a p : Nat) (hp : Nat.Prime p) :
    p ∣ (a ^ p - a) := by
  sorry -- by counting necklaces: (a^p - a) non-constant strings, each orbit has size p

/-- Fermat's little theorem: a^p ≡ a (mod p) for prime p. -/
theorem fermat_little (a : Nat) (p : Nat) (hp : Nat.Prime p) :
    a ^ p % p = a % p := by
  sorry -- follows from necklace_count
```
