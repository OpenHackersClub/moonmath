+++
title = "Markov's Inequality"
description = "A non-negative random variable rarely exceeds a multiple of its expectation"
weight = 10
tags = ["lean4-proof", "probability", "visualization"]
latex = "P(X \\geq a) \\leq \\frac{E[X]}{a}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For any non-negative random variable $X$ and any $a > 0$:

$$P(X \geq a) \leq \frac{E[X]}{a}$$

Equivalently in measure-theoretic form: for a non-negative $\mu$-almost-everywhere measurable $f$ and $\varepsilon > 0$,

$$\varepsilon \cdot \mu\{x \mid \varepsilon \leq f(x)\} \leq \int f \, d\mu$$

## Visualization

Consider rolling a fair six-sided die, so $X \in \{1,2,3,4,5,6\}$ each with probability $1/6$.

$$E[X] = \frac{1+2+3+4+5+6}{6} = \frac{21}{6} = 3.5$$

Markov's inequality with $a = 5$:

| Bound type | Value |
|---|---|
| Exact $P(X \geq 5)$ | $2/6 \approx 0.333$ |
| Markov upper bound $E[X]/5$ | $3.5/5 = 0.7$ |

The bound is loose here because the die is not concentrated near zero — Markov is tight only for distributions that put all mass at $0$ and $a$.

| $a$ | Exact $P(X \geq a)$ | Markov bound |
|---|---|---|
| 2 | $5/6 \approx 0.833$ | $3.5/2 = 1.75$ (trivial) |
| 4 | $3/6 = 0.5$ | $3.5/4 = 0.875$ |
| 5 | $2/6 \approx 0.333$ | $3.5/5 = 0.7$ |
| 6 | $1/6 \approx 0.167$ | $3.5/6 \approx 0.583$ |

## Proof Sketch

Observe that $f(x) \geq \varepsilon \cdot \mathbf{1}_{\{f \geq \varepsilon\}}(x)$ pointwise for $f \geq 0$. Integrating both sides:

$$\int f \, d\mu \geq \varepsilon \cdot \mu\{f \geq \varepsilon\}$$

That is the entire proof.

## Connections

Markov's inequality is the building block for [[Chebyshev's Inequality]], which applies Markov to the squared deviation $|X - E[X]|^2$. It underlies concentration inequalities throughout probability theory. The measure-theoretic statement connects naturally to the Lebesgue integral machinery used in [[Hausdorff Dimension]] and [[Iterated Function Systems]] via their underlying measure spaces.

## Lean4 Proof

The proof below is verified against Mathlib v4.28.0. The key lemma is `MeasureTheory.mul_meas_ge_le_lintegral₀` from `Mathlib.MeasureTheory.Integral.Lebesgue.Markov`.

```lean4
import Mathlib.MeasureTheory.Integral.Lebesgue.Markov

namespace MoonMath

open MeasureTheory

/-- **Markov's inequality** (ENNReal form).
    For a non-negative AE-measurable function `f` and threshold `ε`,
    `ε * μ {x | ε ≤ f x} ≤ ∫⁻ x, f x ∂μ`. -/
theorem markov_inequality
    {α : Type*} {m : MeasurableSpace α} (μ : Measure α)
    {f : α → ℝ≥0∞} (hf : AEMeasurable f μ) (ε : ℝ≥0∞) :
    ε * μ {x | ε ≤ f x} ≤ ∫⁻ x, f x ∂μ :=
  mul_meas_ge_le_lintegral₀ hf ε

end MoonMath
```

`mul_meas_ge_le_lintegral₀` is proved in Mathlib by the pointwise inequality $f \geq \varepsilon \cdot \mathbf{1}_{\{f \geq \varepsilon\}}$ and monotone integration.
