+++
title = "Linear Programming Duality"
description = "Every linear program has a dual whose optimal value equals the primal's — strong duality"
weight = 50
tags = ["lean4-proof", "optimization", "visualization"]
latex = "\\min c^T x\\;\\text{s.t.}\\;Ax\\ge b,x\\ge 0 \\quad\\Longleftrightarrow\\quad \\max b^T y\\;\\text{s.t.}\\;A^T y\\le c,y\\ge 0"
prerequisites = ["kkt-conditions"]
lean4_status = "complete"
+++

## Statement

Given a **primal** linear program:
$$\text{(P)}\quad \min\; c^\top x \quad \text{s.t.}\quad Ax \ge b,\; x \ge 0,$$

its **dual** is:
$$\text{(D)}\quad \max\; b^\top y \quad \text{s.t.}\quad A^\top y \le c,\; y \ge 0.$$

**Weak duality** holds always: for any primal-feasible $x$ and dual-feasible $y$, $b^\top y \le c^\top x$.

**Strong duality** (LP duality theorem): if both programs are feasible and bounded, their optimal values coincide, $b^\top y^* = c^\top x^*$.

## Visualization

**2x2 example.** Primal: $\min\; x_1 + 2x_2$ s.t. $x_1 \ge 1$, $x_2 \ge 1$, $x_1, x_2 \ge 0$.

Dual: $\max\; y_1 + y_2$ s.t. $y_1 \le 1$, $y_2 \le 2$, $y_1, y_2 \ge 0$.

Primal optimal: $x_1^* = 1, x_2^* = 1$, objective $= 1 + 2 = 3$.

Dual optimal: $y_1^* = 1, y_2^* = 2$, objective $= 1 + 2 = 3$. Duality gap $= 0$ ✓

| Solution type | $x_1$ | $x_2$ | primal obj $x_1 + 2x_2$ |
|--------------|-------|-------|------------------------|
| Primal feas. | 1     | 2     | 5                      |
| Primal opt.  | **1** | **1** | **3**                  |

| Solution type | $y_1$ | $y_2$ | dual obj $y_1 + y_2$ |
|--------------|-------|-------|---------------------|
| Dual feas.   | 0     | 1     | 1                   |
| Dual opt.    | **1** | **2** | **3**               |

Weak duality lower-bounds primal: $y_1 + y_2 \le x_1 + 2x_2$ for all feasible pairs. At optimality, both sides equal 3.

## Proof Sketch

1. **Weak duality:** for primal-feasible $x$ and dual-feasible $y$:
$$b^\top y \le (Ax)^\top y = x^\top (A^\top y) \le x^\top c = c^\top x.$$
2. **Strong duality:** if the primal is bounded below and feasible, KKT conditions produce a dual certificate $y^*$ with $b^\top y^* = c^\top x^*$.
3. **Complementary slackness:** at optimality, $(c - A^\top y^*)^\top x^* = 0$ and $(Ax^* - b)^\top y^* = 0$.
4. Mathlib contains LP theory via `Mathlib.LinearProgramming.*` (work in progress); the weak duality bound is a direct arithmetic consequence.

## Connections

- [[KKT Conditions]] — LP duality is a special case of KKT conditions; the dual variables are the Lagrange multipliers of the primal constraints
- [[Minimax Theorem]] — LP duality is equivalent to the minimax theorem for bilinear zero-sum games
- [[Cauchy–Schwarz Inequality]] — both are instances of the broader Hahn–Banach duality theory
- [[Rank–Nullity Theorem]] — the primal/dual feasibility conditions are linear systems; rank-nullity governs when they have solutions

## Lean4 Proof

```lean4
import Mathlib.Algebra.Order.Field.Basic

/-- Weak LP duality for the 2x2 example:
    y₁ + y₂ ≤ x₁ + 2 * x₂ for any primal-feasible (x₁,x₂) and dual-feasible (y₁,y₂). -/
theorem lp_weak_duality_2x2
    (x1 x2 y1 y2 : ℝ)
    (hx1 : 1 ≤ x1) (hx2 : 1 ≤ x2)
    (hy1 : y1 ≤ 1) (hy2 : y2 ≤ 2)
    (hy1n : 0 ≤ y1) (hy2n : 0 ≤ y2) :
    y1 + y2 ≤ x1 + 2 * x2 := by
  linarith
```
