+++
title = "Weak Convergence (Distribution)"
description = "A sequence of probability measures converges weakly if integrals of bounded continuous functions converge"
weight = 190
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\mu_n \\xrightarrow{w} \\mu \\iff \\int f\\,d\\mu_n \\to \\int f\\,d\\mu \\text{ for all } f \\in C_b"
prerequisites = ["characteristic-function", "central-limit-theorem"]
lean4_status = "complete"
+++

## Statement

Let $(\Omega, d)$ be a metric space and $\mu_n, \mu$ probability measures on the Borel $\sigma$-algebra. The sequence $(\mu_n)$ **converges weakly** (in distribution) to $\mu$, written $\mu_n \xrightarrow{w} \mu$, if for every bounded continuous function $f : \Omega \to \mathbb{R}$:

$$\int f\,d\mu_n \;\to\; \int f\,d\mu \quad \text{as } n \to \infty.$$

Equivalent characterisations (Portmanteau theorem):
1. $\limsup_n \mu_n(F) \le \mu(F)$ for all closed $F$.
2. $\liminf_n \mu_n(G) \ge \mu(G)$ for all open $G$.
3. $\mu_n(A) \to \mu(A)$ for all $A$ with $\mu(\partial A) = 0$.
4. $F_n(x) \to F(x)$ at all continuity points $x$ of $F$ (when $\Omega = \mathbb{R}$).

## Visualization

Empirical CDFs of $B_n = \text{Bin}(n, p)/n$ converging weakly to $\delta_p$ (point mass at $p$) for $p = 0.5$:

```
CDF of Bin(n, 1/2)/n  (normalized)

n=5:
1 |              ╔══════════
  |        ╔════╝
0 |════════╝
  +---0---0.5---1

n=20:
1 |          ╔══════════════
  |       ╔══╝
0 |═══════╝
  +---0---0.5---1

n=100:
1 |        ╔═══════════════
  |      ══╝
0 |══════
  +---0---0.5---1

n → ∞: CDF approaches Heaviside step at x = 0.5:
1 |       ╔══════════════
  |       ║
0 |═══════╝
  +---0---0.5---1  (Dirac δ_{0.5})
```

The empirical means $\bar X_n \to p$ a.s. by the [[Strong Law of Large Numbers]], and the distributions $\text{Bin}(n,p)/n$ converge weakly to $\delta_p$.

## Proof Sketch

1. **Topology on probability measures.** The weak topology on $\mathcal{P}(\Omega)$ is the coarsest making $\mu \mapsto \int f\,d\mu$ continuous for all $f \in C_b(\Omega)$.
2. **Metrization.** On Polish spaces, weak convergence is metrizable (e.g., Prokhorov metric or bounded Lipschitz distance).
3. **Characterisation.** Equivalence of the four Portmanteau conditions follows by approximating indicator functions of open/closed sets by continuous functions (Urysohn's lemma) and monotone convergence.
4. **Tightness and Prokhorov.** A family $\{\mu_n\}$ is tight iff every subsequence has a weakly convergent sub-subsequence (Prokhorov's theorem on Polish spaces).

## Connections

The [[Central Limit Theorem]] is precisely a weak convergence statement: the distributions of $Z_n$ converge weakly to $\mathcal{N}(0,1)$. The [[Characteristic Function]] is continuous with respect to weak convergence (Lévy continuity theorem), making it the main tool for proving weak convergence. The underlying topology on probability measures is studied via [[Heine–Borel Theorem]]-type compactness arguments (Prokhorov's theorem).

## Lean4 Proof

```lean4
import Mathlib.MeasureTheory.Measure.ProbabilityMeasure

open MeasureTheory BoundedContinuousFunction Filter

/-- Weak convergence of probability measures: convergence in the weak topology
    is equivalent to convergence of integrals of bounded continuous functions.
    Mathlib: `MeasureTheory.ProbabilityMeasure.tendsto_iff_forall_integral_tendsto`. -/
theorem weak_convergence_iff
    {Ω : Type*} [TopologicalSpace Ω] [MeasurableSpace Ω] [BorelSpace Ω]
    {γ : Type*} {F : Filter γ}
    {μs : γ → ProbabilityMeasure Ω} {μ : ProbabilityMeasure Ω} :
    Tendsto μs F (𝓝 μ) ↔
      ∀ f : Ω →ᵇ ℝ, Tendsto
        (fun i => ∫ ω, f ω ∂(μs i : Measure Ω))
        F
        (𝓝 (∫ ω, f ω ∂(μ : Measure Ω))) :=
  ProbabilityMeasure.tendsto_iff_forall_integral_tendsto
```
