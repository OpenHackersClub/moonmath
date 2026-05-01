+++
title = "Fundamental Theorem of Algebra"
description = "Every non-constant polynomial with complex coefficients has at least one root in ℂ"
weight = 50
tags = ["lean4-proof", "analysis", "visualization"]
latex = "\\deg p \\geq 1,\\; p \\in \\mathbb{C}[z] \\implies \\exists z_0 \\in \\mathbb{C}: p(z_0) = 0"
prerequisites = []
lean4_status = "complete"
+++

The **Fundamental Theorem of Algebra** (FTA) makes $\mathbb{C}$ algebraically closed: every non-constant polynomial with complex coefficients splits completely over $\mathbb{C}$. This is the reason that eigenvalue theory, spectral analysis, and partial-fraction decomposition all work without restriction over the complex numbers.

## Statement

Let $p \in \mathbb{C}[z]$ be a polynomial of degree $d \geq 1$. Then there exists $z_0 \in \mathbb{C}$ such that $p(z_0) = 0$.

By induction, $p$ factors completely:

$$p(z) = a_d \prod_{k=1}^{d} (z - z_k), \qquad z_1, \ldots, z_d \in \mathbb{C}.$$

## Visualization

Factoring over $\mathbb{C}$ versus $\mathbb{R}$ — explicit examples:

**$p(z) = z^4 - 1$.**  Over $\mathbb{R}$: $z^4 - 1 = (z^2 - 1)(z^2 + 1) = (z-1)(z+1)(z^2+1)$, and $z^2 + 1$ is irreducible over $\mathbb{R}$.  Over $\mathbb{C}$: $z^2 + 1 = (z - i)(z + i)$, giving the complete factorisation

$$z^4 - 1 = (z-1)(z+1)(z-i)(z+i).$$

The four roots $\{1, -1, i, -i\}$ lie at the vertices of the unit square in $\mathbb{C}$.

**$p(z) = z^2 + 1$.**  Roots: $z = \pm i$.  Real axis: no root.

**$p(z) = z^3 - 1$.**  Roots: $1,\, \omega,\, \omega^2$ where $\omega = e^{2\pi i/3} = -\tfrac{1}{2} + \tfrac{\sqrt{3}}{2}i$.

```
  Im
   i ──── × (i, root of z²+1)
   |
───+────────── Re
   |
  -i ──── × (-i, root of z²+1)
```

The unit circle $|z| = 1$ contains all roots of unity — the FTA guarantees their existence; complex analysis pinpoints their location.

## Proof Sketch

There are several proofs. The slickest uses [[Liouville's Theorem]]:

Suppose $p$ has no root in $\mathbb{C}$. Then $f(z) = 1/p(z)$ is entire. Because $|p(z)| \to \infty$ as $|z| \to \infty$ (a degree-$d$ polynomial grows like $|a_d||z|^d$), $f(z) \to 0$ at infinity, so $f$ is bounded. By Liouville, $f$ is constant, contradicting the assumption that $p$ is non-constant.

## Connections

The FTA is the endpoint of a path starting with completeness: the [[Cauchy Criterion]] ensures $\mathbb{C}$ is complete; complex differentiability is rigid enough for the [[Liouville's Theorem|Liouville]] argument; and together they yield algebraic closure. From the other direction, the FTA implies that every endomorphism of a finite-dimensional complex vector space has an eigenvalue — the starting point for the Jordan normal form. The explicit factorisation of $z^n - 1$ via roots of unity connects to [[Mandelbrot Set|complex iteration]] (the $n$-th roots of unity are fixed points of $z \mapsto z^n$).

## Lean4 Proof

```lean4
import Mathlib.Analysis.Complex.Polynomial.Basic

open Polynomial

/-- Fundamental Theorem of Algebra: every non-constant polynomial over ℂ has a root. -/
theorem fta {p : ℂ[X]} (h : 0 < degree p) : ∃ z : ℂ, IsRoot p z :=
  Complex.exists_root h
```
