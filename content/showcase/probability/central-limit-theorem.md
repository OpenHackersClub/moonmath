+++
title = "Central Limit Theorem"
description = "Standardized sums of i.i.d. finite-variance random variables converge in distribution to the standard normal"
weight = 130
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\frac{\\sum_{i=1}^n X_i - n\\mu}{\\sigma\\sqrt{n}} \\xrightarrow{d} \\mathcal{N}(0,1)"
prerequisites = ["strong-law-large-numbers", "chebyshev-inequality"]
lean4_status = "complete"
+++

## Statement

Let $X_1, X_2, \ldots$ be i.i.d. with mean $\mu = \mathbb{E}[X_1]$ and finite variance $\sigma^2 = \mathrm{Var}(X_1) > 0$. Define the standardized sum

$$Z_n = \frac{\sum_{i=1}^n X_i - n\mu}{\sigma\sqrt{n}}.$$

Then $Z_n$ **converges in distribution** to the standard normal $\mathcal{N}(0,1)$:

$$\lim_{n\to\infty} P(Z_n \le t) = \Phi(t) = \frac{1}{\sqrt{2\pi}}\int_{-\infty}^t e^{-x^2/2}\,dx.$$

The **moment generating function (MGF) route**: if $M_{X}(s) = \mathbb{E}[e^{sX}]$ is finite near 0, then $M_{Z_n}(s) \to e^{s^2/2}$, which is the MGF of $\mathcal{N}(0,1)$.

## Visualization

Histograms of $S_n = \sum_{i=1}^n \mathrm{Bernoulli}(1/2)$ standardized by $(S_n - n/2)/(\sqrt{n}/2)$, for $n = 1, 5, 30, 100$:

```
n=1  (2 bars)            n=5  (6 bars)
 P                         P
0.5|█ █               0.3|  ██
   |                      |  ████
   |                   0.1| ████████
   +--0--1                +--0--5

n=30 (bell emerging)    n=100 (near-Gaussian)
 P                        P
0.15|    ████           0.08|     ████████
    |   ████████            |   ████████████
    |  ██████████           |  ██████████████
    | ████████████          | ████████████████
    +----0----30            +------0------100

       ↑ bell shape clearly visible for n≥30
```

For $n=100$, $\text{Bin}(100,1/2)$: $\mu=50$, $\sigma=5$. The probability $P(|Z_{100}| \le 2) \approx 0.954$, matching $\Phi(2) - \Phi(-2) \approx 0.9545$.

## Proof Sketch

1. **Taylor expand the MGF.** For the centred variable $Y_i = X_i - \mu$, write $M_{Y}(s/(\sigma\sqrt{n})) = 1 + \frac{s^2}{2n} + O(n^{-3/2})$.
2. **Independence gives product.** The MGF of $Z_n$ is $\bigl(M_Y(s/(\sigma\sqrt{n}))\bigr)^n$ by independence of the $X_i$.
3. **Limit of $(1 + x/n + o(1/n))^n$.** The product converges to $e^{s^2/2}$.
4. **MGF convergence implies distributional convergence** (Lévy continuity theorem via characteristic functions).
5. **Identify the limit.** $e^{s^2/2}$ is the MGF of $\mathcal{N}(0,1)$.

Mathlib v4.28.0 does not yet contain a full general CLT statement. The characteristic function of the Gaussian distribution is available as `ProbabilityTheory.charFun_gaussianReal`.

## Connections

The CLT builds directly on the [[Strong Law of Large Numbers]] and the [[Characteristic Function]] machinery for measuring distributional convergence. The bound on convergence rates is given by the Berry–Esseen theorem, which sharpens [[Chebyshev's Inequality]].

## Lean4 Proof

```lean4
import Mathlib.Probability.Distributions.Gaussian.Real
import Mathlib.MeasureTheory.Measure.CharacteristicFunction

open ProbabilityTheory MeasureTheory Complex

/-- The characteristic function of the standard Gaussian N(0,1) is exp(-t²/2).
    This is the key identity at the heart of the CLT (via Lévy's continuity theorem).
    Mathlib: `ProbabilityTheory.charFun_gaussianReal` (with μ=0, v=1). -/
theorem clt_key_identity (t : ℝ) :
    charFun (gaussianReal (μ := 0) (v := 1)) t = cexp (-(t : ℂ) ^ 2 / 2) := by
  rw [charFun_gaussianReal]
  simp only [mul_zero, zero_mul, ofReal_one, one_mul]
  ring_nf
```
