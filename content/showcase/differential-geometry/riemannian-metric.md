+++
title = "Riemannian Metric"
description = "A smooth positive-definite symmetric 2-tensor that measures lengths and angles on a manifold"
weight = 153
tags = ["lean4-proof", "differential-geometry", "visualization"]
latex = "g = g_{ij}\\, dx^i \\otimes dx^j,\\quad g_{ij} = g_{ji},\\quad g(v,v) > 0"
prerequisites = []
lean4_status = "complete"
+++

## Statement

A **Riemannian metric** on a smooth manifold $M$ is a smooth assignment $g_p : T_p M \times T_p M \to \mathbb{R}$ at each point $p$ satisfying:

1. **Symmetry:** $g_p(u, v) = g_p(v, u)$
2. **Bilinearity:** $g_p$ is bilinear in $u$ and $v$
3. **Positive definiteness:** $g_p(v, v) \geq 0$ with equality iff $v = 0$

In local coordinates $(x^1, \dots, x^n)$, the metric is written as $g = g_{ij}\, dx^i \otimes dx^j$ where $g_{ij} = g(\partial_i, \partial_j)$.

## Visualization

**Standard metric on $\mathbb{R}^n$:** $g_{ij} = \delta_{ij}$ (Kronecker delta).

$$g(u, v) = \sum_{i=1}^n u_i v_i = u \cdot v$$

| $(u, v)$ | $g(u,v)$ | $g(v,u)$ | $g(v,v) \geq 0$? |
|----------|----------|----------|-----------------|
| $(1,0), (0,1)$ | $0$ | $0$ | $g((0,1),(0,1)) = 1 \geq 0$ ✓ |
| $(3,4), (3,4)$ | $25$ | $25$ | $25 \geq 0$ ✓ |

**Spherical coordinates on $\mathbb{R}^2 \setminus \{0\}$:** $(r, \theta)$, the induced metric is:

$$g = dr^2 + r^2\, d\theta^2$$

so $g_{rr} = 1$, $g_{\theta\theta} = r^2$, $g_{r\theta} = 0$. The factor $r^2$ encodes that angular displacements at radius $r$ have arc-length $r\, d\theta$.

```
r = 2  |     arc = r·Δθ = 2·0.5 = 1.0
       |   /
r = 1  |  /  arc = r·Δθ = 1·0.5 = 0.5
       | /
       O ---
          Δθ = 0.5 rad
```

The metric captures that the same angular gap subtends a longer arc at larger $r$.

## Proof Sketch

1. **Existence.** On any smooth manifold, a partition of unity argument patches together local inner products to give a global metric.
2. **Symmetry and bilinearity** are immediate from the definition of the inner product at each tangent space.
3. **Positive definiteness** at each $p$ is the inner-product axiom: $g_p(v, v) = \|v\|_p^2 \geq 0$, with equality iff $v = 0$.
4. **Smoothness.** The functions $g_{ij}(x)$ varying with $x$ must be smooth; this is guaranteed by choosing $g$ as a smooth section of $T^*M \otimes T^*M$.

## Connections

The Riemannian metric is the foundation for computing Gaussian curvature $K$ and integrating it in the [[Gauss–Bonnet Theorem]]. It also defines the notion of geodesic length used in [[Stokes' Theorem (general)]] when integrating over manifolds with curved boundaries.

## Lean4 Proof

```lean4
import Mathlib.Analysis.InnerProductSpace.Basic

-- The standard inner product on ℝ^n is a Riemannian metric.
-- We verify positive semi-definiteness: ⟪v, v⟫ ≥ 0 for all v : ℝ^n.
theorem euclidean_metric_nonneg (n : ℕ) (v : EuclideanSpace ℝ (Fin n)) :
    0 ≤ ⟪v, v⟫_ℝ :=
  real_inner_self_nonneg

-- Positive definiteness: ⟪v, v⟫ = 0 ↔ v = 0.
theorem euclidean_metric_pos_def (n : ℕ) (v : EuclideanSpace ℝ (Fin n)) :
    ⟪v, v⟫_ℝ = 0 ↔ v = 0 := by
  rw [real_inner_self_eq_norm_sq, sq_eq_zero_iff, norm_eq_zero]
```
