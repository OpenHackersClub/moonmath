+++
title = "Jensen's Inequality"
description = "For a convex function phi, phi(E[X]) <= E[phi(X)] — the image of the mean is at most the mean of images"
weight = 40
tags = ["lean4-proof", "probability", "visualization"]
latex = "\\varphi(E[X]) \\leq E[\\varphi(X)]"
prerequisites = ["markov-inequality"]
lean4_status = "complete"
+++

## Statement

Let $\varphi : \mathbb{R} \to \mathbb{R}$ be a convex function and $X$ an integrable real-valued random variable. Then:

$$\varphi(E[X]) \leq E[\varphi(X)]$$

Equivalently, in finite-weight form: if $w_1, \ldots, w_n \geq 0$ with $\sum w_i = 1$ and $x_1, \ldots, x_n \in \mathbb{R}$, then:

$$\varphi\!\left(\sum_{i=1}^n w_i x_i\right) \leq \sum_{i=1}^n w_i \varphi(x_i)$$

## Visualization

Take $\varphi(x) = x^2$ (convex) and a fair die: $X \in \{1,2,3,4,5,6\}$, each with weight $1/6$.

**Mean first, then square:**

$$E[X] = \frac{1+2+3+4+5+6}{6} = 3.5 \quad \Rightarrow \quad \varphi(E[X]) = (3.5)^2 = 12.25$$

**Square first, then average:**

| $x$ | $x^2$ | weight $w$ | $w \cdot x^2$ |
|-----|-------|-----------|---------------|
| 1   | 1     | 1/6       | 1/6           |
| 2   | 4     | 1/6       | 4/6           |
| 3   | 9     | 1/6       | 9/6           |
| 4   | 16    | 1/6       | 16/6          |
| 5   | 25    | 1/6       | 25/6          |
| 6   | 36    | 1/6       | 36/6          |

$$E[X^2] = \frac{1+4+9+16+25+36}{6} = \frac{91}{6} \approx 15.17$$

Jensen: $12.25 \leq 15.17$. The gap $E[X^2] - (E[X])^2 = \text{Var}(X)$ is exactly the variance.

## Proof Sketch

1. **Tangent line at the mean.** Because $\varphi$ is convex, there exists a supporting hyperplane (tangent) at $\mu = E[X]$: a constant $c$ such that $\varphi(t) \geq \varphi(\mu) + c(t - \mu)$ for all $t$.
2. **Substitute $t = X$.** Pointwise: $\varphi(X) \geq \varphi(\mu) + c(X - \mu)$.
3. **Take expectations.** $E[\varphi(X)] \geq \varphi(\mu) + c \cdot (E[X] - \mu) = \varphi(\mu)$.

The discrete version follows the same idea with weighted sums replacing integrals.

## Connections

Jensen's inequality extends [[Cauchy–Schwarz Inequality]] to general convex functions: Cauchy–Schwarz is Jensen applied to $\varphi(t) = t^2$ with a specific pairing. It also underpins the [[AM–GM Inequality]]: apply Jensen to $\varphi = -\log$ (convex) with uniform weights to obtain $\log(\bar x) \geq \overline{\log x}$, i.e., AM $\geq$ GM.

## Lean4 Proof

The discrete finite-weight form lives in `Mathlib.Analysis.Convex.Jensen` as `ConvexOn.map_sum_le`.

```lean4
import Mathlib.Analysis.Convex.Jensen

namespace MoonMath

open Finset

/-- **Jensen's inequality** (finite discrete form).
    For a convex function `φ` on a convex set `s`, non-negative weights summing to 1,
    and points in `s`, `φ(Σ w_i x_i) ≤ Σ w_i φ(x_i)`. -/
theorem jensen_discrete {ι : Type*} (t : Finset ι)
    {φ : ℝ → ℝ} {s : Set ℝ} (hφ : ConvexOn ℝ s φ)
    {w : ι → ℝ} {x : ι → ℝ}
    (hw : ∀ i ∈ t, 0 ≤ w i) (hw1 : ∑ i ∈ t, w i = 1)
    (hx : ∀ i ∈ t, x i ∈ s) :
    φ (∑ i ∈ t, w i • x i) ≤ ∑ i ∈ t, w i • φ (x i) :=
  hφ.map_sum_le hw hw1 hx

end MoonMath
```

`ConvexOn.map_sum_le` is proved in Mathlib by induction on the finite set, applying the two-point convexity condition at each step.

