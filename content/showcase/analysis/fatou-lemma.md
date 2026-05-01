+++
title = "Fatou's Lemma"
description = "The integral of the liminf of nonnegative measurable functions is at most the liminf of their integrals"
weight = 70
tags = ["lean4-proof", "analysis", "visualization", "measure-theory"]
latex = "\\int \\liminf_{n} f_n \\, d\\mu \\leq \\liminf_{n} \\int f_n \\, d\\mu"
prerequisites = ["monotone-convergence"]
lean4_status = "complete"
+++

**Fatou's lemma** is one of the foundational inequalities in measure theory. It gives a lower bound on what integrals can do in the limit — and its power lies in requiring no domination hypothesis at all, only nonnegativity.

## Statement

Let $(X, \mathcal{M}, \mu)$ be a measure space and $(f_n)$ a sequence of nonneg­ative measurable functions $f_n : X \to [0,\infty]$. Then

$$\int_X \liminf_{n \to \infty} f_n \, d\mu \;\leq\; \liminf_{n \to \infty} \int_X f_n \, d\mu.$$

The inequality can be strict, and no condition on convergence of the $f_n$ is assumed.

## Visualization

**Canonical example: $f_n = n \cdot \mathbf{1}_{[0,1/n]}$ on $[0,1]$ with Lebesgue measure.**

| $n$ | $\int f_n \, d\lambda$ | pointwise $\liminf$ |
|-----|------------------------|---------------------|
| 1   | $1 \cdot 1 = 1$        | $0$ everywhere       |
| 2   | $2 \cdot \tfrac{1}{2} = 1$ | $0$           |
| 5   | $5 \cdot \tfrac{1}{5} = 1$ | $0$           |
| 10  | $10 \cdot \tfrac{1}{10} = 1$ | $0$         |
| 100 | $100 \cdot \tfrac{1}{100} = 1$ | $0$       |

Each $f_n$ is a rectangle of area $1$, but the rectangle shrinks to a point: $f_n(x) \to 0$ for every fixed $x > 0$ (and for $x = 0$ the sequence $f_n(0) = n \to \infty$, so the liminf at $0$ is $\infty$ — irrelevant, as it is a single point).

Therefore $\liminf_n f_n = 0$ a.e., giving

$$\int \liminf_n f_n \, d\lambda = 0 \;\leq\; 1 = \liminf_n \int f_n \, d\lambda.$$

This shows Fatou's inequality is **strict** here: mass escapes to zero width but nonzero height.

```
height
  |
n |   ##
  |   ##
  |   ##
  |   ##
  +---------> x
      0  1/n
```

As $n \to \infty$ the spike travels left and collapses, but each spike carries exactly area $1$.

## Proof Sketch

1. Define $g_n = \inf_{k \geq n} f_k$. Each $g_n$ is measurable, $g_n \leq f_n$, and $g_n \nearrow \liminf_n f_n$.

2. By monotonicity of integration: $\int g_n \leq \int f_n$, hence $\int g_n \leq \inf_{k \geq n} \int f_k$.

3. By the [[Monotone Convergence Theorem]] applied to $(g_n)$:

$$\int \liminf_n f_n = \int \lim_n g_n = \lim_n \int g_n = \liminf_n \int g_n \leq \liminf_n \int f_n.$$

## Connections

Fatou's lemma is the missing piece that turns the [[Monotone Convergence Theorem]] into the [[Dominated Convergence Theorem]]: DCT applies Fatou twice (to $f_n$ and to $g - f_n$) to sandwich the limit. It also appears in the proof of completeness of $L^1$ spaces, and in probability when proving lower semicontinuity of expected values.

## Lean4 Proof

```lean4
import Mathlib.MeasureTheory.Integral.Lebesgue.Add

open MeasureTheory Filter

/-- Fatou's lemma for the lower Lebesgue integral. -/
theorem fatou {α : Type*} {m : MeasurableSpace α} {μ : Measure α}
    {f : ℕ → α → ℝ≥0∞} (hf : ∀ n, Measurable (f n)) :
    ∫⁻ x, liminf (fun n => f n x) atTop ∂μ ≤
    liminf (fun n => ∫⁻ x, f n x ∂μ) atTop :=
  lintegral_liminf_le hf
```
