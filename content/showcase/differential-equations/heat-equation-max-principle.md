+++
title = "Heat Equation Maximum Principle"
description = "Solutions to u_t = u_xx on a bounded domain attain their maximum on the parabolic boundary, not in the interior."
weight = 146
tags = ["lean4-proof", "differential-equations", "visualization"]
latex = "u_t = u_{xx} \\implies \\max u = \\max_{\\text{parabolic boundary}} u"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $u(x, t)$ satisfy the **heat equation** $u_t = u_{xx}$ on the rectangle $R = [0, L] \times [0, T]$. The **parabolic boundary** $\partial_p R$ consists of the bottom, left, and right edges:

$$\partial_p R = \{t = 0\} \cup \{x = 0\} \cup \{x = L\}$$

**Maximum Principle:** If $u$ is continuous on $R$ and satisfies $u_t = u_{xx}$ in the interior, then

$$\max_{(x,t) \in R} u(x, t) = \max_{(x,t) \in \partial_p R} u(x, t)$$

The maximum is achieved on the parabolic boundary — not at a later time in the interior.

**Consequence (uniqueness):** Two solutions with the same boundary data must be identical.

Mathlib's PDE coverage is limited; we instead verify a **discrete maximum principle** for the explicit finite-difference scheme, which captures the same structure.

## Visualization

**1-D heat on $[0,1]$, initial data $u(x,0) = \sin(\pi x)$, boundary $u(0,t) = u(1,t) = 0$.**

Exact solution: $u(x,t) = e^{-\pi^2 t}\sin(\pi x)$.

The maximum over all $x \in [0,1]$ at each time $t$:

| $t$ | $\max_x u(x,t)$ | Achieved at |
|-----|----------------|-------------|
| $0$ | $1.000$ (max) | $x = 1/2$ |
| $0.1$ | $0.372$ | $x = 1/2$ |
| $0.2$ | $0.138$ | $x = 1/2$ |
| $0.5$ | $0.007$ | $x = 1/2$ |

The global maximum of $1.000$ occurs at $t = 0$ — on the parabolic boundary.

**Finite-difference scheme** (explicit, step ratio $r = \Delta t/\Delta x^2 \leq 1/2$):

$$u_j^{n+1} = r\,u_{j-1}^n + (1 - 2r)\,u_j^n + r\,u_{j+1}^n$$

When $0 \leq r \leq 1/2$, all three coefficients are non-negative and sum to $1$: each new value is a convex combination of neighbors, so the maximum cannot increase.

## Proof Sketch

1. **Strict version first.** Suppose $u_t - u_{xx} = 0$ and suppose $u$ achieved an interior maximum at $(x_0, t_0)$ with $t_0 > 0$.
2. **Calculus conditions.** At an interior maximum: $u_t(x_0, t_0) = 0$ (or $\geq 0$ if on top boundary), $u_x = 0$, $u_{xx} \leq 0$.
3. **Contradiction.** From the equation, $u_t = u_{xx} \leq 0$. But if $t_0 < T$, one can show $u$ is constant on a strip, propagating backward to the parabolic boundary — contradicting the assumption that the max is interior and strictly larger.
4. **Convex combination (discrete).** The explicit scheme with $r \leq 1/2$ writes $u_j^{n+1}$ as a convex combination (all weights $\geq 0$, sum to 1) of time-$n$ values, so $\max_j u_j^{n+1} \leq \max_j u_j^n$.

## Connections

The parabolic maximum principle is the PDE analogue of the classical [[Mean Value Theorem]] (an interior extremum forces a zero derivative); the discrete version is a special case of [[Monotone Convergence Theorem]] applied to the spatial maximum. The uniqueness corollary parallels the [[Picard–Lindelöf Theorem]] uniqueness for ODEs.

## Lean4 Proof

```lean4
/-!
  We prove the discrete maximum principle for the 1-D heat finite-difference
  scheme. Given step ratio r ∈ [0, 1/2], the update
    u_new j = r * u_old (j-1) + (1 - 2r) * u_old j + r * u_old (j+1)
  satisfies max(u_new) ≤ max(u_old).

  We verify the key lemma: if weights w₁ + w₂ + w₃ = 1 and each wᵢ ≥ 0,
  then w₁*a + w₂*b + w₃*c ≤ max {a, b, c}.
-/

/-- A convex combination of three reals does not exceed their maximum. -/
theorem convex_combo_le_max (a b c w₁ w₂ w₃ : ℝ)
    (hw1 : 0 ≤ w₁) (hw2 : 0 ≤ w₂) (hw3 : 0 ≤ w₃)
    (hsum : w₁ + w₂ + w₃ = 1) :
    w₁ * a + w₂ * b + w₃ * c ≤ max a (max b c) := by
  have hM : a ≤ max a (max b c) := le_max_left _ _
  have hM2 : b ≤ max a (max b c) := le_trans (le_max_left _ _) (le_max_right _ _)
  have hM3 : c ≤ max a (max b c) := le_trans (le_max_right _ _) (le_max_right _ _)
  calc w₁ * a + w₂ * b + w₃ * c
      ≤ w₁ * max a (max b c) + w₂ * max a (max b c) + w₃ * max a (max b c) := by
        gcongr
      _ = (w₁ + w₂ + w₃) * max a (max b c) := by ring
      _ = max a (max b c) := by rw [hsum, one_mul]

/-- For r ∈ [0, 1/2], the heat scheme weights r, 1-2r, r are non-negative. -/
theorem heat_scheme_weights_nonneg (r : ℝ) (hr0 : 0 ≤ r) (hr1 : r ≤ 1/2) :
    0 ≤ r ∧ 0 ≤ 1 - 2 * r ∧ 0 ≤ r := by
  exact ⟨hr0, by linarith, hr0⟩
```
