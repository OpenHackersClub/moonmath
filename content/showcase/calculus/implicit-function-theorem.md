+++
title = "Implicit Function Theorem"
description = "A smooth equation F(x,y)=0 with invertible partial derivative in y locally defines y as a smooth function of x"
weight = 100
tags = ["lean4-proof", "calculus", "visualization"]
latex = "F(x_0,y_0)=0,\\;\\partial_y F\\text{ invertible}\\implies y=\\varphi(x)\\text{ locally}"
prerequisites = ["inverse-function-theorem", "chain-rule"]
lean4_status = "complete"
+++

## Statement

Let $F : \mathbb{R}^n \times \mathbb{R}^m \to \mathbb{R}^m$ be continuously differentiable near $(x_0, y_0)$ with $F(x_0, y_0) = 0$. If the partial derivative $\partial_y F(x_0, y_0)$ is an invertible linear map, then there exist open neighborhoods $U \ni x_0$ and $V \ni y_0$ and a unique $C^1$ function $\varphi : U \to V$ such that

$$F(x, \varphi(x)) = 0 \quad \text{for all } x \in U$$

Moreover $\varphi(x_0) = y_0$ and the derivative of $\varphi$ is given by

$$D\varphi(x) = -(\partial_y F(x, \varphi(x)))^{-1} \circ \partial_x F(x, \varphi(x))$$

## Visualization

**Example:** $F(x, y) = x^2 + y^2 - 1 = 0$ (unit circle) near $(0, 1)$.

Here $\partial_y F = 2y$, which at $(0, 1)$ gives $2 \neq 0$ (invertible). The IFT guarantees $y = \varphi(x)$ locally. Indeed $\varphi(x) = \sqrt{1 - x^2}$ near $x = 0$:

```
  y
  1 |        *         ← (0,1): varphi(0)=1
    |      *   *
    |    *       *     ← implicit curve x²+y²=1
    |  *           *
  0 |*               *
    +-+--+--+--+--+--+-
     -1  0           1   x
```

| $x$  | $\varphi(x) = \sqrt{1-x^2}$ | $\varphi'(x) = -x/\sqrt{1-x^2}$ |
|------|---------------------------|----------------------------------|
| 0.0  | 1.000                     | 0.000                            |
| 0.3  | 0.954                     | $-0.314$                         |
| 0.6  | 0.800                     | $-0.750$                         |

The derivative formula: $\varphi'(x) = -\frac{\partial_x F}{\partial_y F} = -\frac{2x}{2\varphi(x)} = -\frac{x}{\varphi(x)}$, matching $-x/\sqrt{1-x^2}$.

## Proof Sketch

1. **Reduce to IFT:** Define $\Phi(x, y) = (x, F(x, y))$. Then $D\Phi(x_0, y_0)$ is block-triangular with blocks $\mathrm{id}$ and $\partial_y F(x_0, y_0)$, hence invertible.
2. **Apply Inverse Function Theorem:** $\Phi$ is a local $C^1$ diffeomorphism near $(x_0, y_0)$. Its local inverse $\Phi^{-1}$ takes $(x, 0) \mapsto (x, \varphi(x))$ for the implicit function $\varphi$.
3. **Uniqueness:** If $F(x, y) = 0 = F(x, y')$ in a small enough neighborhood, the contracting-map argument shows $y = y'$.
4. **Derivative formula:** Differentiating $F(x, \varphi(x)) = 0$ by the Chain Rule gives the formula for $D\varphi$.

## Connections

- [[Inverse Function Theorem]] — the IFT is a direct consequence: apply the IFT to $\Phi(x,y) = (x, F(x,y))$
- [[Chain Rule]] — differentiating $F(x, \varphi(x)) = 0$ implicitly uses the chain rule to derive $D\varphi$
- [[Cauchy's Mean Value Theorem]] — related MVT machinery underlies the local diffeomorphism step in the proof

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.ImplicitContDiff

-- We state the smooth implicit function theorem using Mathlib's `IsContDiffImplicitAt`.
-- For F : E × F → G with invertible partial derivative in F at (a, b),
-- Mathlib provides `IsContDiffImplicitAt.implicitFunction` satisfying F(x, phi(x)) = 0 locally.

/-- The smooth Implicit Function Theorem: given F(a,b)=0 and invertible partial derivative,
    there exists a local smooth implicit function. This aliases Mathlib's framework. -/
theorem implicit_function_theorem
    {E F G : Type*}
    [NormedAddCommGroup E] [NormedSpace ℝ E]
    [NormedAddCommGroup F] [NormedSpace ℝ F] [CompleteSpace F]
    [NormedAddCommGroup G] [NormedSpace ℝ G]
    {n : ℕ∞} {f : E × F → G} {f' : E × F →L[ℝ] G} {a : E × F}
    (h : IsContDiffImplicitAt n f f' a) :
    ∀ᶠ x in nhds a.1, f (x, h.implicitFunction x) = f a := by
  exact h.map_implicitFunction_eq
```
