+++
title = "Borel-Cantelli Lemma"
description = "If the sum of event probabilities is finite, almost surely only finitely many events occur"
weight = 70
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\sum_{n=1}^\\infty P(A_n) < \\infty \\Rightarrow P(A_n \\text{ i.o.}) = 0"
prerequisites = ["markov-inequality"]
lean4_status = "complete"
+++

## Statement

**First Borel–Cantelli Lemma.** Let $(A_n)_{n \geq 1}$ be a sequence of events in a probability space $(\Omega, \mathcal{F}, P)$. If

$$\sum_{n=1}^{\infty} P(A_n) < \infty$$

then $P(A_n \text{ infinitely often}) = 0$, i.e., almost surely only finitely many $A_n$ occur:

$$P\!\left(\limsup_{n \to \infty} A_n\right) = P\!\left(\bigcap_{N=1}^\infty \bigcup_{n \geq N} A_n\right) = 0$$

**Second Borel–Cantelli Lemma.** If the $A_n$ are additionally independent and $\sum P(A_n) = \infty$, then $P(A_n \text{ i.o.}) = 1$.

## Visualization

**Coin-flip example.** Flip a coin on trial $n$ with probability $p_n = 1/n^2$ of heads ($A_n$ = "heads on trial $n$").

| $n$ | $p_n = 1/n^2$ | partial sum $\sum_{k=1}^n p_k$ |
|-----|--------------|-------------------------------|
| 1   | 1.000        | 1.000                         |
| 2   | 0.250        | 1.250                         |
| 5   | 0.040        | 1.464                         |
| 10  | 0.010        | 1.548                         |
| 100 | 0.0001       | 1.635                         |
| $\infty$ | —      | $\pi^2/6 \approx 1.645$      |

Since $\sum p_n = \pi^2/6 < \infty$, the first Borel–Cantelli lemma says: almost surely, only finitely many trials yield heads. After some random time $N(\omega)$, all subsequent coins show tails.

**Contrast:** With $p_n = 1/n$, $\sum p_n = \infty$ (harmonic series), and if the trials are independent, BC2 gives $P(\text{i.o.}) = 1$.

## Proof Sketch

1. **Measure of the limsup.** $P(\limsup A_n) = P\!\left(\bigcap_N \bigcup_{n \geq N} A_n\right) = \lim_{N \to \infty} P\!\left(\bigcup_{n \geq N} A_n\right)$.
2. **Sub-additivity (union bound).** $P\!\left(\bigcup_{n \geq N} A_n\right) \leq \sum_{n \geq N} P(A_n)$.
3. **Tail goes to 0.** Because $\sum_{n=1}^\infty P(A_n) < \infty$, its tail $\sum_{n \geq N} P(A_n) \to 0$ as $N \to \infty$.
4. **Conclude.** $P(\limsup A_n) \leq \lim_{N \to \infty} \sum_{n \geq N} P(A_n) = 0$.

## Connections

Borel–Cantelli is the probabilistic cousin of [[Markov's Inequality]]: both control tail probabilities. It is used heavily in the study of [[Iterated Function Systems]] (almost-sure convergence of random IFS orbits) and in proving the strong law of large numbers, which itself refines [[Chebyshev's Inequality]].

## Lean4 Proof

Mathlib provides the first Borel–Cantelli lemma as `MeasureTheory.measure_limsup_atTop_eq_zero` in `Mathlib.MeasureTheory.OuterMeasure.BorelCantelli`.

```lean4
import Mathlib.MeasureTheory.OuterMeasure.BorelCantelli

namespace MoonMath

open MeasureTheory Filter

/-- **First Borel–Cantelli lemma**.
    If `Σ μ(sₙ) < ∞`, then `μ(limsup sₙ) = 0`.
    Here `limsup sₙ` along `atTop` is the set of points in infinitely many `sₙ`. -/
theorem borel_cantelli_first {α : Type*} {m : MeasurableSpace α}
    (μ : Measure α) {s : ℕ → Set α}
    (hs : ∑' n, μ (s n) ≠ ∞) :
    μ (limsup s atTop) = 0 :=
  measure_limsup_atTop_eq_zero hs

end MoonMath
```

`measure_limsup_atTop_eq_zero` is proved by the tail-sum argument: $\mu(\limsup s_n) \leq \sum_{n \geq N} \mu(s_n) \to 0$ via `ENNReal.tendsto_tsum_compl_atTop_zero`.

