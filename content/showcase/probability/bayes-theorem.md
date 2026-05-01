+++
title = "Bayes' Theorem"
description = "Reversing conditional probability to update beliefs from evidence"
weight = 30
tags = ["lean4-proof", "probability", "visualization"]
latex = "P(A \\mid B) = \\frac{P(B \\mid A)\\, P(A)}{P(B)}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For events $A$ and $B$ with $P(B) > 0$:

$$P(A \mid B) = \frac{P(B \mid A) \cdot P(A)}{P(B)}$$

In measure-theoretic notation, with $\mu[\cdot \mid S]$ denoting the conditional measure:

$$\mu[A \mid B] = \frac{\mu[B \mid A] \cdot \mu(A)}{\mu(B)}$$

## Visualization

**Disease testing.** A disease affects 1% of the population. A test is 99% sensitive (true positive rate) and 99% specific (true negative rate).

| | Test $+$ | Test $-$ | Total |
|---|---|---|---|
| Disease | 99 | 1 | 100 |
| No disease | 99 | 9801 | 9900 |
| **Total** | **198** | **9802** | **10000** |

- Prior: $P(\text{disease}) = 100/10000 = 0.01$
- Likelihood: $P(\text{test}+ \mid \text{disease}) = 99/100 = 0.99$
- Marginal: $P(\text{test}+) = 198/10000 = 0.0198$

Posterior via Bayes:

$$P(\text{disease} \mid \text{test}+) = \frac{0.99 \times 0.01}{0.0198} = \frac{0.0099}{0.0198} = 0.5$$

Despite a 99%-accurate test, only half of positive results indicate true disease — a striking consequence of the low base rate.

## Proof Sketch

From the definition $P(A \mid B) = P(A \cap B)/P(B)$ and symmetry $P(A \cap B) = P(B \cap A)$:

$$P(A \mid B) = \frac{P(A \cap B)}{P(B)} = \frac{P(B \mid A) \cdot P(A)}{P(B)}$$

The identity $P(B \mid A) \cdot P(A) = P(B \cap A)$ is the multiplication rule.

## Connections

Bayes' theorem is the foundation of Bayesian inference, where the prior $P(A)$ is updated by evidence $B$ to yield the posterior $P(A \mid B)$. The law of total probability $P(B) = P(B \mid A)P(A) + P(B \mid A^c)P(A^c)$ decomposes the denominator into cases. In the context of this showcase, the rigorous measure-theoretic conditional measure $\mu[\cdot \mid S]$ underpins [[Markov's Inequality]] and [[Chebyshev's Inequality]] — all three live in the same $\sigma$-algebra framework.

## Lean4 Proof

The proof uses `ProbabilityTheory.cond_eq_inv_mul_cond_mul` from `Mathlib.Probability.ConditionalProbability` (Mathlib v4.28.0), which is explicitly labelled **Bayes' Theorem** in Mathlib.

```lean4
import Mathlib.Probability.ConditionalProbability

namespace MoonMath

open MeasureTheory ProbabilityTheory

/-- **Bayes' theorem** (measure-theoretic form).
    `μ[A | B] = (μ B)⁻¹ * μ[B | A] * μ A`
    for measurable `A`, `B` and finite `μ`. -/
theorem bayes_theorem
    {Ω : Type*} {m : MeasurableSpace Ω} (μ : Measure Ω)
    [IsFiniteMeasure μ]
    {A B : Set Ω} (hA : MeasurableSet A) (hB : MeasurableSet B) :
    μ[A | B] = (μ B)⁻¹ * μ[B | A] * μ A :=
  cond_eq_inv_mul_cond_mul hB hA μ

end MoonMath
```

`cond_eq_inv_mul_cond_mul` is proved in Mathlib by unfolding the conditional measure definition $\mu[A \mid B] = (\mu B)^{-1} \cdot \mu(B \cap A)$, rewriting $\mu(B \cap A) = \mu[B \mid A] \cdot \mu(A)$ via `cond_mul_eq_inter`, and simplifying with `Set.inter_comm`.
