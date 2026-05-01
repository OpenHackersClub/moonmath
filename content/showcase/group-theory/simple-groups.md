+++
title = "Simple Groups"
description = "A simple group has no proper nontrivial normal subgroups; the atoms of group composition"
weight = 180
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "G \\text{ simple} \\Longleftrightarrow \\forall N \\unlhd G,\\; N = 1 \\text{ or } N = G"
prerequisites = ["lagrange-theorem", "first-isomorphism-theorem"]
lean4_status = "complete"
+++

## Statement

A group $G$ is **simple** if it is nontrivial and its only normal subgroups are $\{1\}$ and $G$ itself. Simple groups are the building blocks of all finite groups in the same way primes are the building blocks of integers.

**Key examples:**
- $\mathbb{Z}/p\mathbb{Z}$ is simple for every prime $p$ (the only finite abelian simple groups).
- $A_n$ is simple for all $n \geq 5$ (the only nonabelian simple groups in the alternating series).
- The Monster group $\mathbb{M}$ has order $\approx 8 \times 10^{53}$ and is simple.

## Visualization

Comparison of $A_4$ (not simple) and $A_5$ (simple):

```
A_4   (order 12) — NOT simple
  Normal subgroup: V_4 = {(), (12)(34), (13)(24), (14)(23)}
  Subgroup lattice:
    A_4
    ├── V_4  (normal, order 4)
    │    ├── ⟨(12)(34)⟩  (order 2)
    │    ├── ⟨(13)(24)⟩  (order 2)
    │    └── ⟨(14)(23)⟩  (order 2)
    ├── ⟨(123)⟩  (order 3, not normal)
    └── {()}     (order 1)

A_5   (order 60) — SIMPLE
  Conjugacy classes:  1 + 15 + 20 + 12 + 12 = 60
  (sizes must divide 60 and normal subgroups are unions of conjugacy classes + {e})
  No subset of {15, 20, 12, 12} sums to 59 → no normal subgroup of order 2–59.
```

For $\mathbb{Z}/p\mathbb{Z}$ with $p = 5$: the only subgroups have order dividing 5 (by Lagrange), so orders $1$ and $5$. Both are trivial or the whole group — hence simple.

## Proof Sketch

Proof that $\mathbb{Z}/p\mathbb{Z}$ is simple (for prime $p$):

1. By [[Lagrange's Theorem]], any subgroup $H \leq \mathbb{Z}/p\mathbb{Z}$ has $|H|$ dividing $p$.
2. Since $p$ is prime, $|H| \in \{1, p\}$, so $H = \{0\}$ or $H = \mathbb{Z}/p\mathbb{Z}$.
3. $\mathbb{Z}/p\mathbb{Z}$ is abelian, so every subgroup is normal.
4. Therefore $\mathbb{Z}/p\mathbb{Z}$ has no proper nontrivial normal subgroup — it is simple.

Mathlib handles this via `isSimpleGroup_of_prime_card`: a group of prime order is simple (since its cardinality is prime, Lagrange forces subgroup orders to be $1$ or $p$).

## Connections

Simple groups are the Jordan-Hölder factors in the composition series of any finite group, analogous to prime factors in the [[Fundamental Theorem of Arithmetic]]. The classification of finite simple groups (CFSG) is one of the deepest results in mathematics. $A_5$ being simple is the key step in proving the [[Impossibility of the Quintic Formula]] — the absence of a solvable composition series for $A_5$ prevents radical expressions for quintic roots. The [[Sylow Theorems]] are the primary tool for proving a given group is NOT simple.

## Lean4 Proof

```lean4
import Mathlib.GroupTheory.SpecificGroups.Cyclic

/-- ℤ/pℤ is a simple group for any prime p.
    Mathlib: `isSimpleGroup_of_prime_card` in
    `Mathlib.GroupTheory.SpecificGroups.Cyclic`. -/
theorem zmod_prime_simple (p : ℕ) [hp : Fact p.Prime] :
    IsSimpleGroup (ZMod p) :=
  isSimpleGroup_of_prime_card (by simp [ZMod.card])
```
