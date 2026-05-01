+++
title = "Legendre Symbol"
description = "The Legendre symbol (a|p) encodes whether a is a quadratic residue mod a prime p"
weight = 160
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\left(\\frac{a}{p}\\right) = a^{(p-1)/2} \\bmod p"
prerequisites = ["quadratic-residues", "fermats-little-theorem"]
lean4_status = "complete"
+++

## Statement

For an odd prime $p$ and integer $a$, the **Legendre symbol** $\left(\frac{a}{p}\right)$ (also written $\text{legendreSym}\; p\; a$ in Lean) is defined as

$$\left(\frac{a}{p}\right) = \begin{cases} 0 & p \mid a \\ 1 & a \text{ is a QR mod } p \\ -1 & a \text{ is a QNR mod } p \end{cases}$$

**Euler's criterion** gives a computable formula: for $p \nmid a$,

$$\left(\frac{a}{p}\right) \equiv a^{(p-1)/2} \pmod{p}$$

In Mathlib this is `legendreSym.eq_pow`: the integer value of $\text{legendreSym}\; p\; a$ cast into $\mathbb{Z}/p\mathbb{Z}$ equals $(a \bmod p)^{p/2}$.

## Visualization

**Legendre symbol $\left(\frac{a}{7}\right)$ for $p = 7$, $a = 1,\ldots,6$:**

| $a$ | $a^3 \bmod 7$ | $\left(\frac{a}{7}\right)$ |
|-----|--------------|--------------------------|
| 1   | 1            | $+1$                     |
| 2   | 1            | $+1$  (2 is QR: $3^2=9\equiv 2$) |
| 3   | 6 $\equiv -1$| $-1$  (3 is QNR)         |
| 4   | 1            | $+1$  (4 $= 2^2$)        |
| 5   | 6 $\equiv -1$| $-1$  (5 is QNR)         |
| 6   | 6 $\equiv -1$| $-1$  (6 is QNR)         |

Check: QRs mod 7 are $\{1, 2, 4\}$ — three values, which is $(7-1)/2 = 3$. The product of all six symbols is $(-1)^3 = -1$.

## Proof Sketch

1. The group $(\mathbb{Z}/p\mathbb{Z})^\times$ is cyclic of order $p-1$ by [[Primitive Roots]]. Let $g$ be a generator; write $a = g^k$.
2. $a$ is a QR iff $k$ is even. The map $a \mapsto a^{(p-1)/2}$ sends $g^k$ to $g^{k(p-1)/2}$. Since $g^{p-1} = 1$, this is $1$ when $k$ is even and $-1$ when $k$ is odd.
3. This coincides with the Legendre symbol, establishing Euler's criterion.
4. Multiplicativity $\left(\frac{ab}{p}\right) = \left(\frac{a}{p}\right)\left(\frac{b}{p}\right)$ follows from the same power-map argument.

## Connections

The Legendre symbol is the building block of [[Quadratic Reciprocity]], which relates $\left(\frac{p}{q}\right)$ and $\left(\frac{q}{p}\right)$ for distinct odd primes. The generalisation to composite moduli is the [[Jacobi Symbol]].

## Lean4 Proof

```lean4
/-- Euler's criterion in Mathlib: the cast of `legendreSym p a` into `ZMod p`
    equals `(a : ZMod p) ^ (p / 2)`. -/
theorem legendre_euler_criterion (p : ℕ) [Fact p.Prime] (a : ℤ) :
    (legendreSym p a : ZMod p) = (a : ZMod p) ^ (p / 2) :=
  legendreSym.eq_pow p a
```
