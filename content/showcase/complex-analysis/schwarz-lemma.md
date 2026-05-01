+++
title = "Schwarz Lemma"
description = "A holomorphic self-map of the unit disk fixing the origin satisfies |f(z)| ≤ |z| and |f'(0)| ≤ 1."
weight = 40
tags = ["lean4-proof", "complex-analysis", "visualization"]
latex = "|f(z)| \\le |z|,\\quad |f'(0)| \\le 1 \\quad\\text{for } f: \\mathbb{D}\\to\\overline{\\mathbb{D}},\\; f(0)=0"
prerequisites = ["maximum-modulus", "cauchy-integral-formula"]
lean4_status = "complete"
+++

## Statement

Let $\mathbb{D} = \{z \in \mathbb{C} : |z| < 1\}$ be the open unit disk. If $f : \mathbb{D} \to \overline{\mathbb{D}}$ is holomorphic with $f(0) = 0$, then:

$$|f(z)| \le |z| \quad \text{for all } z \in \mathbb{D}$$
$$|f'(0)| \le 1$$

Moreover, equality $|f(z_0)| = |z_0|$ for some $z_0 \ne 0$ (or $|f'(0)| = 1$) forces $f$ to be a rotation: $f(z) = e^{i\theta}z$.

## Visualization

**$f(z) = z^2$**: satisfies $f(0) = 0$, maps $\mathbb{D}$ into $\mathbb{D}$.

| $z$ | $|z|$ | $|f(z)| = |z|^2$ | $|f(z)| \le |z|$? |
|-----|-------|-----------------|-------------------|
| $0.2$ | $0.2$ | $0.04$ | Yes ($0.04 \le 0.2$) |
| $0.5$ | $0.5$ | $0.25$ | Yes ($0.25 \le 0.5$) |
| $0.8$ | $0.8$ | $0.64$ | Yes ($0.64 \le 0.8$) |
| $0.99$ | $0.99$ | $0.9801$ | Yes |

The gap $|z| - |f(z)| = |z| - |z|^2 = |z|(1 - |z|) \ge 0$ is always positive for $0 < |z| < 1$.

The rotation $f(z) = e^{i\alpha}z$ saturates the bound: $|f(z)| = |z|$ for all $z$.

```
Unit disk cross-section along real axis:
  -1         0         1
   |----[----*----]-----|
        |z|=0.5
   |f(z)| = 0.25 < 0.5 = |z|    (for f(z)=z^2)
```

## Proof Sketch

1. Define $g(z) = f(z)/z$ for $z \ne 0$ and $g(0) = f'(0)$. Since $f(0) = 0$, $g$ extends to a holomorphic function on $\mathbb{D}$.
2. On the boundary circle $|z| = r < 1$: $|g(z)| = |f(z)|/r \le 1/r$.
3. By the Maximum Modulus Principle, $|g(z)| \le 1/r$ on $|z| \le r$. Sending $r \to 1$: $|g(z)| \le 1$ on $\mathbb{D}$.
4. So $|f(z)| = |z||g(z)| \le |z|$ and $|f'(0)| = |g(0)| \le 1$.
5. If $|g(z_0)| = 1$ for any $z_0 \in \mathbb{D}$, the Maximum Modulus Principle forces $g$ to be a constant $e^{i\theta}$, so $f(z) = e^{i\theta}z$.

## Connections

The Schwarz Lemma is the simplest rigidity theorem for holomorphic maps and is closely related to the [[Maximum Modulus Principle]] (the proof is a direct application). It drives hyperbolic geometry on the disk: the automorphisms of $\mathbb{D}$ fixing the origin are exactly rotations. Compare with the [[Cauchy Integral Formula]], which also bounds derivatives by boundary values.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Complex.Schwarz

open Complex Metric

/-- **Schwarz Lemma**: if `f` is holomorphic on the unit disk, maps it into
    the closed unit disk, and satisfies `f 0 = 0`, then `‖f z‖ ≤ ‖z‖`.
    Uses `Complex.norm_le_norm_of_mapsTo_ball` from Mathlib. -/
theorem schwarz_lemma {f : ℂ → ℂ}
    (hd : DifferentiableOn ℂ f (ball 0 1))
    (h_maps : MapsTo f (ball 0 1) (closedBall 0 1))
    (h0 : f 0 = 0)
    {z : ℂ} (hz : ‖z‖ < 1) :
    ‖f z‖ ≤ ‖z‖ :=
  Complex.norm_le_norm_of_mapsTo_ball hd h_maps h0 hz
```
