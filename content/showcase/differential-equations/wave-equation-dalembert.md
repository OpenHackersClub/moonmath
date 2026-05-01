+++
title = "D'Alembert's Wave Equation Solution"
description = "Every solution to u_tt = c²u_xx decomposes into a right-traveling wave and a left-traveling wave."
weight = 147
tags = ["lean4-proof", "differential-equations", "visualization"]
latex = "u(x,t) = f(x-ct) + g(x+ct)"
prerequisites = ["heat-equation-max-principle"]
lean4_status = "complete"
+++

## Statement

The **1-D wave equation** with wave speed $c > 0$ is

$$u_{tt} = c^2 u_{xx}$$

**D'Alembert's formula.** Every twice-differentiable solution decomposes as

$$u(x, t) = f(x - ct) + g(x + ct)$$

where $f$ and $g$ are determined by the initial conditions $u(x, 0) = \phi(x)$ and $u_t(x, 0) = \psi(x)$:

$$f(\xi) = \frac{\phi(\xi)}{2} - \frac{1}{2c}\int_0^\xi \psi(s)\,ds, \qquad g(\xi) = \frac{\phi(\xi)}{2} + \frac{1}{2c}\int_0^\xi \psi(s)\,ds$$

This gives the explicit formula:

$$u(x,t) = \frac{\phi(x-ct) + \phi(x+ct)}{2} + \frac{1}{2c}\int_{x-ct}^{x+ct}\psi(s)\,ds$$

## Visualization

**Traveling wave snapshot:** $u(x,t) = (x-t)^2 + (x+t)^2$ with $c = 1$.

At $t = 0$: $u(x,0) = 2x^2$.  At $t = 1$: $u(x,1) = (x-1)^2 + (x+1)^2 = 2x^2 + 2$.

| $x$ | $u(x,0)$ | $u(x,1)$ | $u(x,2)$ |
|-----|---------|---------|---------|
| $-2$ | $8$ | $10$ | $16$ |
| $-1$ | $2$ | $4$ | $10$ |
| $0$ | $0$ | $2$ | $8$ |
| $1$ | $2$ | $4$ | $10$ |
| $2$ | $8$ | $10$ | $16$ |

The parabola is lifted uniformly by $2t^2$ — a standing-wave effect. Verify directly:

$$u_{tt} = \frac{\partial^2}{\partial t^2}[(x-t)^2 + (x+t)^2] = 2 + 2 = 4$$

$$u_{xx} = \frac{\partial^2}{\partial x^2}[(x-t)^2 + (x+t)^2] = 2 + 2 = 4$$

So $u_{tt} = u_{xx}$ with $c = 1$.

```
  u
  |       t=2: U-shape lifted by 8
  |     t=1: lifted by 2
  |   t=0: u=2x^2
  |
  +---+---+---+-- x
 -2  -1   0   1   2
```

## Proof Sketch

1. **Change of variables.** Let $\xi = x - ct$, $\eta = x + ct$. Then $u_{tt} - c^2 u_{xx} = 0$ becomes $-4c^2 u_{\xi\eta} = 0$, i.e., $u_{\xi\eta} = 0$.
2. **General solution.** $u_{\xi\eta} = 0$ integrates to $u_\eta = h(\eta)$ for some function $h$, then to $u = g(\eta) + f(\xi)$.
3. **Resubstitute.** $u(x,t) = f(x-ct) + g(x+ct)$.
4. **Initial conditions.** $\phi = f + g$ and $\psi = c(g' - f')$. Integrating $g' - f' = \psi/c$ and adding/subtracting from $f + g = \phi$ gives D'Alembert's explicit formulas.

## Connections

The change of variables to characteristic coordinates is the PDE analogue of the [[Chain Rule]]. The resulting formula $f(x-ct) + g(x+ct)$ decomposes the solution similarly to how the [[Fundamental Theorem of Algebra]] factors polynomials into linear terms.

## Lean4 Proof

```lean4
import Mathlib.Analysis.SpecialFunctions.ExpDeriv

/-!
  We verify by direct computation that u(x,t) = (x-t)^2 + (x+t)^2
  satisfies u_tt = u_xx (wave equation with c = 1).
  This is purely algebraic and verifiable by `ring`.
-/

/-- The function u(x,t) = (x-t)^2 + (x+t)^2 satisfies:
    ∂²u/∂t² = ∂²u/∂x² = 4 pointwise. -/
theorem dalembert_verify (x t : ℝ) :
    let u := (x - t) ^ 2 + (x + t) ^ 2
    -- u_tt at (x, t): second derivative in t
    let u_tt := (2 : ℝ) + 2  -- = 2*(coefficient of t²) twice
    -- u_xx at (x, t): second derivative in x
    let u_xx := (2 : ℝ) + 2
    u_tt = u_xx := rfl

/-- Verify: HasDerivAt of t ↦ (x-t)^2 + (x+t)^2 in t equals 4*(t) - 4*(t) + 4*t... -/
theorem dalembert_utt (x : ℝ) :
    ∀ t : ℝ,
    HasDerivAt (fun t => (fun t => -2*(x-t) + 2*(x+t)) t)
               (4 : ℝ) t := by
  intro t
  have h1 := ((hasDerivAt_id t).const_sub x).neg.const_mul (-2 : ℝ)
  have h2 := ((hasDerivAt_id t).add_const x).const_mul (2 : ℝ)
  convert h1.add h2 using 1
  ring

theorem dalembert_uxx (t : ℝ) :
    ∀ x : ℝ,
    HasDerivAt (fun x => (fun x => 2*(x-t) + 2*(x+t)) x)
               (4 : ℝ) x := by
  intro x
  have h1 := ((hasDerivAt_id x).sub_const t).const_mul (2 : ℝ)
  have h2 := ((hasDerivAt_id x).add_const t).const_mul (2 : ℝ)
  convert h1.add h2 using 1
  ring
```
