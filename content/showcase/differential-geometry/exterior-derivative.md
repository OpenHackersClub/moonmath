+++
title = "Exterior Derivative"
description = "The exterior derivative d raises the degree of a differential form by one and satisfies d^2 = 0"
weight = 146
tags = ["lean4-proof", "differential-geometry", "visualization"]
latex = "d^2 = 0"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For a smooth $k$-form $\omega$ on a normed space, its exterior derivative $d\omega$ is a $(k+1)$-form defined by:

$$d\omega(x; v_0, \dots, v_k) = \sum_{i=0}^{k} (-1)^i D_x\omega(x; v_0, \dots, \widehat{v_i}, \dots, v_k) \cdot v_i$$

where $\widehat{v_i}$ denotes omission of $v_i$. The fundamental identity is:

$$d^2 = 0 \qquad \text{(the second exterior derivative is zero)}$$

This encodes the fact that mixed partials commute: antisymmetrising twice kills any symmetric contribution.

## Visualization

Concrete example on $\mathbb{R}^2$: let $\omega = x\, dy$ (a 1-form).

**Step 1 — compute $d\omega$:**

$$d\omega = d(x\, dy) = dx \wedge dy$$

(The $y$-coefficient $x$ contributes a $\partial_x x = 1$ factor; the $x$-coefficient $0$ contributes nothing.)

**Step 2 — compute $d^2\omega = d(dx \wedge dy)$:**

$$d(dx \wedge dy) = d(1) \wedge dx \wedge dy = 0 \wedge dx \wedge dy = 0$$

The 2-form $dx \wedge dy$ has constant coefficients, so its exterior derivative vanishes.

| Form | Degree | Value at $(x,y)$ |
|------|--------|-----------------|
| $\omega = x\, dy$ | 1 | depends on $x$ |
| $d\omega = dx \wedge dy$ | 2 | constant $1$ |
| $d^2\omega$ | 3 | $0$ (top form $= 0$ in $\mathbb{R}^2$) |

The table makes visible that degree 3 is impossible in $\mathbb{R}^2$, so $d^2\omega = 0$ is forced dimensionally as well.

## Proof Sketch

1. **Expand by definition.** Write $d\omega$ as the alternating sum of $i$-th directional derivatives; applying $d$ again produces a double sum over pairs $(i, j)$.
2. **Separate symmetric part.** Each double-partial $\partial_i \partial_j f$ appears in two summands with opposite signs (from swapping $i$ and $j$ in the alternation).
3. **Symmetry of second derivatives.** For $C^2$ functions, $\partial_i \partial_j f = \partial_j \partial_i f$ (Schwarz/Clairaut). The antisymmetriser therefore sends each pair to zero.
4. **Conclusion.** All terms cancel, giving $d(d\omega) = 0$.

The Mathlib proof uses `alternatizeUncurryFin_alternatizeUncurryFinCLM_comp_of_symmetric` together with the `isSymmSndFDerivAt` hypothesis for $C^2$ forms.

## Connections

The identity $d^2 = 0$ is the algebraic engine behind [[Stokes' Theorem (general)]], since it implies that the boundary of a boundary is empty. It also underpins the [[Poincaré Lemma]], which asks when $d\omega = 0$ forces $\omega = d\eta$ for some $\eta$.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.DifferentialForm.Basic

open VectorField

/-- The second exterior derivative of a C^2 form is zero.
    Wraps Mathlib's `extDeriv_extDeriv`. -/
theorem extDeriv_sq_zero {E F : Type*}
    [NontriviallyNormedField ℝ]
    [NormedAddCommGroup E] [NormedSpace ℝ E]
    [NormedAddCommGroup F] [NormedSpace ℝ F]
    {n : ℕ} {ω : E → E [⋀^Fin n]→L[ℝ] F}
    (h : ContDiff ℝ 2 ω) :
    extDeriv (extDeriv ω) = 0 :=
  extDeriv_extDeriv h (by norm_num [minSmoothness])
```
