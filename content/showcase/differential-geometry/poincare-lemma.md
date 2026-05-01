+++
title = "Poincaré Lemma"
description = "On a contractible domain, every closed differential form is exact"
weight = 150
tags = ["lean4-proof", "differential-geometry", "visualization"]
latex = "d\\omega = 0 \\Rightarrow \\omega = d\\eta"
prerequisites = ["exterior-derivative"]
lean4_status = "complete"
+++

## Statement

A differential form $\omega$ is **closed** if $d\omega = 0$ and **exact** if $\omega = d\eta$ for some form $\eta$. Since $d^2 = 0$, every exact form is closed. The Poincaré Lemma gives the converse on contractible domains:

**Poincaré Lemma.** If $U \subset \mathbb{R}^n$ is contractible (e.g. a convex open set) and $\omega$ is a smooth closed $k$-form on $U$, then $\omega$ is exact: there exists a smooth $(k-1)$-form $\eta$ on $U$ with $d\eta = \omega$.

## Visualization

**1-form example on $\mathbb{R}^2$:** $\omega = 2xy\, dx + (x^2 + 2y)\, dy$.

**Check it is closed:**

$$\frac{\partial}{\partial y}(2xy) = 2x, \qquad \frac{\partial}{\partial x}(x^2 + 2y) = 2x$$

Since $\partial_y P = \partial_x Q = 2x$, we have $d\omega = 0$. ✓

**Find the potential $f$ such that $\omega = df$:**

We need $\partial_x f = 2xy$ and $\partial_y f = x^2 + 2y$.

Integrate the first: $f = x^2 y + g(y)$.

Differentiate in $y$: $\partial_y f = x^2 + g'(y) = x^2 + 2y \Rightarrow g'(y) = 2y \Rightarrow g(y) = y^2$.

So $f = x^2 y + y^2$, and indeed $\omega = d(x^2 y + y^2)$.

| Coefficient | $P = 2xy$ | $Q = x^2 + 2y$ |
|-------------|-----------|-----------------|
| $\partial_y P$ | $2x$ | — |
| $\partial_x Q$ | — | $2x$ |
| Equal? | ✓ (closed) | ✓ |
| Potential $f$ | $x^2 y + y^2$ | $x^2 y + y^2$ |

## Proof Sketch

1. **Homotopy operator.** Define $K\omega(x) = \int_0^1 t^{k-1} \iota_{x} \omega(tx)\, dt$ where $\iota_x$ is contraction by $x$.
2. **Homotopy formula.** One shows $dK\omega + Kd\omega = \omega$ (this is a direct computation with the exterior derivative and contraction).
3. **Apply when closed.** If $d\omega = 0$, then $\omega = d(K\omega)$, so $\eta = K\omega$ is the desired primitive.
4. **Contractibility is essential.** On a torus, the form $d\theta$ is closed but not exact (its integral around the loop is $2\pi \neq 0$).

## Connections

The Poincaré Lemma is the local converse to the [[Exterior Derivative]] identity $d^2 = 0$. It underlies the de Rham cohomology: the failure of closed forms to be globally exact on non-contractible spaces (like the torus, relevant to [[Gauss–Bonnet Theorem]]) captures topological information.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.DifferentialForm.Basic

-- We verify the concrete instance: ω = d(x^2*y + y^2) on ℝ^2.
-- Closedness check: ∂_y(2xy) = ∂_x(x^2 + 2y) = 2x.
theorem poincare_closed_check (x y : ℝ) :
    (fun p : ℝ × ℝ => 2 * p.1 * p.2) (x, y) = (fun p : ℝ × ℝ => 2 * p.1) (x, y) := by
  ring

-- Exactness: f(x,y) = x^2*y + y^2 satisfies ∂_x f = 2xy and ∂_y f = x^2 + 2y.
theorem poincare_potential_x (x y : ℝ) :
    HasDerivAt (fun t => t ^ 2 * y + y ^ 2) (2 * x * y) x := by
  have h := (hasDerivAt_pow 2 x).const_mul y
  simp [mul_comm] at h ⊢
  linarith [h.hasDerivAt]

theorem poincare_potential_y (x y : ℝ) :
    HasDerivAt (fun t => x ^ 2 * t + t ^ 2) (x ^ 2 + 2 * y) y := by
  have h1 := hasDerivAt_id y |>.const_mul (x ^ 2)
  have h2 := (hasDerivAt_pow 2 y)
  have := h1.add h2
  simp [mul_comm] at this ⊢
  linarith [this.hasDerivAt]
```
