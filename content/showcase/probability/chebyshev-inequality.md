+++
title = "Chebyshev's Inequality"
description = "A random variable rarely deviates from its mean by more than a few standard deviations"
weight = 20
tags = ["lean4-proof", "probability", "visualization"]
latex = "P(|X - \\mu| \\geq k\\sigma) \\leq \\frac{1}{k^2}"
prerequisites = ["markov-inequality"]
lean4_status = "complete"
+++

## Statement

For a square-integrable real random variable $X$ with mean $\mu = E[X]$, variance $\sigma^2 = \text{Var}(X)$, and any $c > 0$:

$$P(|X - \mu| \geq c) \leq \frac{\sigma^2}{c^2}$$

Writing $c = k\sigma$ gives the memorable form $P(|X - \mu| \geq k\sigma) \leq 1/k^2$.

## Visualization

Consider a discrete distribution approximating a bell shape:

| $x$ | $-3$ | $-2$ | $-1$ | $0$ | $1$ | $2$ | $3$ |
|---|---|---|---|---|---|---|---|
| $p(x)$ | $1/16$ | $2/16$ | $4/16$ | $2/16$ | $4/16$ | $2/16$ | $1/16$ |

Mean $\mu = 0$ (by symmetry). Variance $\sigma^2 \approx 2.25$, so $\sigma \approx 1.5$.

| $k$ | Threshold $k\sigma$ | Exact $P(\|X\| \geq k\sigma)$ | Chebyshev $1/k^2$ |
|---|---|---|---|
| 1 | 1.5 | $3/16 \approx 0.19$ | $1.0$ (trivial) |
| 1.5 | 2.25 | $3/16 \approx 0.19$ | $0.44$ |
| 2 | 3.0 | $2/16 \approx 0.13$ | $0.25$ |
| 3 | 4.5 | $0$ | $0.11$ |

The bound is distribution-free — it cannot be improved for all distributions simultaneously (the bound is tight for a two-point mass).

## Proof Sketch

Chebyshev is Markov applied to the non-negative random variable $Y = (X - \mu)^2$:

$$P(|X - \mu| \geq c) = P(Y \geq c^2) \leq \frac{E[Y]}{c^2} = \frac{\sigma^2}{c^2}$$

## Connections

Chebyshev's inequality is [[Markov's Inequality]] applied to the square deviation. Its power comes from using more information (the variance). The weak law of large numbers follows directly: sample averages concentrate around the mean with rate $\sigma^2/n$. The same spirit of concentration appears in the study of [[Iterated Function Systems]], where contraction ratios control convergence rates.

## Lean4 Proof

The proof below uses `ProbabilityTheory.meas_ge_le_variance_div_sq` from `Mathlib.Probability.Moments.Variance` (Mathlib v4.28.0).

```lean4
import Mathlib.Probability.Moments.Variance

namespace MoonMath

open MeasureTheory ProbabilityTheory

/-- **Chebyshev's inequality** (real-valued form).
    For a square-integrable random variable `X` and threshold `c > 0`,
    `μ {ω | c ≤ |X ω - μ[X]|} ≤ ENNReal.ofReal (Var[X] / c²)`. -/
theorem chebyshev_inequality
    {Ω : Type*} {m : MeasurableSpace Ω} (μ : Measure Ω)
    [IsFiniteMeasure μ] {X : Ω → ℝ}
    (hX : MemLp X 2 μ) {c : ℝ} (hc : 0 < c) :
    μ {ω | c ≤ |X ω - μ[X]|} ≤ ENNReal.ofReal (variance X μ / c ^ 2) :=
  meas_ge_le_variance_div_sq hX hc

end MoonMath
```

`meas_ge_le_variance_div_sq` is itself proved by reducing to `meas_ge_le_evariance_div_sq`, which in turn reduces to `meas_ge_le_mul_pow_eLpNorm_enorm` — precisely the $L^2$ Markov inequality applied to $|X - E[X]|$.
