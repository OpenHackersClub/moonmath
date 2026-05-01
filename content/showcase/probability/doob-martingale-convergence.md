+++
title = "Doob's Martingale Convergence"
description = "An L1-bounded submartingale converges almost surely to an integrable limit"
weight = 150
tags = ["lean4-proof", "probability", "visualization"]
latex = "M_n \\text{ submartingale, } \\sup_n \\mathbb{E}[M_n^+] < \\infty \\Rightarrow M_n \\xrightarrow{\\text{a.s.}} M_\\infty"
prerequisites = ["optional-stopping", "markov-inequality"]
lean4_status = "complete"
+++

## Statement

Let $(\Omega, \mathcal{F}, P)$ be a probability space with filtration $(\mathcal{F}_n)_{n \ge 0}$. A sequence $(M_n)$ is a **submartingale** if each $M_n$ is $\mathcal{F}_n$-measurable, integrable, and $\mathbb{E}[M_{n+1} \mid \mathcal{F}_n] \ge M_n$ a.s.

**Doob's Almost Sure Convergence Theorem.** If $(M_n)$ is an $L^1$-bounded submartingale — meaning

$$\sup_{n \ge 0} \mathbb{E}[M_n^+] < \infty$$

where $M_n^+ = \max(M_n, 0)$ — then there exists an integrable random variable $M_\infty$ such that

$$M_n \xrightarrow{\text{a.s.}} M_\infty \quad \text{as } n \to \infty.$$

In particular, every nonneg $L^1$-bounded martingale converges a.s.

## Visualization

Fair game: $M_n$ = cumulative winnings in a fair coin-flip game ($\pm 1$ per flip), stopped at $|M_n| > 5$. The martingale property says no system beats a fair game.

```
Sample paths of M_n (3 trajectories):

 M
 6|               path B hits +5 barrier
 4|         ╱╲ ╱
 2|    ╱╲╱╲╱   ╲╱╲ ╱
 0|── ─╱────────────╲────────────────→  path A wanders
-2|                   ╲╱╲
-4|                      ╲
-6|                       path C hits -5 barrier

Each path converges once it stops (optional stopping).
The a.s. limit M_∞ = lim Mₙ exists for every path.

Upcrossings of [a,b] = [1,3]:
──2──1──3──1──3──1──3── …
   ↑     ↑     ↑
   Each [1→3] passage = one upcrossing.
   Doob's upcrossing inequality: E[U_n[a,b]] ≤ (E[Mₙ⁺] + |a|)/(b-a)
   Bounded E[Upcrossings] → convergence a.s.
```

## Proof Sketch

1. **Upcrossing inequality.** Let $U_n[a,b]$ count how many times $M_0, \ldots, M_n$ crosses upward from below $a$ to above $b$. Doob proves $\mathbb{E}[U_n[a,b]] \le (E[M_n^+] + |a|)/(b-a)$.
2. **Finiteness of upcrossings.** The $L^1$ bound keeps $\mathbb{E}[U_\infty[a,b]] < \infty$, so $U_\infty[a,b] < \infty$ a.s. for each rational pair $(a,b)$.
3. **No oscillation.** On the event $\{U_\infty[a,b] < \infty \text{ for all rational } a < b\}$, the sequence cannot oscillate, so $\liminf M_n = \limsup M_n$ a.s.
4. **Integrability of limit.** Fatou's lemma: $\mathbb{E}[|M_\infty|] \le \liminf_n \mathbb{E}[|M_n|] < \infty$.

## Connections

Doob's theorem is the probabilistic analogue of the [[Monotone Convergence Theorem]] for martingales. Combined with the [[Optional Stopping Theorem]], it characterizes when a martingale is uniformly integrable.

## Lean4 Proof

```lean4
import Mathlib.Probability.Martingale.Convergence

open MeasureTheory

/-- Doob's a.e. martingale convergence theorem: an L1-bounded submartingale
    converges almost everywhere to the `limitProcess`.
    Mathlib: `MeasureTheory.Submartingale.ae_tendsto_limitProcess`. -/
theorem doob_convergence
    {Ω : Type*} {m0 : MeasurableSpace Ω}
    {μ : Measure Ω} [IsFiniteMeasure μ]
    {ℱ : Filtration ℕ m0}
    {f : ℕ → Ω → ℝ}
    {R : ℝ≥0∞}
    (hf : Submartingale f ℱ μ)
    (hbdd : ∀ n, eLpNorm (f n) 1 μ ≤ R) :
    ∀ᵐ ω ∂μ, Filter.Tendsto (fun n => f n ω) Filter.atTop
      (nhds (ℱ.limitProcess f μ ω)) :=
  hf.ae_tendsto_limitProcess hbdd
```
