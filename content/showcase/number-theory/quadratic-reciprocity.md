+++
title = "Quadratic Reciprocity"
description = "Relationship between solvability of quadratic congruences for two odd primes"
weight = 40
difficulty = "advanced"
tags = ["number-theory", "modular-arithmetic"]
latex = "\\left(\\frac{p}{q}\\right)\\left(\\frac{q}{p}\\right) = (-1)^{\\frac{p-1}{2}\\cdot\\frac{q-1}{2}}"
prerequisites = ["fermats-little-theorem"]
lean4_status = "sorry"
+++

## Statement

For distinct odd primes $p$ and $q$:

$$\left(\frac{p}{q}\right)\left(\frac{q}{p}\right) = (-1)^{\frac{p-1}{2}\cdot\frac{q-1}{2}}$$

where $\left(\frac{a}{p}\right)$ is the Legendre symbol — equal to $1$ if $a$ is a quadratic residue mod $p$, and $-1$ otherwise.

In words: $p$ is a square mod $q$ and $q$ is a square mod $p$, unless both $p \equiv q \equiv 3 \pmod{4}$, in which case exactly one is a square mod the other.

## Supplements

The **first supplement** states:
$$\left(\frac{-1}{p}\right) = (-1)^{\frac{p-1}{2}}$$

The **second supplement** states:
$$\left(\frac{2}{p}\right) = (-1)^{\frac{p^2 - 1}{8}}$$

## Historical Note

Gauss called this the "golden theorem" (*theorema aureum*) and published six different proofs during his lifetime. Over 240 proofs are now known, using techniques from combinatorics, algebra, analysis, and algebraic geometry.

## Connections

The proof relies on [[Fermat's Little Theorem]]. Some modern proofs use ideas from [[Fundamental Theorem of Galois Theory|Galois theory]].

## Lean4 Proof

```lean4
/-- Legendre symbol: 1 if a is a quadratic residue mod p, -1 otherwise. -/
def legendreSymbol (a : Int) (p : Nat) (hp : Nat.Prime p) (hp2 : p ≠ 2) : Int :=
  sorry -- defined via Euler's criterion: a^((p-1)/2) mod p

/-- Quadratic reciprocity: (p/q)(q/p) = (-1)^((p-1)/2 · (q-1)/2). -/
theorem quadratic_reciprocity (p q : Nat)
    (hp : Nat.Prime p) (hq : Nat.Prime q)
    (hp2 : p ≠ 2) (hq2 : q ≠ 2) (hpq : p ≠ q) :
    legendreSymbol (↑p) q hq hq2 * legendreSymbol (↑q) p hp hp2 =
      (-1) ^ ((p - 1) / 2 * ((q - 1) / 2)) := by
  sorry -- Gauss's "golden theorem"
```
