+++
title = "Absolutely Continuous Functions"
description = "A function is absolutely continuous iff it is the integral of its derivative — the bridge between differentiation and Lebesgue integration"
weight = 110
tags = ["lean4-proof", "analysis", "visualization", "measure-theory"]
latex = "F(x) = F(a) + \\int_a^x f(t) \\, dt \\iff F \\text{ absolutely continuous}"
prerequisites = ["lebesgue-differentiation", "dominated-convergence"]
lean4_status = "complete"
+++

**Absolute continuity** is the property that distinguishes functions arising as Lebesgue integrals from the larger class of functions of bounded variation. The Radon-Nikodym theorem gives the precise measure-theoretic statement: $\nu \ll \mu$ if and only if $\nu$ has a density with respect to $\mu$.

## Statement

A function $F : [a,b] \to \mathbb{R}$ is **absolutely continuous** if for every $\varepsilon > 0$ there exists $\delta > 0$ such that for any finite collection of disjoint subintervals $(a_i, b_i) \subset [a,b]$,

$$\sum_i (b_i - a_i) < \delta \implies \sum_i |F(b_i) - F(a_i)| < \varepsilon.$$

**Fundamental theorem of Lebesgue integration**: $F$ is absolutely continuous on $[a,b]$ if and only if there exists $f \in L^1([a,b])$ such that

$$F(x) = F(a) + \int_a^x f(t)\,dt \quad \text{for all } x \in [a,b].$$

In this case $F' = f$ almost everywhere.

**Radon-Nikodym theorem** (measure-theoretic form): If $\mu$ and $\nu$ are $\sigma$-finite measures and $\nu \ll \mu$ (meaning $\mu(A) = 0 \Rightarrow \nu(A) = 0$), then there exists a measurable function $f : X \to [0,\infty]$ such that $\nu = \mu$-with-density $f$, i.e.,

$$\nu(A) = \int_A f \, d\mu \quad \text{for all measurable } A.$$

## Visualization

**Cantor function vs. an AC function — comparison:**

The **Cantor function** $C : [0,1] \to [0,1]$ is continuous, monotone, $C' = 0$ a.e. (it is constant on each removed interval), yet $C(0) = 0$ and $C(1) = 1$. It is **not** absolutely continuous.

The AC function $A(x) = \int_0^x \mathbf{1}_{[0,1/2]}(t)\,dt$ satisfies $A' = \mathbf{1}_{[0,1/2]}$ a.e.

| $x$ | Cantor $C(x)$ | $A(x) = \int_0^x \mathbf{1}_{[0,1/2]}$ | $A'(x)$ |
|-----|---------------|------------------------------------------|----------|
| $0$   | $0$     | $0$          | $1$   |
| $1/4$ | $1/4$   | $1/4$        | $1$   |
| $1/3$ | $1/2$   | $1/3$        | $1$   |
| $1/2$ | $1/2$   | $1/2$        | jumps |
| $2/3$ | $1/2$   | $1/2$        | $0$   |
| $3/4$ | $3/4$   | $1/2$        | $0$   |
| $1$   | $1$     | $1/2$        | $0$   |

The Cantor function "hides" its total variation on a set of measure zero (the Cantor set). AC functions cannot do this.

**Why AC fails for Cantor:** Take $n$ disjoint intervals of total length $\delta$ from the complement of the Cantor set. The function changes by at most $\delta$ total — fine. But intervals _inside_ the Cantor set (measure zero, but uncountably many) account for the entire variation from $0$ to $1$.

```
Variation budget:   [ AC function: proportional to length ]
                    [ Cantor function: can be 1 with total length → 0 ]
```

## Proof Sketch

1. (**If** $F = F(a) + \int_a^x f$): Given $\varepsilon$, set $\delta = \varepsilon / (2\|f\|_{L^1})$... wait, use uniform integrability of $f$: since $f \in L^1$, for any $\varepsilon$ there is $\delta$ with $\int_E |f| < \varepsilon$ whenever $\lambda(E) < \delta$. Then $\sum |F(b_i) - F(a_i)| \leq \int_{\bigcup (a_i,b_i)} |f| < \varepsilon$.

2. (**Only if**): If $F$ is AC, define $\nu(A) = \int_A dF$ (the Lebesgue-Stieltjes measure). Then AC implies $\nu \ll \lambda$ (Lebesgue measure). By Radon-Nikodym, $\nu = \lambda$-with-density $f$ for some $f \in L^1$, giving $F(x) - F(a) = \int_a^x f$.

3. Differentiating via the [[Lebesgue Differentiation Theorem]] recovers $F' = f$ a.e.

## Connections

The Radon-Nikodym theorem is the measure-theoretic backbone of conditional expectation in probability (see [[Bayes' Theorem]]) and of the Hahn decomposition. The characterisation of AC functions is the precise form of the [[Fundamental Theorem of Calculus]] for Lebesgue integration. Singular functions like the Cantor function illustrate that [[Monotone Convergence Theorem]] arguments can fail unless absolute continuity is verified.

## Lean4 Proof

```lean4
import Mathlib

open MeasureTheory Measure

/-- Radon-Nikodym theorem: if ν ≪ μ (absolutely continuous), then ν has a
    measurable density (Radon-Nikodym derivative) with respect to μ. -/
theorem radon_nikodym {α : Type*} [MeasurableSpace α]
    {μ ν : Measure α} [SigmaFinite μ] [HaveLebesgueDecomposition ν μ]
    (h : ν ≪ μ) :
    ν = μ.withDensity (ν.rnDeriv μ) :=
  (withDensity_rnDeriv_eq ν μ h).symm
```
