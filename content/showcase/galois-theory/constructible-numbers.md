+++
title = "Constructible Numbers"
description = "Which regular n-gons can be constructed with compass and straightedge?"
weight = 30
tags = ["lean4-proof", "galois-theory", "geometry", "visualization"]
latex = "\\text{Regular } n\\text{-gon constructible} \\iff n = 2^a \\cdot p_1 \\cdot p_2 \\cdots p_k"
prerequisites = []
lean4_status = "sorry"
+++

## Statement (Gauss-Wantzel Theorem)

A regular $n$-gon is constructible with compass and straightedge if and only if:

$$n = 2^a \cdot p_1 \cdot p_2 \cdots p_k$$

where $a \geq 0$ and $p_1, p_2, \ldots, p_k$ are distinct Fermat primes (primes of the form $2^{2^m} + 1$).

The known Fermat primes are $3, 5, 17, 257, 65537$.

## Connection to Galois Theory

Constructibility of $\cos(2\pi/n)$ is equivalent to the degree $[\mathbb{Q}(\cos(2\pi/n)) : \mathbb{Q}]$ being a power of 2. Via the [[Fundamental Theorem of Galois Theory|Galois correspondence]], this reduces to the Galois group $\text{Gal}(\mathbb{Q}(\zeta_n)/\mathbb{Q}) \cong (\mathbb{Z}/n\mathbb{Z})^*$ having a composition series with all factors of order 2.

## Classical Problems Resolved

- **Doubling the cube:** Impossible — requires constructing $\sqrt[3]{2}$, which has degree 3 over $\mathbb{Q}$
- **Trisecting an angle:** Impossible in general — trisecting $60°$ requires solving a cubic
- **Squaring the circle:** Impossible — $\pi$ is transcendental (Lindemann, 1882)

## Connections

The proof uses the [[Fundamental Theorem of Galois Theory]] to translate geometric constructibility into group theory. The notion of Fermat primes connects to [[Infinitude of Primes|prime numbers]] in number theory.

## Lean4 Proof

```lean4
/-- A Fermat prime is a prime of the form 2^(2^m) + 1. -/
def IsFermatPrime (p : Nat) : Prop :=
  Nat.Prime p ∧ ∃ m : Nat, p = 2 ^ (2 ^ m) + 1

/-- Gauss-Wantzel theorem: a regular n-gon is constructible with compass
    and straightedge iff n = 2^a · p₁ · p₂ · ... · pₖ where each pᵢ
    is a distinct Fermat prime. -/
theorem gauss_wantzel (n : Nat) (hn : n ≥ 3) :
    Constructible (regularNgon n) ↔
      ∃ (a : Nat) (ps : Finset Nat),
        (∀ p ∈ ps, IsFermatPrime p) ∧
        ps.card = ps.toList.length ∧  -- all distinct
        n = 2 ^ a * ps.prod := by
  sorry -- requires showing [Q(ζ_n):Q] is a power of 2 iff the condition holds
```
