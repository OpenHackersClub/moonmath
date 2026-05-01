+++
title = "Primitive Roots"
description = "A primitive root mod p is a generator of the cyclic group (Z/pZ)*, existing for every prime p"
weight = 180
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "g \\text{ is a primitive root mod } p \\iff \\{g^0, g^1, \\ldots, g^{p-2}\\} = (\\mathbb{Z}/p\\mathbb{Z})^\\times"
prerequisites = ["fermats-little-theorem", "eulers-totient"]
lean4_status = "complete"
+++

## Statement

An element $g \in (\mathbb{Z}/p\mathbb{Z})^\times$ is a **primitive root** mod $p$ (a generator) if every element of $(\mathbb{Z}/p\mathbb{Z})^\times$ is a power of $g$, i.e.\ the multiplicative order of $g$ equals $p - 1$.

**Theorem.** For every prime $p$, the group $(\mathbb{Z}/p\mathbb{Z})^\times$ is cyclic, so primitive roots exist.

In Mathlib the cyclicity is an instance: `IsCyclic (ZMod p)ˣ`, proved via `ZMod.isCyclic_units_prime`. `IsCyclic.exists_generator` then yields a concrete generator.

## Visualization

**Primitive roots mod 11** (the full group $(\mathbb{Z}/11\mathbb{Z})^\times$ has order 10):

| $g$ | Powers $g^1,\ldots,g^{10} \bmod 11$ | Generator? |
|-----|--------------------------------------|-----------|
| 2   | 2,4,8,5,10,9,7,3,6,1                | Yes       |
| 3   | 3,9,5,4,1,…                         | No (order 5) |
| 6   | 6,3,7,9,10,5,8,4,2,1                | Yes       |
| 7   | 7,5,2,3,10,4,6,9,8,1                | Yes       |

Primitive roots mod 11: $\{2, 6, 7, 8\}$ — there are $\phi(10) = 4$ of them.

**Primitive roots mod 13** (order 12):

Primitive roots: $\{2, 6, 7, 11\}$ — there are $\phi(12) = 4$ of them.

**Primitive roots mod 17** (order 16):

Primitive roots: $\{3, 5, 6, 7, 10, 11, 12, 14\}$ — there are $\phi(16) = 8$ of them.

In general, if primitive roots exist mod $n$, there are exactly $\phi(\phi(n))$ of them.

## Proof Sketch

1. $(\mathbb{Z}/p\mathbb{Z})^\times$ is a finite abelian group of order $p-1$.
2. By the structure theorem for finite abelian groups, it decomposes into cyclic factors. One shows (using the polynomial $X^d - 1$ having at most $d$ roots in $\mathbb{Z}/p\mathbb{Z}$) that the group is cyclic.
3. Any generator $g$ is a primitive root. Since $g$ generates a cyclic group of order $p-1$, every element appears among $g^0, g^1, \ldots, g^{p-2}$.
4. The number of generators of a cyclic group of order $n$ is $\phi(n)$ by [[Euler's Totient Function]].

## Connections

Primitive roots underpin [[Discrete Logarithm]] — the discrete log of $a$ base $g$ is the unique $k$ with $g^k = a$. They also appear in the proof of [[Quadratic Reciprocity]] via Gauss sums.

## Lean4 Proof

```lean4
/-- The unit group of ZMod p is cyclic for any prime p.
    This is `ZMod.isCyclic_units_prime` in Mathlib. -/
theorem zmod_prime_units_cyclic (p : ℕ) (hp : p.Prime) : IsCyclic (ZMod p)ˣ :=
  ZMod.isCyclic_units_prime hp

/-- A cyclic group has a generator. -/
theorem primitive_root_exists (p : ℕ) (hp : p.Prime) :
    ∃ g : (ZMod p)ˣ, ∀ x : (ZMod p)ˣ, x ∈ Subgroup.zpowers g := by
  haveI : IsCyclic (ZMod p)ˣ := ZMod.isCyclic_units_prime hp
  obtain ⟨g, hg⟩ := IsCyclic.exists_generator (α := (ZMod p)ˣ)
  exact ⟨g, hg⟩
```
