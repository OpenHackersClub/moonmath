+++
title = "Lagrange Multipliers"
description = "At a constrained extremum, the gradient of the objective is proportional to the gradient of the constraint"
weight = 40
tags = ["lean4-proof", "optimization", "visualization"]
latex = "\\nabla f(x^*) = \\lambda\\, \\nabla g(x^*)"
prerequisites = ["kkt-conditions"]
lean4_status = "complete"
+++

## Statement

To find the extrema of $f : \mathbb{R}^n \to \mathbb{R}$ subject to the constraint $g(x) = 0$, introduce a multiplier $\lambda \in \mathbb{R}$ and require:

$$\nabla f(x^*) = \lambda\, \nabla g(x^*).$$

Geometrically: at the constrained extremum, the level set of $f$ and the constraint surface $g = 0$ are tangent — their normals are parallel.

**Concrete example.** Maximise $f(x,y) = xy$ subject to $g(x,y) = x + y - 4 = 0$.

Lagrangian: $\mathcal{L} = xy - \lambda(x + y - 4)$.

First-order conditions: $y = \lambda$ and $x = \lambda$, so $x = y$.  From $x + y = 4$: $x^* = y^* = 2$, $\lambda = 2$, $f(x^*) = 4$.

## Visualization

Level curves of $f(x,y) = xy$ (hyperbolas) and the constraint $x + y = 4$ (line):

```
  y
  4 *
    |\ tangent point at (2,2)
  3 | \  xy = 4  (tangent level curve)
    |  *------
  2 |  |(2,2)  <-- optimum
    |  |    \
  1 |  |     xy = 1
    |  |
  0 +--+--+--*---  x
     0  1  2  4
```

At the optimum $(2,2)$: $\nabla f = (y,x) = (2,2)$ and $\nabla g = (1,1)$, so $\nabla f = 2 \cdot \nabla g$, confirming $\lambda = 2$.

| $(x,y)$ | $xy$ | $x + y$ | feasible? |
|---------|------|---------|-----------|
| $(1, 3)$  | 3    | 4       | yes       |
| $(2, 2)$  | **4**| 4       | yes (max) |
| $(3, 1)$  | 3    | 4       | yes       |
| $(0, 4)$  | 0    | 4       | yes       |

## Proof Sketch

1. **Parametrise the constraint:** the feasible set is a smooth manifold $M = \{g = 0\}$.
2. **Tangent condition:** at a constrained extremum, $\nabla f(x^*)$ must be orthogonal to every tangent vector of $M$, i.e., $\nabla f(x^*) \perp \ker(\nabla g(x^*))$.
3. **Linear algebra:** $v \perp \ker(L)$ iff $v \in \text{range}(L^\top)$ — so $\nabla f = \lambda \nabla g$.
4. **Mathlib:** `IsLocalExtrOn.exists_multipliers_of_hasStrictFDerivAt_1d` provides the rigorous 1D-constraint version in `Mathlib.Analysis.Calculus.LagrangeMultipliers`.

For $\max xy$ s.t. $x + y = 4$: verified directly that $f(2,2) = 4 \ge f(x,y)$ for all $(x,y)$ with $x + y = 4$.

## Connections

- [[KKT Conditions]] — KKT conditions extend Lagrange multipliers to inequality constraints with sign conditions on $\lambda$
- [[Mean Value Theorem]] — the proof of Lagrange multipliers uses the MVT to construct the tangent parametrisation
- [[Cauchy–Schwarz Inequality]] — Cauchy–Schwarz maximises $\langle u, v \rangle$ subject to $\|u\| = \|v\| = 1$ via Lagrange multipliers
- [[Spectral Theorem]] — eigenvalue problems are Lagrange multiplier problems: $\max \langle Ax, x \rangle$ s.t. $\|x\| = 1$

## Lean4 Proof

```lean4
import Mathlib.Analysis.InnerProductSpace.Basic

/-- The Lagrange optimum for max xy s.t. x+y=4 is (2,2) with value 4.
    Any feasible (x,y) satisfies xy ≤ 4. -/
theorem lagrange_max_product :
    ∀ x y : ℝ, x + y = 4 → x * y ≤ 4 := by
  intro x y hxy
  nlinarith [sq_nonneg (x - y)]
```
