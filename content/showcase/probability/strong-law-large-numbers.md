+++
title = "Strong Law of Large Numbers"
description = "The sample mean of i.i.d. integrable random variables converges almost surely to the population mean"
weight = 120
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\frac{1}{n}\\sum_{i=1}^n X_i \\xrightarrow{\\text{a.s.}} \\mathbb{E}[X_1]"
prerequisites = ["markov-inequality", "chebyshev-inequality"]
lean4_status = "complete"
+++

## Statement

Let $X_1, X_2, \ldots$ be independent and identically distributed (i.i.d.) random variables with $\mathbb{E}[|X_1|] < \infty$. Then the sample mean converges **almost surely** to the expectation:

$$\frac{1}{n}\sum_{i=1}^n X_i \xrightarrow{\text{a.s.}} \mathbb{E}[X_1] \quad \text{as } n \to \infty.$$

"Almost surely" means the event of convergence has probability 1: for $\mu$-almost every $\omega$, the sequence of partial means converges.

## Visualization

1000 fair coin flips ($X_i \in \{0,1\}$, $p = 1/2$). Partial means converge to 0.5:

```
Mean
1.0 |*
    | **
0.8 |   *  *
    |    ** **
0.6 |       ****** *
    |             ****
0.5 |.....................*****............  ← E[X] = 0.5
    |                         ****
0.4 |                             **
    |
0.2 |
    +----+----+----+----+----+----+--→ n
    0   100  200  400  600  800 1000

Partial means stabilise around 0.5 after ~200 flips.
```

Numerical trace (first 10 flips: 1 0 1 0 0 1 1 0 1 1):

| $n$ | $\sum_{i=1}^n X_i$ | $\bar X_n$ |
|-----|-------------------|-----------|
| 1 | 1 | 1.000 |
| 2 | 1 | 0.500 |
| 5 | 2 | 0.400 |
| 10 | 6 | 0.600 |
| 50 | 27 | 0.540 |
| 200 | 103 | 0.515 |
| 1000 | 499 | 0.499 |

## Proof Sketch

1. **Reduce to nonneg case.** Write $X = X^+ - X^-$ and handle each part separately.
2. **Etemadi's truncation.** For nonneg $X$, define truncated variables $X_i^n = X_i \mathbf{1}_{X_i \le n}$. The contribution of the tail $\{X_i > n\}$ vanishes by integrability.
3. **Second moment bound on subsequence.** Apply Chebyshev's inequality to the truncated means along $\lfloor c^k \rfloor$ for some $c > 1$. The sum of variances converges, giving a.s. convergence along the subsequence.
4. **Interpolation.** Between consecutive subsequence points the mean moves by at most the maximum of a few terms; monotone integrability bounds control this gap.
5. **Conclusion.** The full sequence inherits the a.s. limit $\mathbb{E}[X_1]$.

## Connections

The SLLN strengthens the [[Chebyshev's Inequality]]-based Weak Law. It also underpins the [[Monotone Convergence Theorem]] approach used inside Mathlib's proof via the Borel–Cantelli strategy.

## Lean4 Proof

```lean4
import Mathlib.Probability.StrongLaw

/-- Strong Law of Large Numbers (almost everywhere version).
    Mathlib: `ProbabilityTheory.strong_law_ae` in
    `Mathlib/Probability/StrongLaw.lean`. -/
theorem slln_alias
    {Ω E : Type*} [MeasurableSpace Ω] [NormedAddCommGroup E]
    [NormedSpace ℝ E] [CompleteSpace E] [SecondCountableTopology E]
    [BorelSpace E]
    {μ : MeasureTheory.Measure Ω} [MeasureTheory.IsProbabilityMeasure μ]
    (X : ℕ → Ω → E)
    (hint : MeasureTheory.Integrable (X 0) μ)
    (hindep : ProbabilityTheory.iIndepFun X μ)
    (hident : ∀ i, MeasureTheory.IdentDistrib (X i) (X 0) μ μ) :
    ∀ᵐ ω ∂μ, Filter.Tendsto
      (fun n => (n : ℝ)⁻¹ • ∑ i ∈ Finset.range n, X i ω)
      Filter.atTop
      (nhds (∫ x, X 0 x ∂μ)) :=
  ProbabilityTheory.strong_law_ae X hint hindep hident
```
