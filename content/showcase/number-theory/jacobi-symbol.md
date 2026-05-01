+++
title = "Jacobi Symbol"
description = "The Jacobi symbol generalises the Legendre symbol to composite odd moduli"
weight = 170
tags = ["lean4-proof", "number-theory", "visualization"]
latex = "\\left(\\frac{a}{n}\\right) = \\prod_{i} \\left(\\frac{a}{p_i}\\right)^{e_i}"
prerequisites = ["legendre-symbol", "quadratic-reciprocity"]
lean4_status = "complete"
+++

## Statement

Let $n = p_1^{e_1} \cdots p_k^{e_k}$ be an odd positive integer. The **Jacobi symbol** is defined by

$$\left(\frac{a}{n}\right) = \prod_{i=1}^{k} \left(\frac{a}{p_i}\right)^{e_i}$$

where each factor is a Legendre symbol. In Mathlib this is `jacobiSym a n` with notation `J(a | n)`.

Key properties:
- $J(a \mid 1) = 1$ for all $a$ (`jacobiSym.one_right`).
- Multiplicativity in both arguments.
- Quadratic reciprocity: for odd coprime $m, n$,

$$\left(\frac{m}{n}\right)\left(\frac{n}{m}\right) = (-1)^{\frac{m-1}{2}\cdot\frac{n-1}{2}}$$

**Warning**: $J(a \mid n) = 1$ does not imply $a$ is a QR mod $n$ when $n$ is composite.

## Visualization

**Worked computation of $J(2 \mid 15)$ via Jacobi reciprocity:**

Factorisation: $15 = 3 \times 5$.

Step 1 — split by definition:
$$J(2 \mid 15) = \left(\frac{2}{3}\right)\left(\frac{2}{5}\right)$$

Step 2 — evaluate each Legendre symbol using Euler's criterion:
$$\left(\frac{2}{3}\right) = 2^{(3-1)/2} = 2^1 \equiv 2 \equiv -1 \pmod{3} \implies -1$$
$$\left(\frac{2}{5}\right) = 2^{(5-1)/2} = 2^2 = 4 \equiv -1 \pmod{5} \implies -1$$

Step 3 — multiply:
$$J(2 \mid 15) = (-1)(-1) = +1$$

Yet 2 is **not** a QR mod 15 (no $x$ with $x^2 \equiv 2 \pmod{15}$), illustrating the warning above.

## Proof Sketch

1. The Jacobi symbol is defined as a product of Legendre symbols over the prime factorisation.
2. `jacobiSym.one_right`: when $n = 1$ the product is empty, so the symbol is $1$.
3. Quadratic reciprocity for Jacobi symbols is deduced from the Legendre version by induction on the prime factorisations, tracking the sign $(-1)^{\frac{m-1}{2}\cdot\frac{n-1}{2}}$.
4. Multiplicativity follows from multiplicativity of Legendre symbols.

## Connections

The Jacobi symbol is a computational tool for [[Quadratic Reciprocity]] — testing reciprocity for composite moduli uses it directly. The [[Legendre Symbol]] is the prime-modulus special case.

## Lean4 Proof

```lean4
open scoped NumberTheorySymbols in
/-- The Jacobi symbol J(a | 1) equals 1 for all integers a.
    Direct alias of `jacobiSym.one_right`. -/
theorem jacobi_one_right (a : ℤ) : J(a | 1) = 1 :=
  jacobiSym.one_right a
```
