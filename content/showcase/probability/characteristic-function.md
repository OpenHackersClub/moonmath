+++
title = "Characteristic Function"
description = "The characteristic function φ_X(t) = E[exp(itX)] uniquely determines the distribution of a random variable"
weight = 170
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\phi_X(t) = \\mathbb{E}[e^{itX}] = \\int e^{itx}\\,dF_X(x)"
prerequisites = ["central-limit-theorem", "weak-convergence"]
lean4_status = "complete"
+++

## Statement

For a real-valued random variable $X$ on a probability space $(\Omega, \mathcal{F}, P)$, the **characteristic function** is

$$\phi_X(t) = \mathbb{E}[e^{itX}] = \int_{-\infty}^\infty e^{itx}\,dF_X(x), \qquad t \in \mathbb{R},$$

where $F_X$ is the CDF of $X$. The characteristic function:

1. Always exists (since $|e^{itx}| = 1$).
2. Satisfies $\phi_X(0) = 1$ and $|\phi_X(t)| \le 1$.
3. Is uniformly continuous on $\mathbb{R}$.
4. **Uniquely determines the distribution** of $X$ (inversion theorem).

For the standard Gaussian $X \sim \mathcal{N}(\mu, \sigma^2)$:

$$\phi_X(t) = e^{i\mu t - \sigma^2 t^2 / 2}.$$

In particular for $\mathcal{N}(0,1)$: $\phi_X(t) = e^{-t^2/2}$.

## Visualization

$X \sim \text{Bernoulli}(1/2)$: $X = 0$ with probability $1/2$, $X = 1$ with probability $1/2$.

$$\phi_X(t) = \frac{1}{2} e^{i \cdot 0 \cdot t} + \frac{1}{2} e^{i \cdot 1 \cdot t} = \frac{1 + e^{it}}{2}.$$

Numerical table ($|\phi_X(t)|^2 = \cos^2(t/2)$):

| $t$ | $\phi_X(t)$ (exact) | $|\phi_X(t)|$ |
|-----|---------------------|--------------|
| $0$ | $1$ | $1.000$ |
| $\pi/4$ | $(1 + e^{i\pi/4})/2$ | $0.924$ |
| $\pi/2$ | $(1 + i)/2$ | $0.707$ |
| $\pi$ | $0$ | $0.000$ |
| $2\pi$ | $1$ | $1.000$ |

The characteristic function of the **sum** of two independent Bernoulli(1/2) random variables:

$$\phi_{X+Y}(t) = \phi_X(t)^2 = \frac{(1+e^{it})^2}{4}.$$

This equals the characteristic function of $\text{Bin}(2, 1/2)$, recovering the distribution of the sum.

## Proof Sketch

**Existence and normalization.**
$|\phi_X(t)| = |\mathbb{E}[e^{itX}]| \le \mathbb{E}[|e^{itX}|] = \mathbb{E}[1] = 1$.
At $t=0$: $e^{i \cdot 0 \cdot X} = 1$, so $\phi_X(0) = \mathbb{E}[1] = 1$.

**Uniform continuity.** For any $t, s$:
$|\phi_X(t) - \phi_X(s)| \le \mathbb{E}[|e^{itX} - e^{isX}|] = \mathbb{E}[|e^{i(t-s)X} - 1|]$.
By the dominated convergence theorem (dominator = 2) and $|e^{ih} - 1| \to 0$ as $h \to 0$, continuity follows.

**Inversion (Lévy inversion formula).** Given $\phi_X$, one recovers $F_X$ via:
$F_X(b) - F_X(a) = \lim_{T\to\infty} \frac{1}{2\pi} \int_{-T}^T \frac{e^{-ita} - e^{-itb}}{it} \phi_X(t)\,dt$.

## Connections

The characteristic function is the Fourier transform of the distribution. The [[Central Limit Theorem]] is most cleanly proved by showing $\phi_{Z_n}(t) \to e^{-t^2/2}$ pointwise — the characteristic function of $\mathcal{N}(0,1)$. The Lévy continuity theorem converts pointwise convergence of $\phi$ into [[Weak Convergence (Distribution)]].

## Lean4 Proof

```lean4
import Mathlib.MeasureTheory.Measure.CharacteristicFunction
import Mathlib.MeasureTheory.Measure.MeasureSpace

open MeasureTheory Complex

/-- The characteristic function of a probability measure equals 1 at t = 0.
    Mathlib: `MeasureTheory.charFun_zero` composed with
    `IsProbabilityMeasure.measure_univ` and `Measure.real`. -/
theorem charFun_at_zero
    {E : Type*} [SeminormedAddCommGroup E] [InnerProductSpace ℝ E]
    (μ : Measure E) [IsProbabilityMeasure μ] :
    charFun μ (0 : E) = 1 := by
  simp [charFun_zero, Measure.real, IsProbabilityMeasure.measure_univ,
        ENNReal.one_toReal]
```
