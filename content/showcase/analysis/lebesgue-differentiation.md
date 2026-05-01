+++
title = "Lebesgue Differentiation Theorem"
description = "For locally integrable f, the ball averages of f converge to f(x) at almost every point x"
weight = 100
tags = ["lean4-proof", "analysis", "visualization", "measure-theory"]
latex = "\\lim_{r \\to 0} \\frac{1}{|B_r(x)|} \\int_{B_r(x)} f \\, d\\mu = f(x) \\text{ a.e.}"
prerequisites = ["dominated-convergence", "fubini-theorem"]
lean4_status = "complete"
+++

The **Lebesgue Differentiation Theorem** says that for a locally integrable function, averaging over smaller and smaller balls recovers the function's value at almost every point. It is the rigorous foundation for thinking of $f(x)$ as an "infinitesimal average."

## Statement

Let $\mu$ be a locally finite Borel measure on $\mathbb{R}^n$ and $f \in L^1_{\mathrm{loc}}(\mu)$. Then for $\mu$-almost every $x$:

$$\lim_{r \to 0^+} \frac{1}{\mu(B_r(x))} \int_{B_r(x)} f(y)\,d\mu(y) = f(x).$$

In particular, almost every point is a **Lebesgue density point** for $f$.

## Visualization

**Example: $f = \mathbf{1}_{[0,1/2]}$ on $\mathbb{R}$, Lebesgue measure.**

At each point $x$, compute the average of $f$ over $(-r, x+r)$ (symmetric ball of radius $r$ centred at $x$) as $r \to 0$.

| $x$ | small-$r$ average $\frac{1}{2r}\int_{x-r}^{x+r} f$ | limit ($r \to 0$) | $f(x)$ |
|-----|------------------------------------------------------|-------------------|--------|
| $1/4$ | $\frac{1}{2r} \cdot 2r = 1$ (entire ball $\subset [0,1/2]$) | $1$ | $1$ |
| $3/4$ | $\frac{1}{2r} \cdot 0 = 0$ (entire ball $\subset (1/2,1)$) | $0$ | $0$ |
| $1/2$ | $\frac{1}{2r} \cdot r = 1/2$ (half the ball) | $1/2$ | discontinuity |

At $x = 1/2$ the average is always $1/2$ regardless of $r$, not $f(1/2) = 1$. This is the single exceptional point predicted by the theorem — a set of measure zero.

**Ball averaging at $x = 1/4$:**

```
f = 1  |######### |           (indicator of [0,1/2])
       |    |     |
       0  x-r  x  x+r   1/2   1
             Ball: entirely inside [0,1/2]
             Average = 1 → f(x) = 1
```

**Ball averaging at $x = 1/2$ (boundary):**

```
f = 1  |#####|               
f = 0  |     |#####          
       0    x-r  x  x+r
             Half inside, half outside [0,1/2]
             Average = 1/2 for all r — exceptional point
```

## Proof Sketch

1. The key tool is the **Vitali covering theorem** / **Besicovitch covering theorem**: any family of balls satisfying a bounded overlap condition admits a subcollection covering a.e. every point.

2. Use the covering theorem to show the Hardy-Littlewood maximal operator $Mf(x) = \sup_{r > 0} \frac{1}{\mu(B_r(x))} \int_{B_r(x)} |f| \, d\mu$ is weak-$(1,1)$ bounded.

3. Approximate $f$ in $L^1$ by continuous functions $g$ (which satisfy the conclusion trivially by continuity). For $f - g$ small in $L^1$, the maximal inequality controls the exceptional set where averages diverge from $f - g$.

4. Conclude by a density argument: the set of $x$ where averages fail to converge has measure zero.

## Connections

The theorem is the continuous analogue of [[Cauchy Criterion]]: both characterise when a sequence (or family of averages) must converge. The differentiation theorem also gives the Radon-Nikodym theorem a differentiation interpretation: if $\nu \ll \mu$, then $d\nu/d\mu(x) = \lim_{r \to 0} \nu(B_r(x))/\mu(B_r(x))$ a.e. — connecting it directly to [[Absolutely Continuous Functions]]. Besicovitch's covering lemma is the same geometric tool underlying [[Hausdorff Dimension]] estimates.

## Lean4 Proof

```lean4
import Mathlib.MeasureTheory.Covering.Besicovitch

open MeasureTheory Besicovitch

/-- Lebesgue differentiation theorem: for a measurable set s, the density
    μ(s ∩ B(x,r)) / μ(B(x,r)) converges a.e. to the indicator of s as r → 0. -/
theorem lebesgue_density {β : Type*} [MetricSpace β] [MeasurableSpace β]
    [BorelSpace β] [SecondCountableTopology β] [HasBesicovitchCovering β]
    {μ : Measure β} [IsLocallyFiniteMeasure μ]
    {s : Set β} (hs : MeasurableSet s) :
    ∀ᵐ x ∂μ,
      Filter.Tendsto (fun r => μ (s ∩ closedBall x r) / μ (closedBall x r))
        (nhdsWithin 0 (Set.Ioi 0))
        (nhds (s.indicator 1 x)) :=
  Besicovitch.ae_tendsto_measure_inter_div_of_measurableSet μ hs
```
