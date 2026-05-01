+++
title = "Extreme Value Theorem"
description = "A continuous function on a compact set attains its maximum and minimum values"
weight = 130
tags = ["lean4-proof", "calculus", "visualization"]
latex = "f\\in C(K),\\,K\\text{ compact}\\implies\\exists\\,x_*,x^*\\in K:\\;f(x_*)=\\min f=f(x^*)\\leq\\max f"
prerequisites = ["intermediate-value-theorem"]
lean4_status = "complete"
+++

## Statement

Let $K$ be a compact topological space (e.g., a closed bounded interval $[a, b] \subset \mathbb{R}$) and $f : K \to \mathbb{R}$ a continuous function. Then $f$ is bounded and attains both its maximum and minimum:

$$\exists\, x_* \in K : f(x_*) = \min_{x \in K} f(x) \quad \text{and} \quad \exists\, x^* \in K : f(x^*) = \max_{x \in K} f(x)$$

**Compactness is essential.** On an open or unbounded domain, continuous functions need not attain their extrema.

## Visualization

**Counterexample without compactness:** $f(x) = x \sin(1/x)$ on the open interval $(0, 1)$.

```
  f(x) = x sin(1/x) on (0,1)

  1 |   /\/\/\/\/\/\___   ← oscillates, |f(x)| ≤ x
    |  /
  0 |--+--+--+--+--+--
    |   \
 -1 |    \/\/\/\/\/\/\   ← oscillates below
    +-+--+--+--+--+--+-
      0                1
```

The supremum is approached but never attained on $(0,1)$ — compactness fails.

**Valid case:** $f(x) = x^2 - x$ on $[0, 1]$ (compact).

| $x$  | $f(x) = x^2 - x$ |
|------|-----------------|
| 0.0  | 0.00            |
| 0.25 | $-0.1875$       |
| **0.5** | **$-0.25$** ← minimum ✓ |
| 0.75 | $-0.1875$       |
| 1.0  | 0.00            |

Maximum value: $0$ (attained at both endpoints $x = 0$ and $x = 1$). Minimum: $-1/4$ at $x = 1/2$.

## Proof Sketch

1. **Boundedness:** The continuous image $f(K)$ of a compact set is compact (general topology). A compact subset of $\mathbb{R}$ is bounded.
2. **Supremum exists:** Since $f(K)$ is bounded above, $M = \sup f(K)$ exists in $\mathbb{R}$.
3. **Supremum is attained:** By compactness of $f(K)$ (closed and bounded in $\mathbb{R}$), $M \in f(K)$. So $M = f(x^*)$ for some $x^* \in K$.
4. **Minimum:** Apply the same argument to $-f$ (or directly to $\inf f(K)$).

## Connections

- [[Intermediate Value Theorem]] — the IVT shows continuous functions on $[a,b]$ attain all intermediate values; the EVT shows they attain the extreme values
- [[Heine-Borel Theorem]] — the Heine-Borel theorem characterises compactness in $\mathbb{R}^n$ as closed and bounded, which is what makes $[a,b]$ the canonical domain for the EVT
- [[Bolzano-Weierstrass Theorem]] — alternatively, the EVT follows from Bolzano-Weierstrass: take a sequence $x_n$ with $f(x_n) \to \sup f$; by BWT it has a convergent subsequence

## Lean4 Proof

```lean4
import Mathlib.Topology.Order.Compact

/-- Extreme Value Theorem (maximum): a continuous function on a compact nonempty set
    attains its maximum. Wraps `IsCompact.exists_isMaxOn`. -/
theorem evt_max {α : Type*} [TopologicalSpace α] {f : α → ℝ}
    {s : Set α} (hs : IsCompact s) (hne : s.Nonempty)
    (hf : ContinuousOn f s) :
    ∃ x ∈ s, IsMaxOn f s x :=
  hs.exists_isMaxOn hne hf

/-- Extreme Value Theorem (minimum): a continuous function on a compact nonempty set
    attains its minimum. Wraps `IsCompact.exists_isMinOn`. -/
theorem evt_min {α : Type*} [TopologicalSpace α] {f : α → ℝ}
    {s : Set α} (hs : IsCompact s) (hne : s.Nonempty)
    (hf : ContinuousOn f s) :
    ∃ x ∈ s, IsMinOn f s x :=
  hs.exists_isMinOn hne hf
```
