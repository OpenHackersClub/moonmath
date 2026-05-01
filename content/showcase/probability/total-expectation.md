+++
title = "Law of Total Expectation"
description = "The expectation of X equals the expectation of its conditional expectation: E[X] = E[E[X|Y]]"
weight = 82
tags = ["lean4-proof", "probability", "visualization"]
latex = "E[X] = E[E[X \\mid Y]]"
prerequisites = ["markov-inequality"]
lean4_status = "complete"
+++

## Statement

Let $X$ be an integrable real-valued random variable and $Y$ any random variable generating a sub-$\sigma$-algebra $\mathcal{G}$. Then:

$$E[X] = E\!\left[E[X \mid Y]\right]$$

More precisely, for any sub-$\sigma$-algebra $\mathcal{G} \subseteq \mathcal{F}$:

$$\int_\Omega X \, dP = \int_\Omega E[X \mid \mathcal{G}] \, dP$$

This is the **tower property** (or law of iterated expectation): integrating out conditioning leaves the unconditional mean.

## Visualization

**Roll a fair six-sided die.** Let $X$ = face value, $Y = \mathbf{1}[\text{die is even}]$.

| Event $Y = y$ | Outcomes | $E[X \mid Y = y]$ | $P(Y = y)$ | contribution |
|--------------|----------|-------------------|-----------|--------------|
| $Y = 1$ (even) | $\{2, 4, 6\}$ | $(2+4+6)/3 = 4$ | $1/2$ | $4 \times 1/2 = 2$ |
| $Y = 0$ (odd)  | $\{1, 3, 5\}$ | $(1+3+5)/3 = 3$ | $1/2$ | $3 \times 1/2 = 1.5$ |

$$E\!\left[E[X \mid Y]\right] = 2 + 1.5 = 3.5 = E[X] \quad \checkmark$$

The conditional expectation $E[X \mid Y]$ is itself a random variable:
- $E[X \mid Y = 1] = 4$ (probability $1/2$)
- $E[X \mid Y = 0] = 3$ (probability $1/2$)

Averaging these over $Y$ recovers $E[X] = 3.5$.

## Proof Sketch

1. **Definition of conditional expectation.** $E[X \mid \mathcal{G}]$ is characterised as the unique $\mathcal{G}$-measurable integrable function satisfying $\int_A E[X \mid \mathcal{G}] \, dP = \int_A X \, dP$ for all $A \in \mathcal{G}$.
2. **Take $A = \Omega$.** Since $\Omega \in \mathcal{G}$, the defining property gives $\int_\Omega E[X \mid \mathcal{G}] \, dP = \int_\Omega X \, dP$.
3. **Rewrite.** $E\!\left[E[X \mid \mathcal{G}]\right] = E[X]$.

The tower property $E[E[X \mid \mathcal{F}] \mid \mathcal{G}] = E[X \mid \mathcal{G}]$ for $\mathcal{G} \subseteq \mathcal{F}$ follows from the same argument restricted to sets in $\mathcal{G}$.

## Connections

This is a special case of the iterated conditioning formula `condExp_condExp_of_le` in Mathlib. It generalises [[Bayes' Theorem]] (which computes conditional probabilities) and is the key step in proving the martingale property — see [[Martingale Definition]]. The tower property also underlies the [[Chebyshev's Inequality]] proof strategy of conditioning on a suitable event.

## Lean4 Proof

Mathlib provides `integral_condExp` (total expectation) in `Mathlib.MeasureTheory.Function.ConditionalExpectation.Basic`.

```lean4
import Mathlib.MeasureTheory.Function.ConditionalExpectation.Basic

namespace MoonMath

open MeasureTheory

/-- **Law of Total Expectation**.
    For a sub-σ-algebra `m ≤ m₀`, integrating the conditional expectation `μ[f | m]`
    over the whole space recovers the unconditional integral of `f`. -/
theorem total_expectation {α : Type*} {m₀ m : MeasurableSpace α}
    (μ : Measure α) [SigmaFinite (μ.trim (le_refl m₀))]
    (hm : m ≤ m₀) [SigmaFinite (μ.trim hm)]
    {f : α → ℝ} (hf : Integrable f μ) :
    ∫ x, (μ[f | m]) x ∂μ = ∫ x, f x ∂μ :=
  integral_condExp hm

end MoonMath
```

`integral_condExp` is proved in Mathlib directly from the defining property of conditional expectation applied to $A = \Omega$.

