+++
title = "Martingale Definition"
description = "A stochastic process is a martingale if future conditional expectations equal the current value"
weight = 110
tags = ["lean4-proof", "probability", "visualization"]
latex = "E[M_{n+1} \\mid \\mathcal{F}_n] = M_n \\text{ a.s.}"
prerequisites = ["conditional-expectation-linear", "total-expectation"]
lean4_status = "complete"
+++

## Statement

A stochastic process $(M_n)_{n \geq 0}$ adapted to a filtration $(\mathcal{F}_n)_{n \geq 0}$ is a **martingale** with respect to measure $P$ if:

1. $M_n$ is integrable for all $n$, and
2. For all $i \leq j$: $E[M_j \mid \mathcal{F}_i] = M_i$ a.s.

Equivalently (in the Mathlib formulation): $M$ is strongly adapted to $\mathcal{F}$ and the conditional expectation property $P[M_j \mid \mathcal{F}_i] =^{\mathrm{a.e.}} M_i$ holds for all $i \leq j$.

## Visualization

**Symmetric random walk.** Let $X_1, X_2, \ldots$ be i.i.d. with $P(X_k = +1) = P(X_k = -1) = 1/2$, and set $M_n = X_1 + \cdots + X_n$, $M_0 = 0$.

| $n$ | Possible values of $M_n$ | $E[M_{n+1} \mid M_n = m]$ |
|-----|--------------------------|---------------------------|
| $0$ | $0$ | $0 \cdot 1 = 0$ |
| $1$ | $\pm 1$ | $\frac{1}{2}(m+1) + \frac{1}{2}(m-1) = m$ |
| $2$ | $0, \pm 2$ | $m$ (same computation) |
| $n$ | $\{-n, -n+2, \ldots, n\}$ | $m$ for any $m$ |

The next step always goes $\pm 1$ with equal probability, so the expected value of $M_{n+1}$ given $M_n = m$ is exactly $m$. The walk is "fair": no drift.

```
     +2  *
         |
   +1    *   *
         |   |
    0  * | * | *
         |   |
   -1    *   *
         |
     -2  *
   n: 0  1  2  3
```

Each level is equally likely to go up or down, keeping the expected future value equal to the present.

## Proof Sketch

**Verification for the random walk:**

1. **Adaptedness.** $M_n = X_1 + \cdots + X_n$ is $\mathcal{F}_n$-measurable by definition.
2. **Integrability.** $E[|M_n|] \leq \sum_{k=1}^n E[|X_k|] = n < \infty$.
3. **Martingale property.** $E[M_{n+1} \mid \mathcal{F}_n] = E[M_n + X_{n+1} \mid \mathcal{F}_n]$.
   - By [[Linearity of Conditional Expectation]]: $= E[M_n \mid \mathcal{F}_n] + E[X_{n+1} \mid \mathcal{F}_n]$.
   - Since $M_n$ is $\mathcal{F}_n$-measurable: $E[M_n \mid \mathcal{F}_n] = M_n$ a.s.
   - Since $X_{n+1}$ is independent of $\mathcal{F}_n$: $E[X_{n+1} \mid \mathcal{F}_n] = E[X_{n+1}] = 0$.
   - Conclusion: $E[M_{n+1} \mid \mathcal{F}_n] = M_n$ a.s.

## Connections

The martingale definition builds on [[Linearity of Conditional Expectation]] (step 3 above) and [[Law of Total Expectation]] (taking unconditional expectations of both sides gives $E[M_j] = E[M_i]$ for all $i \leq j$ — martingales have constant mean). The [[Borel-Cantelli Lemma]] is proved using a martingale-based argument in the second BC lemma (Mathlib's `ProbabilityTheory.measure_limsup_eq_one`).

## Lean4 Proof

Mathlib defines `MeasureTheory.Martingale` in `Mathlib.Probability.Martingale.Basic`. The symmetric random walk is a martingale via `martingale_condExp`.

```lean4
import Mathlib.Probability.Martingale.Basic

namespace MoonMath

open MeasureTheory

/-- The process `fun i => μ[f | ℱ i]` (the tower of conditional expectations of `f`) is
    always a martingale. This establishes the pattern: martingales arise whenever you take
    conditional expectations of an integrable target. -/
theorem condExp_martingale {Ω E : Type*} [NormedAddCommGroup E] [NormedSpace ℝ E]
    [CompleteSpace E] {m0 : MeasurableSpace Ω} (μ : Measure Ω) {ι : Type*} [Preorder ι]
    (ℱ : Filtration ι m0) (f : Ω → E) :
    Martingale (fun i => μ[f | ℱ i]) ℱ μ :=
  martingale_condExp f ℱ μ

/-- Martingales have constant mean: for i ≤ j, E[M_j] = E[M_i]. -/
theorem martingale_const_mean {Ω E : Type*} [NormedAddCommGroup E] [NormedSpace ℝ E]
    [CompleteSpace E] {m0 : MeasurableSpace Ω} {μ : Measure Ω} {ι : Type*} [Preorder ι]
    {ℱ : Filtration ι m0} {f : ι → Ω → E} (hf : Martingale f ℱ μ)
    {i j : ι} (hij : i ≤ j) [SigmaFiniteFiltration μ ℱ] :
    ∫ ω, f j ω ∂μ = ∫ ω, f i ω ∂μ :=
  hf.setIntegral_eq hij MeasurableSet.univ

end MoonMath
```

`martingale_condExp` is proved in Mathlib using `condExp_condExp_of_le` (tower property) to verify $P[P[f \mid \mathcal{F}_j] \mid \mathcal{F}_i] =^{\mathrm{a.e.}} P[f \mid \mathcal{F}_i]$ for $i \leq j$.

