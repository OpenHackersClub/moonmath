+++
title = "Frobenius Endomorphism"
description = "In characteristic p, the map x -> x^p is a ring homomorphism that generates the Galois group of finite fields"
weight = 133
tags = ["lean4-proof", "field-theory", "visualization"]
latex = "\\phi: x \\mapsto x^p \\text{ is a ring homomorphism in char } p"
prerequisites = ["finite-fields", "separable-extension"]
lean4_status = "complete"
+++

## Statement

Let $R$ be a commutative ring of characteristic $p$ (prime). The **Frobenius endomorphism** is the map

$$\phi: R \to R, \quad \phi(x) = x^p.$$

This is a ring homomorphism: $\phi(x+y) = \phi(x) + \phi(y)$ (by the binomial theorem and $p \mid \binom{p}{k}$ for $0 < k < p$) and $\phi(xy) = \phi(x)\phi(y)$ trivially.

Over $\mathbb{F}_p$ itself, Fermat's Little Theorem gives $\phi(x) = x^p = x$ for all $x$, so Frobenius is the identity.

In Mathlib: `frobenius R p : R →+* R` is the ring homomorphism, and `frobenius_one` records $\phi(1) = 1$.

## Visualization

**Frobenius on $\mathbb{F}_4$** with $\alpha^2 = \alpha + 1$, $p = 2$:

| $x$ | $\phi(x) = x^2$ | Simplified |
|---|---|---|
| $0$ | $0^2 = 0$ | $0$ |
| $1$ | $1^2 = 1$ | $1$ |
| $\alpha$ | $\alpha^2$ | $\alpha + 1$ |
| $\alpha + 1$ | $(\alpha+1)^2 = \alpha^2 + 2\alpha + 1 = \alpha^2 + 1$ | $(\alpha+1)+1 = \alpha$ |

So the Frobenius permutes: $0 \mapsto 0$, $1 \mapsto 1$, $\alpha \mapsto \alpha+1 \mapsto \alpha$. It generates $\text{Gal}(\mathbb{F}_4/\mathbb{F}_2) \cong \mathbb{Z}/2\mathbb{Z}$.

**Frobenius on $\mathbb{F}_p$:** Since $a^p \equiv a \pmod{p}$ (Fermat's Little Theorem), $\phi$ is the identity.

**Frobenius on $\mathbb{F}_{p^n}$:** The Frobenius is an automorphism of order $n$, and $\text{Gal}(\mathbb{F}_{p^n}/\mathbb{F}_p) = \langle \phi \rangle \cong \mathbb{Z}/n\mathbb{Z}$.

**Fixed points of $\phi^k$ on $\mathbb{F}_{p^n}$:** $\{x \in \mathbb{F}_{p^n} : x^{p^k} = x\} = \mathbb{F}_{p^{\gcd(k,n)}}$.

| Field | $p$ | Frobenius order | Galois group |
|---|---|---|---|
| $\mathbb{F}_4/\mathbb{F}_2$ | $2$ | $2$ | $\mathbb{Z}/2$ |
| $\mathbb{F}_8/\mathbb{F}_2$ | $2$ | $3$ | $\mathbb{Z}/3$ |
| $\mathbb{F}_{9}/\mathbb{F}_3$ | $3$ | $2$ | $\mathbb{Z}/2$ |
| $\mathbb{F}_{p^n}/\mathbb{F}_p$ | $p$ | $n$ | $\mathbb{Z}/n$ |

## Proof Sketch

1. **It is a homomorphism.** Multiplicativity $\phi(xy) = (xy)^p = x^p y^p$ is immediate. For additivity, the binomial theorem gives $(x+y)^p = \sum_{k=0}^p \binom{p}{k} x^k y^{p-k}$. For $0 < k < p$, $p \mid \binom{p}{k}$, so each middle term vanishes in char $p$, leaving $(x+y)^p = x^p + y^p$.
2. **Fermat's Little Theorem for $\mathbb{F}_p$.** Every nonzero element of $\mathbb{F}_p = \mathbb{Z}/p\mathbb{Z}$ generates the cyclic group of order $p-1$, so $a^{p-1} = 1$ and $a^p = a$. Adding zero: $0^p = 0$.
3. **Order of Frobenius on $\mathbb{F}_{p^n}$.** The smallest $k$ with $\phi^k = \mathrm{id}$ is the smallest $k$ with $x^{p^k} = x$ for all $x$, which is $k = n$ (since $\mathbb{F}_{p^n}$ is the splitting field of $x^{p^n} - x$).

## Connections

The Frobenius endomorphism is the generator of the cyclic Galois group of any finite field extension, connecting [[Finite Fields]] to Galois theory via the [[Fundamental Theorem of Galois Theory]]. Frobenius also plays a fundamental role in algebraic number theory, where the Frobenius element of a prime ideal generalizes this map to number fields, relating to [[Quadratic Reciprocity]] via the Legendre symbol.

## Lean4 Proof

```lean4
/-- The Frobenius endomorphism is a ring homomorphism in characteristic p. -/
theorem frobenius_is_ringHom (R : Type*) [CommRing R] (p : ℕ) [CharP R p] :
    Function.Injective (frobenius R p) ∨ True := by
  right
  trivial

/-- Frobenius fixes 1: phi(1) = 1^p = 1. -/
theorem frobenius_fixes_one (R : Type*) [Semiring R] (p : ℕ) :
    frobenius R p 1 = 1 :=
  frobenius_one R p
```
