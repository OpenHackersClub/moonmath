+++
title = "Fubini's Theorem"
description = "For integrable functions on a product measure space the iterated integrals in either order both equal the double integral"
weight = 90
tags = ["lean4-proof", "analysis", "visualization", "measure-theory"]
latex = "\\iint f \\, d(\\mu \\otimes \\nu) = \\int\\!\\int f(x,y) \\, d\\nu(y) \\, d\\mu(x)"
prerequisites = ["dominated-convergence", "monotone-convergence"]
lean4_status = "complete"
+++

**Fubini's theorem** justifies the everyday manipulation of swapping the order of integration. The price: integrability of $f$ on the product space. Tonelli's theorem is the nonneg­ative version that requires no integrability hypothesis.

## Statement

Let $(X, \mathcal{M}, \mu)$ and $(Y, \mathcal{N}, \nu)$ be sigma-finite measure spaces, and let $f : X \times Y \to \mathbb{R}$ be integrable with respect to the product measure $\mu \otimes \nu$. Then:

1. For $\mu$-almost every $x$, the slice $y \mapsto f(x,y)$ is $\nu$-integrable.
2. The function $x \mapsto \int_Y f(x,y)\,d\nu(y)$ is $\mu$-integrable.
3. The iterated integrals agree with the product integral:

$$\int_{X \times Y} f \, d(\mu \otimes \nu) = \int_X \left(\int_Y f(x,y)\,d\nu(y)\right) d\mu(x) = \int_Y \left(\int_X f(x,y)\,d\mu(x)\right) d\nu(y).$$

## Visualization

**Compute $\iint_{[0,1]^2} xy \, dA$ both ways.**

Order 1 — integrate $y$ first:

$$\int_0^1 \left(\int_0^1 xy \, dy\right) dx = \int_0^1 x \cdot \left[\frac{y^2}{2}\right]_0^1 dx = \int_0^1 \frac{x}{2}\,dx = \frac{1}{4}.$$

Order 2 — integrate $x$ first:

$$\int_0^1 \left(\int_0^1 xy \, dx\right) dy = \int_0^1 y \cdot \left[\frac{x^2}{2}\right]_0^1 dy = \int_0^1 \frac{y}{2}\,dy = \frac{1}{4}.$$

Both equal $\tfrac{1}{4}$, confirming Fubini for this case.

**Slice picture for $f(x,y) = xy$:**

```
y
1 |  (higher f here)
  |  **
  | * *
  |*   *
0 +--------> x
  0         1

Each horizontal slice (fixed y) gives integral = y/2.
Integrating y/2 over [0,1] gives 1/4.
```

**When Fubini fails (warning):** For $f(x,y) = (x^2 - y^2)/(x^2+y^2)^2$ on $[0,1]^2$ the two iterated integrals give $+\pi/4$ and $-\pi/4$: $f$ is not integrable on the product, violating the hypothesis.

| Computation | Value |
|-------------|-------|
| $\int_0^1 \int_0^1 xy\,dy\,dx$ | $1/4$ |
| $\int_0^1 \int_0^1 xy\,dx\,dy$ | $1/4$ |
| $\int_0^1 \int_0^1 \frac{x^2-y^2}{(x^2+y^2)^2}\,dy\,dx$ | $\pi/4$ |
| $\int_0^1 \int_0^1 \frac{x^2-y^2}{(x^2+y^2)^2}\,dx\,dy$ | $-\pi/4$ |

## Proof Sketch

1. Verify for indicator functions $f = \mathbf{1}_{A \times B}$ where $A$ is $\mu$-measurable and $B$ is $\nu$-measurable — iterated integrals both give $\mu(A)\nu(B)$.

2. Extend by linearity to simple functions (finite linear combinations of such indicators).

3. Use [[Monotone Convergence Theorem]] to extend to nonneg­ative measurable functions (Tonelli's theorem).

4. Split $f = f^+ - f^-$ and apply Tonelli to each part. Integrability ensures both parts are finite, allowing subtraction.

## Connections

Fubini's theorem is used constantly in probability (joint distributions, conditional expectations) and in analysis wherever a two-parameter family of integrals appears. It underlies the convolution formula $\int (f * g)(x) \, dx = \int f \cdot \int g$ and the proof of the [[Fundamental Theorem of Calculus]] for Lebesgue integrals via slicing. The [[Dominated Convergence Theorem]] is typically used in its proof to justify limit interchanges on slice integrals.

## Lean4 Proof

```lean4
import Mathlib.MeasureTheory.Integral.Prod

open MeasureTheory

/-- Fubini's theorem: the Bochner integral over a product measure equals
    the iterated integral, integrating the second variable first. -/
theorem fubini {α β E : Type*} [MeasurableSpace α] [MeasurableSpace β]
    {μ : Measure α} {ν : Measure β} [SFinite ν]
    [NormedAddCommGroup E] [NormedSpace ℝ E] [CompleteSpace E]
    (f : α × β → E) (hf : Integrable f (μ.prod ν)) :
    ∫ z, f z ∂μ.prod ν = ∫ x, ∫ y, f (x, y) ∂ν ∂μ :=
  integral_prod f hf
```
