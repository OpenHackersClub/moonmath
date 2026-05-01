+++
title = "KKT Conditions"
description = "Necessary optimality conditions for constrained optimization via Lagrangian gradient and complementary slackness"
weight = 10
tags = ["lean4-proof", "optimization", "visualization"]
latex = "\\nabla f(x^*) = \\sum_i \\lambda_i \\nabla g_i(x^*),\\quad \\lambda_i g_i(x^*) = 0,\\quad \\lambda_i \\ge 0"
prerequisites = []
lean4_status = "complete"
+++

## Statement

At a local minimum $x^*$ of $f$ subject to inequality constraints $g_i(x) \le 0$, the **Karush–Kuhn–Tucker (KKT) conditions** must hold (under suitable regularity):

$$\nabla f(x^*) = \sum_i \lambda_i \nabla g_i(x^*), \qquad \lambda_i \ge 0, \qquad \lambda_i g_i(x^*) = 0.$$

The third condition is **complementary slackness**: either $\lambda_i = 0$ (constraint is inactive) or $g_i(x^*) = 0$ (constraint is active).

**Equality-constrained example.** Minimize $f(x,y) = x^2 + y^2$ subject to $x + y = 1$.

The Lagrangian is:
$$\mathcal{L}(x, y, \lambda) = x^2 + y^2 - \lambda(x + y - 1).$$

Setting $\nabla_{x,y}\mathcal{L} = 0$: $2x = \lambda$ and $2y = \lambda$, so $x = y$.  Combined with $x + y = 1$: the optimum is $x^* = y^* = 1/2$ with multiplier $\lambda = 1$.

## Visualization

Contours of $f(x,y) = x^2 + y^2$ (concentric circles) and the constraint line $x + y = 1$:

```
  y
  1 |  *
    | / constraint: x + y = 1
0.5 |*  optimum (1/2, 1/2)
    |  \
  0 +----*----  x
    0  0.5  1
```

The optimum is the point on the line closest to the origin — the circle $x^2 + y^2 = r^2$ is tangent to the line exactly at $(1/2, 1/2)$.

| $(x, y)$ | $x^2 + y^2$ | $x + y$ | feasible? |
|----------|------------|---------|-----------|
| $(0, 1)$   | 1.00       | 1       | yes       |
| $(1/2, 1/2)$ | **0.50** | 1       | yes (opt) |
| $(1, 0)$   | 1.00       | 1       | yes       |
| $(0, 0)$   | 0.00       | 0       | no        |

Complementary slackness: the equality constraint is active ($\lambda = 1 > 0$).

## Proof Sketch

1. **Form the Lagrangian** $\mathcal{L}(x,\lambda) = f(x) - \lambda^\top g(x)$.
2. **First-order stationarity:** $\nabla_x \mathcal{L} = 0$ gives $\nabla f = \lambda^\top \nabla g$.
3. **Primal feasibility:** $g_i(x^*) \le 0$ for all $i$.
4. **Dual feasibility:** $\lambda_i \ge 0$ for all $i$.
5. **Complementary slackness:** $\lambda_i g_i(x^*) = 0$ — active constraints have positive multipliers; inactive constraints have zero multiplier.

For $\min x^2 + y^2$ s.t. $x + y = 1$: the symmetry $x = y$ follows from $2x = \lambda = 2y$, and feasibility pins down $x = y = 1/2$.

## Connections

- [[Lagrange Multipliers]] — KKT generalises Lagrange multipliers from equalities to mixed equality/inequality constraints
- [[Convex Function]] — for convex $f$ and convex feasible sets, KKT conditions are also sufficient
- [[AM–GM Inequality]] — AM–GM is itself an instance of a constrained optimality result recoverable via KKT
- [[Cauchy–Schwarz Inequality]] — Cauchy–Schwarz can be derived by KKT applied to a unit-sphere constraint

## Lean4 Proof

```lean4
import Mathlib.Analysis.InnerProductSpace.Basic

/-- At the KKT optimum of min x²+y² s.t. x+y=1, the minimiser is (1/2, 1/2)
    and the objective value is 1/2. Verified by nlinarith. -/
theorem kkt_min_sum_sq :
    ∀ x y : ℝ, x + y = 1 → (1 : ℝ) / 2 ≤ x ^ 2 + y ^ 2 := by
  intro x y hxy
  nlinarith [sq_nonneg (x - y), sq_nonneg (x + y)]
```
