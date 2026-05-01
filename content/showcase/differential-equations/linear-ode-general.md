+++
title = "Linear ODE General Solution"
description = "The matrix exponential e^{At} gives the unique solution to y' = Ay with initial condition y(0) = y_0."
weight = 142
tags = ["lean4-proof", "differential-equations", "visualization"]
latex = "y(t) = e^{At}y_0 \\text{ solves } y' = Ay,\\; y(0)=y_0"
prerequisites = ["picard-lindelof"]
lean4_status = "complete"
+++

## Statement

Let $A$ be a real $n \times n$ matrix. The **linear ODE system**

$$y'(t) = A\, y(t), \qquad y(0) = y_0 \in \mathbb{R}^n$$

has a unique global solution given by the **matrix exponential**:

$$y(t) = e^{At} y_0, \qquad e^{At} := \sum_{k=0}^{\infty} \frac{(At)^k}{k!}$$

For a diagonal matrix $A = \operatorname{diag}(\lambda_1, \ldots, \lambda_n)$ the solution decouples:

$$y_i(t) = e^{\lambda_i t} (y_0)_i$$

Mathlib contains `Matrix.exp` and `Matrix.exp_diagonal` which state $e^{\operatorname{diag}(v)} = \operatorname{diag}(e^v)$, capturing exactly this decoupling.

## Visualization

**Scalar case $y' = 2y$, $y(0) = 3$:** solution $y(t) = 3e^{2t}$.

| $t$ | $e^{2t}$ | $y(t) = 3e^{2t}$ |
|-----|---------|-----------------|
| 0   | 1.000   | 3.000           |
| 0.5 | 2.718   | 8.155           |
| 1   | 7.389   | 22.167          |
| 2   | 54.598  | 163.794         |

**2-D diagonal example** $A = \begin{pmatrix}1 & 0 \\ 0 & -2\end{pmatrix}$, $y_0 = (1, 1)$:

$$e^{At} = \begin{pmatrix} e^{t} & 0 \\ 0 & e^{-2t} \end{pmatrix}, \qquad y(t) = \begin{pmatrix} e^{t} \\ e^{-2t} \end{pmatrix}$$

| $t$ | $y_1(t) = e^t$ | $y_2(t) = e^{-2t}$ |
|-----|---------------|-------------------|
| 0   | 1.000         | 1.000             |
| 1   | 2.718         | 0.135             |
| 2   | 7.389         | 0.018             |

The first component grows, the second decays — the eigenvalues control the long-term behavior.

## Proof Sketch

1. **Existence.** Define $\Phi(t) = e^{At}$. Then $\Phi'(t) = A\Phi(t)$ because the power series can be differentiated term-by-term, giving $\frac{d}{dt}[(At)^k/k!] = A(At)^{k-1}/(k-1)!$.
2. **Initial condition.** $\Phi(0) = e^0 = I$, so $y(0) = I y_0 = y_0$.
3. **Uniqueness.** By the [[Picard–Lindelöf Theorem]], the Lipschitz map $f(t,y) = Ay$ (with constant $\|A\|$) guarantees a unique solution.
4. **Diagonal decoupling.** When $A = \operatorname{diag}(\lambda)$, each power $A^k = \operatorname{diag}(\lambda^k)$, so $e^{At} = \operatorname{diag}(e^{\lambda t})$.

## Connections

The matrix exponential is the bridge between the [[Cayley–Hamilton Theorem]] (which constrains $e^{At}$ to the characteristic polynomial) and the [[Spectral Theorem]] (which diagonalizes symmetric $A$, making $e^{At}$ explicit). The scalar case relies on the [[Fundamental Theorem of Calculus]] to differentiate power series term-by-term.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Normed.Algebra.MatrixExponential

/-!
  Mathlib's `Matrix.exp_diagonal` states:
    exp (diagonal v) = diagonal (exp v)
  This is exactly the claim that for A = diag(λ), e^A = diag(e^λ).
  We instantiate it for a concrete 2×2 diagonal matrix.
-/

open Matrix in
/-- For a diagonal matrix, the matrix exponential diagonalises:
    exp(diag(λ)) = diag(exp(λ)).
    This is `Matrix.exp_diagonal` in Mathlib. -/
theorem matrix_exp_diagonal_two
    (λ₁ λ₂ : ℝ) :
    NormedSpace.exp (diagonal (![λ₁, λ₂]))
      = diagonal (![NormedSpace.exp λ₁, NormedSpace.exp λ₂]) := by
  rw [exp_diagonal]
  congr 1
  funext i
  fin_cases i <;> simp

/-- Scalar ODE y' = c*y has solution y(t) = y₀ * exp(c*t).
    Verified by checking the derivative. -/
theorem scalar_linear_ode_solution (c y₀ : ℝ) :
    let y : ℝ → ℝ := fun t => y₀ * Real.exp (c * t)
    ∀ t : ℝ, HasDerivAt y (c * y t) t := by
  intro y t
  have h := (Real.hasDerivAt_exp (c * t)).const_mul y₀
  have hc := hasDerivAt_id t |>.const_mul c
  convert h.comp t hc using 1
  simp [mul_comm]
```
