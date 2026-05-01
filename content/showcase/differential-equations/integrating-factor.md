+++
title = "Integrating Factor Method"
description = "Multiplying a first-order linear ODE by e^{∫p dx} converts it into an exact derivative and yields a closed-form solution."
weight = 143
tags = ["lean4-proof", "differential-equations", "visualization"]
latex = "\\mu(x)=e^{\\int p(x)\\,dx},\\quad (\\mu y)'=\\mu q"
prerequisites = ["linear-ode-general"]
lean4_status = "complete"
+++

## Statement

The first-order linear ODE

$$y' + p(x)\,y = q(x)$$

is solved by choosing the **integrating factor** $\mu(x) = e^{\int p(x)\,dx}$. Multiplying both sides gives

$$(\mu y)' = \mu q$$

which integrates to

$$y(x) = \frac{1}{\mu(x)}\!\left(\int \mu(x)\,q(x)\,dx + C\right)$$

**Worked example:** $y' + y = e^x$. Here $p \equiv 1$, so $\mu = e^x$.

$$e^x y' + e^x y = e^{2x} \implies (e^x y)' = e^{2x}$$

Integrating: $e^x y = \tfrac{1}{2}e^{2x} + C$, giving $y = \tfrac{1}{2}e^x + Ce^{-x}$.

## Visualization

**Solution to $y' + 2xy = x$ with $y(0) = 0$:**

Integrating factor: $\mu = e^{x^2}$. Then $(e^{x^2} y)' = x e^{x^2}$, integrating:

$$y(x) = \frac{1}{2} + Ce^{-x^2}, \qquad y(0) = 0 \implies C = -\tfrac{1}{2}$$

$$y(x) = \frac{1}{2}(1 - e^{-x^2})$$

| $x$ | $e^{-x^2}$ | $y(x) = \tfrac{1}{2}(1-e^{-x^2})$ |
|-----|------------|-----------------------------------|
| 0.0 | 1.000      | 0.000                             |
| 0.5 | 0.779      | 0.111                             |
| 1.0 | 0.368      | 0.316                             |
| 1.5 | 0.105      | 0.448                             |
| 2.0 | 0.018      | 0.491                             |
| 3.0 | 0.000      | 0.500                             |

The solution rises from $0$ and saturates at $y = 1/2$ as $x \to \infty$ — controlled by the Gaussian decay in $e^{-x^2}$.

## Proof Sketch

1. **Define $\mu(x) = e^{\int_{x_0}^x p(s)\,ds}$.** Then $\mu' = p\mu$.
2. **Product rule.** $(\mu y)' = \mu' y + \mu y' = p\mu y + \mu(q - py) = \mu q$.
3. **Integrate.** $\mu(x) y(x) = \int_{x_0}^x \mu(s) q(s)\,ds + C$.
4. **Divide by $\mu(x) \neq 0$.** (Exponential is always positive.) This gives the explicit formula.

The key identity is the product rule from the [[Fundamental Theorem of Calculus]] and the chain rule $\mu' = p\mu$.

## Connections

The integrating factor $e^{\int p}$ is the scalar instance of the matrix exponential in [[Linear ODE General Solution]]; the product rule step mirrors the computation in [[Chain Rule]].

## Lean4 Proof

```lean4
import Mathlib.Analysis.SpecialFunctions.ExpDeriv

/-!
  We verify the integrating factor identity for y' + y = eˣ.
  The claim: y(x) = (1/2)*exp(x) + C*exp(-x) satisfies y' + y = exp(x).
  We verify this by direct differentiation using `ring` after collecting
  derivative facts.
-/

/-- For any constant C, y(x) = (1/2)*exp(x) + C*exp(-x)
    satisfies y'(x) + y(x) = exp(x). -/
theorem integrating_factor_example (C : ℝ) :
    let y  : ℝ → ℝ := fun x => (1/2) * Real.exp x + C * Real.exp (-x)
    let y' : ℝ → ℝ := fun x => (1/2) * Real.exp x - C * Real.exp (-x)
    ∀ x : ℝ, y' x + y x = Real.exp x := by
  intro y y' x
  simp only [y, y']
  ring

/-- The derivative of y(x) = (1/2)*exp(x) + C*exp(-x) equals y'(x). -/
theorem integrating_factor_deriv (C : ℝ) (x : ℝ) :
    HasDerivAt (fun x => (1/2 : ℝ) * Real.exp x + C * Real.exp (-x))
               ((1/2) * Real.exp x - C * Real.exp (-x)) x := by
  have h1 := (Real.hasDerivAt_exp x).const_mul (1/2 : ℝ)
  have h2 := ((Real.hasDerivAt_exp (-x)).comp x
               (hasDerivAt_neg x)).const_mul C
  simp only [neg_neg] at h2
  convert h1.add h2 using 1
  ring
```
