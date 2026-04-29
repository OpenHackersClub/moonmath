+++
title = "Quadratic Reciprocity"
description = "Relationship between solvability of quadratic congruences for two odd primes"
weight = 40
difficulty = "advanced"
tags = ["number-theory", "modular-arithmetic"]
latex = "\\left(\\frac{p}{q}\\right)\\left(\\frac{q}{p}\\right) = (-1)^{\\frac{p-1}{2}\\cdot\\frac{q-1}{2}}"
prerequisites = ["fermats-little-theorem"]
lean4_status = "complete"
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
/-- Quadratic reciprocity for distinct odd primes. The Legendre symbol
    `legendreSym p q` is Mathlib's `(a/p)` defined via Euler's
    criterion. For odd `p`, the integer division `p / 2` equals
    `(p-1) / 2`, so the exponent matches the textbook form
    `((p-1)/2) · ((q-1)/2)`. -/
theorem quadratic_reciprocity {p q : ℕ} [Fact p.Prime] [Fact q.Prime]
    (hp : p ≠ 2) (hq : q ≠ 2) (hpq : p ≠ q) :
    legendreSym q p * legendreSym p q = (-1) ^ (p / 2 * (q / 2)) :=
  legendreSym.quadratic_reciprocity hp hq hpq
```
