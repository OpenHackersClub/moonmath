+++
title = "Optional Stopping Theorem"
description = "The expected value of a stopped martingale equals its initial value under integrability conditions"
weight = 160
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\mathbb{E}[M_\\tau] = \\mathbb{E}[M_0]"
prerequisites = ["doob-martingale-convergence", "markov-inequality"]
lean4_status = "complete"
+++

## Statement

Let $(M_n, \mathcal{F}_n)$ be a martingale on a probability space $(\Omega, \mathcal{F}, P)$. A **stopping time** $\tau$ is a $\{0,1,2,\ldots\}$-valued random variable with $\{\tau \le n\} \in \mathcal{F}_n$ for all $n$.

The **stopped process** $M_{n \wedge \tau}$ is again a martingale. Under boundedness ($\tau \le N$ a.s. for some fixed $N$, or $\sup_n |M_n| < C$ a.s.):

$$\mathbb{E}[M_\tau] = \mathbb{E}[M_0].$$

More generally (for submartingales with bounded stopping time $\sigma \le \tau \le N$):

$$\mathbb{E}[M_\sigma] \le \mathbb{E}[M_\tau].$$

## Visualization

**Simple random walk** $S_n = X_1 + \cdots + X_n$, $X_i \sim \text{Uniform}\{-1,+1\}$. Set $\tau = $ first time $S_n \in \{-a, b\}$ for $a, b > 0$.

```
Sample path with a=2, b=3:
S
 3|                  *  ← absorbed at b=+3
 2|              * *
 1|   *   * * *
 0|──*─*─*───────────→ n
-1|     *
-2|                   (barrier, not hit)

τ = first hitting time of {-2, +3}

E[Sτ] = E[S₀] = 0  (OST, since τ is bounded by a·b steps)

More precisely: P(Sτ = b) = a/(a+b), P(Sτ = -a) = b/(a+b)
Check: b · a/(a+b) + (-a) · b/(a+b) = (ab - ab)/(a+b) = 0 ✓

For a=2, b=3:  P(hit +3) = 2/5 = 0.4,  P(hit -2) = 3/5 = 0.6
E[Sτ] = 3·(2/5) + (-2)·(3/5) = 6/5 - 6/5 = 0 ✓
```

## Proof Sketch

1. **Stopped process is a (sub)martingale.** For $\sigma \le \tau \le N$, express $M_\tau - M_\sigma = \sum_{k=\sigma}^{\tau-1} (M_{k+1} - M_k)$.
2. **Write as a predictable integral.** Define $H_k = \mathbf{1}_{\sigma < k \le \tau}$ which is $\mathcal{F}_{k-1}$-measurable (since $\sigma, \tau$ are stopping times). Then $M_\tau - M_\sigma = \sum_k H_k (M_k - M_{k-1})$.
3. **Take expectations.** By the martingale property $\mathbb{E}[H_k (M_k - M_{k-1}) \mid \mathcal{F}_{k-1}] = H_k \cdot 0 = 0$ for martingales, or $\ge 0$ for submartingales.
4. **Sum the telescoping differences** to conclude $\mathbb{E}[M_\tau - M_\sigma] = 0$ (or $\ge 0$).

## Connections

Optional stopping is the engine behind the classical gambling ruin calculation and the [[Doob's Martingale Convergence]] theory. Combined with the [[Strong Law of Large Numbers]], it characterises recurrence of random walks.

## Lean4 Proof

```lean4
import Mathlib.Probability.Martingale.OptionalStopping

open MeasureTheory

/-- Optional stopping theorem: for a submartingale, the expected stopped value
    is monotone in the stopping time.
    Mathlib: `MeasureTheory.Submartingale.expected_stoppedValue_mono`. -/
theorem optional_stopping_alias
    {Ω : Type*} {m0 : MeasurableSpace Ω}
    {μ : Measure Ω} [SigmaFiniteFiltration μ (𝒢 : Filtration ℕ m0)]
    {f : ℕ → Ω → ℝ}
    {τ π : Ω → ℕ}
    {N : ℕ}
    (hf : Submartingale f 𝒢 μ)
    (hτ : IsStoppingTime 𝒢 τ)
    (hπ : IsStoppingTime 𝒢 π)
    (hle : τ ≤ π)
    (hbdd : ∀ ω, π ω ≤ N) :
    μ[stoppedValue f τ] ≤ μ[stoppedValue f π] :=
  hf.expected_stoppedValue_mono hτ hπ hle hbdd
```
