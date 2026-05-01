+++
title = "Inverse Function Theorem"
description = "A smooth map with invertible derivative at a point is locally a diffeomorphism"
weight = 110
tags = ["lean4-proof", "calculus", "visualization"]
latex = "f'(a)\\neq 0\\implies\\exists\\,g:\\;g(f(x))=x\\text{ near }a"
prerequisites = ["implicit-function-theorem", "chain-rule"]
lean4_status = "complete"
+++

## Statement

Let $f : \mathbb{R} \to \mathbb{R}$ be continuously differentiable near $a$ with $f'(a) \neq 0$. Then there exists an open interval $U \ni a$ and an open interval $V \ni f(a)$ such that $f : U \to V$ is a bijection with a continuously differentiable inverse $g = f^{-1} : V \to U$. Moreover

$$g'(f(a)) = \frac{1}{f'(a)}$$

In higher dimensions: if $f : \mathbb{R}^n \to \mathbb{R}^n$ has invertible Jacobian $Df(a)$, then $f$ is a local $C^1$-diffeomorphism and $Dg(f(a)) = (Df(a))^{-1}$.

## Visualization

**Example:** $f(x) = x^3 + x$ near $x = 0$.

$f'(x) = 3x^2 + 1$, so $f'(0) = 1 \neq 0$. The IFT guarantees a local inverse $g$ near $y = f(0) = 0$:

```
  y = f(x) = x³ + x       y = g(y) (inverse)

  2 |        /              2 |           /
    |      /                  |         /
  1 |    /                  1 |       /
    |   /  ← slope=1 at 0     |     /  ← slope=1 at 0
  0 |--*--               0 |--*--
    |  /                     |  /
 -1 | /                   -1 | /
    +--+--+--               +--+--+--
      -1  0  1                -1  0  1
```

| $x$   | $f(x) = x^3+x$ | $g'(f(x)) = 1/f'(x)$ |
|-------|---------------|---------------------|
| 0.0   | 0.000         | 1.000               |
| 0.5   | 0.625         | $1/(1.75) \approx 0.571$ |
| 1.0   | 2.000         | $1/4 = 0.250$       |

The inverse $g$ satisfies $g(0) = 0$, $g(2) = 1$, and $g'(0) = 1/f'(0) = 1$.

## Proof Sketch

1. **Strict derivative:** Replace the $C^1$ hypothesis with the equivalent strict differentiability at $a$ (valid for $C^1$ maps).
2. **Approximate linearly:** Near $a$, $f(x) - f(y) \approx f'(a)(x - y)$. Since $f'(a) \neq 0$, the linear approximation is a bijection.
3. **Contraction mapping:** On a small ball, $f$ is an approximate linear isometry; the fixed-point theorem yields a local right inverse $g$ with $f(g(y)) = y$.
4. **Differentiability of $g$:** Differentiate $f(g(y)) = y$ using the chain rule: $f'(g(y)) \cdot g'(y) = 1$, so $g'(y) = 1/f'(g(y))$.

## Connections

- [[Implicit Function Theorem]] — the IFT is proved by applying the IFT to $\Phi(x,y) = (x, F(x,y))$; conversely the IFT follows from the IFT
- [[Chain Rule]] — differentiating $f(g(y)) = y$ uses the chain rule to obtain the inverse derivative formula
- [[Mean Value Theorem]] — the MVT is used in the estimate $|f(x) - f(y) - f'(a)(x-y)| \leq \varepsilon|x-y|$ that makes $f$ a local contraction

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.InverseFunctionTheorem.Deriv

/-- Inverse Function Theorem (1D): if `f` has a non-zero strict derivative at `a`,
    then the local inverse has derivative `f'⁻¹` at `f a`.
    Wraps `HasStrictDerivAt.to_localInverse`. -/
theorem ift_1d (f : ℝ → ℝ) (f' a : ℝ)
    (hf : HasStrictDerivAt f f' a) (hf' : f' ≠ 0) :
    HasStrictDerivAt (hf.localInverse f f' a hf') f'⁻¹ (f a) :=
  hf.to_localInverse hf'
```
