+++
title = "Kolmogorov 0-1 Law"
description = "Every tail event of an independent sequence of random variables has probability exactly 0 or 1"
weight = 140
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\mathcal{T} \\text{ tail event} \\Rightarrow P(\\mathcal{T}) \\in \\{0, 1\\}"
prerequisites = ["strong-law-large-numbers"]
lean4_status = "complete"
+++

## Statement

Let $X_1, X_2, \ldots$ be **independent** random variables on a probability space $(\Omega, \mathcal{F}, P)$. The **tail $\sigma$-algebra** is

$$\mathcal{T} = \bigcap_{n=1}^\infty \sigma(X_n, X_{n+1}, \ldots) = \limsup_{n\to\infty} \sigma(X_n).$$

A set $A \in \mathcal{T}$ is called a **tail event**: it is not affected by changing any finite initial segment of the sequence.

**Kolmogorov's 0-1 Law.** For any tail event $A \in \mathcal{T}$:

$$P(A) = 0 \quad \text{or} \quad P(A) = 1.$$

Equivalently, any $\mathcal{T}$-measurable random variable is $P$-almost surely constant.

## Visualization

Consider the event $A = \{\text{partial sums } S_n \text{ are unbounded}\}$ where $S_n = X_1 + \cdots + X_n$ and $X_i \sim \text{Bernoulli}(1/2)$ (fair coin, $\pm 1$). Observe:

- Knowing $X_1, \ldots, X_{100}$ does not determine whether $\sup_n S_n = \infty$ — this is a tail property.
- Therefore $A \in \mathcal{T}$, and the law says $P(A) \in \{0,1\}$.
- In this case the random walk is recurrent, so $P(A) = 1$.

```
Tail event structure:

σ(X₁,X₂,…) ⊇ σ(X₂,X₃,…) ⊇ σ(X₃,X₄,…) ⊇ … ⊇ T

T = intersection of all these = events independent of any finite prefix

Example tail events:
  {lim sup Sₙ/n = 0}   → probability 1  (by SLLN)
  {Sₙ bounded}          → probability 0  (recurrence)
  {∑Xₙ converges}       → probability 0 or 1 (depends on distribution)
  {Xₙ → 0}             → probability 0 or 1

Example non-tail event:
  {X₁ = 1}             → probability 1/2  (involves only X₁)
```

## Proof Sketch

1. **Tail event is independent of each finite block.** For fixed $m$, the set $A \in \sigma(X_{m+1}, X_{m+2}, \ldots)$ and $\sigma(X_1, \ldots, X_m)$ are independent (by independence of the $X_i$).
2. **Tail is independent of all finite blocks, hence of $\sigma(\bigcup_m \sigma(X_1,\ldots,X_m))$.** By a $\pi$-$\lambda$ argument (Dynkin's theorem), independence extends from $\pi$-systems to the generated $\sigma$-algebras.
3. **The tail is independent of itself.** Since $A \in \mathcal{T} \subseteq \sigma(X_{m+1}, X_{m+2}, \ldots)$ for every $m$, it is independent of $\sigma(X_1, \ldots, X_m)$ for all $m$, hence of $\mathcal{T}$ itself.
4. **Self-independent implies 0-1.** $P(A) = P(A \cap A) = P(A)^2$ forces $P(A) \in \{0,1\}$.

## Connections

The law gives a zero-or-one dichotomy analogous to the determinism of the [[Strong Law of Large Numbers]] (the limiting frequency is $P$-a.s. constant). The $\pi$-$\lambda$ argument used in step 2 is the same device behind the [[Monotone Convergence Theorem]] in measure theory.

## Lean4 Proof

```lean4
import Mathlib.Probability.Independence.ZeroOne

open ProbabilityTheory MeasureTheory MeasurableSpace Filter

/-- Kolmogorov's 0-1 law: a tail-measurable set in an independent sequence
    has measure 0 or 1.
    Mathlib: `ProbabilityTheory.measure_zero_or_one_of_measurableSet_limsup_atTop`. -/
theorem kolmogorov_zero_one
    {Ω : Type*} {m0 : MeasurableSpace Ω}
    {μ : MeasureTheory.Measure Ω} [IsFiniteMeasure μ]
    {s : ℕ → MeasurableSpace Ω}
    (h_le : ∀ n, s n ≤ m0)
    (h_indep : iIndep s μ)
    {t : Set Ω}
    (ht : MeasurableSet[limsup s atTop] t) :
    μ t = 0 ∨ μ t = 1 :=
  measure_zero_or_one_of_measurableSet_limsup_atTop h_le h_indep ht
```
