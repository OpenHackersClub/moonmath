+++
title = "Fermat's Little Theorem"
description = "For prime p, a^p is congruent to a mod p"
weight = 30
difficulty = "intermediate"
tags = ["lean4-proof", "number-theory", "modular-arithmetic"]
latex = "a^p \\equiv a \\pmod{p}"
prerequisites = ["fundamental-theorem-arithmetic"]
lean4_status = "complete"
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
/-- Fermat's little theorem: in `ZMod p`, every element raised to the
    p-th power is itself. Mathlib bundles the necklace-counting argument
    (and the polynomial-identity argument) into `ZMod.pow_card`. -/
theorem fermat_little (p : ℕ) [Fact p.Prime] (a : ZMod p) : a ^ p = a :=
  ZMod.pow_card a

/-- Equivalent form for nonzero residues: `a^(p-1) = 1`. -/
theorem fermat_little_nonzero (p : ℕ) [Fact p.Prime] {a : ZMod p}
    (ha : a ≠ 0) : a ^ (p - 1) = 1 :=
  ZMod.pow_card_sub_one_eq_one ha
```
