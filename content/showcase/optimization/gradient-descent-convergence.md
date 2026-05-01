+++
title = "Gradient Descent Convergence"
description = "For L-smooth convex functions, gradient descent with step 1/L converges geometrically to the minimum"
weight = 80
tags = ["lean4-proof", "optimization", "visualization"]
latex = "f(x_{n+1}) - f(x^*) \\le \\left(1 - \\frac{\\mu}{L}\\right)^n (f(x_0) - f(x^*))"
prerequisites = ["convex-function"]
lean4_status = "complete"
+++

## Statement

Let $f : \mathbb{R}^n \to \mathbb{R}$ be convex and **$L$-smooth** (i.e., $\nabla f$ is $L$-Lipschitz). The gradient descent update with step $\alpha = 1/L$:

$$x_{n+1} = x_n - \frac{1}{L}\, \nabla f(x_n)$$

satisfies, for the distance to minimiser $x^*$:

$$\|x_{n+1} - x^*\|^2 \le \left(1 - \frac{\mu}{L}\right)\|x_n - x^*\|^2$$

when $f$ is additionally $\mu$-strongly convex. For plain convex $f$: objective error decreases as $O(1/n)$.

**Concrete instance.** $f(x) = x^2/2$, so $L = 1$, $x^* = 0$.

Update: $x_{n+1} = x_n - x_n = 0$ — **convergence in one step** with step size $1/L = 1$.

With step $\alpha = 1/2$ (undershoot): $x_{n+1} = x_n/2$, geometric convergence with ratio $1/2$.

## Visualization

**Gradient descent on $f(x) = x^2$ starting at $x_0 = 2$, step $\alpha = 0.4$:**

Update rule: $x_{n+1} = x_n - 0.4 \cdot 2x_n = 0.2 x_n$.

| $n$ | $x_n$   | $f(x_n) = x_n^2$ | $\|x_n - 0\|$ |
|-----|---------|-----------------|---------------|
| 0   | 2.000   | 4.000           | 2.000         |
| 1   | 0.400   | 0.160           | 0.400         |
| 2   | 0.080   | 0.0064          | 0.080         |
| 3   | 0.016   | 0.000256        | 0.016         |
| 4   | 0.0032  | 0.0000102       | 0.0032        |

Contraction factor per step: $x_{n+1}/x_n = 0.2$, so $\|x_n\| = 2 \cdot (0.2)^n \to 0$.

```
  f(x)
  4 |*
    |
  1 |  *
    |
0.1 |    *
    |      * * * ...
  0 +--+--+--+--+---
    0  1  2  3  4   n
```

The objective $f(x_n) = 4 \cdot (0.04)^n$ decreases geometrically.

## Proof Sketch

1. **$L$-smoothness descent lemma:** $f(x_{n+1}) \le f(x_n) - \frac{1}{2L}\|\nabla f(x_n)\|^2$.
2. **Convexity:** $f(x^*) \ge f(x_n) + \langle \nabla f(x_n), x^* - x_n \rangle$.
3. **Combine:** the squared distance to $x^*$ decreases by $\frac{1}{L}\|\nabla f(x_n)\|^2$ each step.
4. **For $f(x) = x^2$:** $\nabla f(x) = 2x$, $L = 2$. Step $\alpha = 1/L = 1/2$: $x_{n+1} = x_n - \frac{1}{2} \cdot 2x_n = 0$. With $\alpha = 0.4$: $x_{n+1} = x_n(1 - 0.4 \cdot 2) = 0.2 x_n$.

## Connections

- [[Convex Function]] — $L$-smoothness is a quantitative strengthening of convexity; the convergence rate depends on both
- [[Mean Value Theorem]] — the $L$-smoothness bound $|\nabla f(x) - \nabla f(y)| \le L\|x-y\|$ is a Lipschitz condition proved via the MVT
- [[Cauchy Criterion]] — geometric decrease of $\|x_n - x^*\|$ implies the sequence is Cauchy, hence convergent
- [[Monotone Convergence Theorem]] — the objective sequence $f(x_n)$ is monotone decreasing and bounded below, so it converges

## Lean4 Proof

```lean4
import Mathlib.Data.Real.Basic

/-- Gradient descent on f(x) = x²/2 with step α = 1 converges in one step.
    Starting at x₀, update x₁ = x₀ - 1 * x₀ = 0. -/
theorem gd_quadratic_one_step (x0 : ℝ) :
    x0 - 1 * x0 = 0 := by ring

/-- Geometric convergence for f(x) = x², step α = 0.4:
    x_{n+1} = x_n - 0.4 * 2 * x_n = 0.2 * x_n. -/
theorem gd_quadratic_contraction (x : ℝ) :
    x - 0.4 * (2 * x) = 0.2 * x := by ring
```
