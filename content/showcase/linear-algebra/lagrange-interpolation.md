+++
title = "Lagrange Interpolation"
description = "Given n distinct nodes, there is a unique polynomial of degree < n passing through all n points."
weight = 60
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "p(x) = \\sum_{i} r_i \\prod_{j \\neq i} \\frac{x - x_j}{x_i - x_j}"
prerequisites = ["rank-nullity"]
lean4_status = "complete"
+++

## Statement

Given $n$ distinct nodes $x_0, x_1, \ldots, x_{n-1}$ in a field $F$ and prescribed values $r_0, \ldots, r_{n-1} \in F$, there exists a **unique** polynomial $p \in F[X]$ of degree $< n$ satisfying $p(x_i) = r_i$ for all $i$.

The Lagrange basis polynomial for node $x_i$ is

$$\ell_i(x) = \prod_{j \neq i} \frac{x - x_j}{x_i - x_j}$$

and the interpolating polynomial is

$$p(x) = \sum_{i=0}^{n-1} r_i \,\ell_i(x).$$

## Visualization

Interpolate through three points $(0, 1)$, $(1, 2)$, $(2, 5)$:

| $i$ | node $x_i$ | value $r_i$ | basis $\ell_i(x)$ |
|-----|------------|-------------|-------------------|
| 0   | 0          | 1           | $\frac{(x-1)(x-2)}{(0-1)(0-2)} = \tfrac{1}{2}(x-1)(x-2)$ |
| 1   | 1          | 2           | $\frac{(x-0)(x-2)}{(1-0)(1-2)} = -(x)(x-2)$ |
| 2   | 2          | 5           | $\frac{(x-0)(x-1)}{(2-0)(2-1)} = \tfrac{1}{2}x(x-1)$ |

Summing: $p(x) = \tfrac{1}{2}(x-1)(x-2) - 2x(x-2) + \tfrac{5}{2}x(x-1)$

Expanding term-by-term and collecting:

$$p(x) = x^2 + 1$$

Verification: $p(0)=1$, $p(1)=2$, $p(2)=5$. Degree $= 2 < 3 = n$. Unique.

## Proof Sketch

1. **Existence.** Form $p = \sum_i r_i \ell_i$. By construction $\ell_i(x_j) = \delta_{ij}$, so $p(x_i) = r_i$. Each $\ell_i$ has degree $n-1$, so $\deg p < n$.
2. **Uniqueness.** Suppose $q$ also satisfies the conditions and $\deg q < n$. Then $p - q$ vanishes at all $n$ nodes and has degree $< n$, so a polynomial of degree $< n$ with $n$ roots must be the zero polynomial.
3. **Linear map.** The assignment $r \mapsto \text{interpolate}(r)$ is an $F$-linear map; the Lagrange basis $\{\ell_i\}$ is a basis for the space of polynomials of degree $< n$.

## Connections

Lagrange interpolation generalizes [[Vieta's Formulas]] (which read off coefficients from roots) and underpins [[Taylor's Theorem]] (the limit as nodes coalesce). The uniqueness argument is the same dimension count as [[Rank–Nullity Theorem]].

## Lean4 Proof

```lean4
-- Mathlib: Mathlib.LinearAlgebra.Lagrange
-- `Lagrange.interpolate s v r` is the unique polynomial of degree < #s
-- interpolating r at the nodes v.
-- We state the two key properties: correct values and degree bound.
open Lagrange in
theorem lagrange_eval_node (F : Type*) [Field F] {ι : Type*} [DecidableEq ι]
    (s : Finset ι) (v : ι → F) (r : ι → F)
    (hvs : Set.InjOn v s) (hi : ∀ i ∈ s, True) (i : ι) (his : i ∈ s) :
    (interpolate s v r).eval (v i) = r i :=
  eval_interpolate_at_node r hvs his

open Lagrange in
theorem lagrange_degree_lt (F : Type*) [Field F] {ι : Type*} [DecidableEq ι]
    (s : Finset ι) (v : ι → F) (r : ι → F) (hvs : Set.InjOn v s) :
    (interpolate s v r).degree < s.card :=
  degree_interpolate_lt r hvs
```