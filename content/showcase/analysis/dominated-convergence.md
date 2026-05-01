+++
title = "Dominated Convergence Theorem"
description = "Pointwise convergence plus a uniform integrable dominating bound lets you pass the limit inside the integral"
weight = 80
tags = ["lean4-proof", "analysis", "visualization", "measure-theory"]
latex = "f_n \\to f \\text{ a.e.},\\; |f_n| \\leq g \\in L^1 \\Rightarrow \\int f_n \\to \\int f"
prerequisites = ["fatou-lemma", "monotone-convergence"]
lean4_status = "complete"
+++

The **Dominated Convergence Theorem** (DCT) is the principal tool for interchanging limits and Lebesgue integrals. The price of admission is a single integrable function that uniformly bounds the entire sequence — the dominating function.

## Statement

Let $(f_n)$ be a sequence of measurable functions on a measure space $(X, \mathcal{M}, \mu)$ such that:

1. $f_n \to f$ pointwise $\mu$-almost everywhere,
2. there exists $g \in L^1(\mu)$ with $|f_n(x)| \leq g(x)$ for a.e. $x$ and all $n$.

Then $f \in L^1(\mu)$ and

$$\lim_{n \to \infty} \int_X f_n \, d\mu = \int_X f \, d\mu, \qquad \text{equivalently} \quad \lim_{n\to\infty} \|f_n - f\|_{L^1} = 0.$$

## Visualization

**Example: $f_n(x) = \dfrac{nx}{1 + n^2 x^2}$ on $[0,1]$.**

Dominating function: $|f_n(x)| \leq \tfrac{1}{2}$ for all $x,n$ (AM–GM on $nx$ and $1/(nx)$), so $g(x) = \tfrac{1}{2}$ is integrable.

Pointwise limit: for each fixed $x > 0$, $f_n(x) = \tfrac{x}{x^2 + 1/n^2} \to 0$.

| $n$ | $\int_0^1 f_n \, d\lambda$ (exact) | max height $f_n$ |
|-----|------------------------------------|-------------------|
| 1   | $\tfrac{1}{2}\ln 2 \approx 0.347$  | $0.5$ at $x=1$   |
| 5   | $\tfrac{1}{2}\ln(26) / 5 \approx 0.130$ | $0.5$ at $x = 0.2$ |
| 10  | $\approx 0.068$                    | $0.5$ at $x = 0.1$ |
| 50  | $\approx 0.014$                    | $0.5$ at $x=0.02$ |
| 100 | $\approx 0.007$                    | $0.5$ at $x=0.01$ |

The peak of $f_n$ stays at height $\tfrac{1}{2}$ but migrates to $x = 1/n \to 0$, shrinking the area beneath it:

```
 0.5 |    *          (* = peak of f_n)
     |   * *
     |  *   *
  0  +-----------> x
     0  1/n  1
```

The integrals converge to $0 = \int f \, d\lambda$, consistent with DCT.

## Proof Sketch

1. The function $g + f_n \geq 0$ and $g - f_n \geq 0$, so [[Fatou's Lemma]] applies to both sequences.

2. Apply Fatou to $(g + f_n)$:
$$\int g + \int f \leq \liminf_n \int (g + f_n) = \int g + \liminf_n \int f_n.$$

3. Apply Fatou to $(g - f_n)$:
$$\int g - \int f \leq \liminf_n \int (g - f_n) = \int g - \limsup_n \int f_n.$$

4. Combining: $\limsup_n \int f_n \leq \int f \leq \liminf_n \int f_n$, so $\int f_n \to \int f$.

## Connections

DCT is often the last step in proving classical theorems of analysis: differentiating under the integral sign uses it to swap $\partial/\partial t$ and $\int$, and it drives the proof that $L^1$ convergence implies convergence of integrals in [[Fundamental Theorem of Calculus]] settings. In probability, it is used to justify computing $\mathbb{E}[\lim_n X_n]$ from $\lim_n \mathbb{E}[X_n]$ under uniform integrability. See also [[Fatou's Lemma]] for the one-sided predecessor and [[Monotone Convergence Theorem]] for the monotone variant with no domination required.

## Lean4 Proof

```lean4
import Mathlib.MeasureTheory.Integral.DominatedConvergence

open MeasureTheory Filter TopologicalSpace

/-- Dominated convergence theorem: pointwise a.e. convergence plus an integrable
    dominating bound implies convergence of integrals. -/
theorem dct {α G : Type*} [MeasurableSpace α] [NormedAddCommGroup G]
    [NormedSpace ℝ G] [CompleteSpace G]
    {μ : Measure α} {F : ℕ → α → G} {f : α → G} {bound : α → ℝ}
    (F_meas : ∀ n, AEStronglyMeasurable (F n) μ)
    (h_bound : ∀ n, ∀ᵐ x ∂μ, ‖F n x‖ ≤ bound x)
    (bound_integrable : Integrable bound μ)
    (h_lim : ∀ᵐ x ∂μ, Tendsto (fun n => F n x) atTop (𝓝 (f x))) :
    Tendsto (fun n => ∫ x, F n x ∂μ) atTop (𝓝 (∫ x, f x ∂μ)) :=
  tendsto_integral_of_dominated_convergence bound F_meas bound_integrable h_bound h_lim
```
