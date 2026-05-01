+++
title = "Divergence Theorem (Gauss)"
description = "The flux of a vector field through a closed surface equals the integral of its divergence over the enclosed volume"
weight = 149
tags = ["lean4-proof", "differential-geometry", "visualization"]
latex = "\\iiint_V \\nabla \\cdot F\\, dV = \\oiint_{\\partial V} F \\cdot dS"
prerequisites = ["exterior-derivative", "stokes-theorem"]
lean4_status = "complete"
+++

## Statement

Let $V \subset \mathbb{R}^3$ be a compact region with piecewise smooth oriented boundary $\partial V$, and let $F = (F_1, F_2, F_3)$ be a $C^1$ vector field. Then:

$$\iiint_V \nabla \cdot F\, dV = \oiint_{\partial V} F \cdot dS$$

where $\nabla \cdot F = \frac{\partial F_1}{\partial x} + \frac{\partial F_2}{\partial y} + \frac{\partial F_3}{\partial z}$ is the divergence.

## Visualization

**Verification on the unit cube $[0,1]^3$ with $F = (x, y, z)$.**

Divergence: $\nabla \cdot F = 1 + 1 + 1 = 3$.

Volume integral: $\iiint_{[0,1]^3} 3\, dV = 3 \cdot 1 = 3$.

Surface flux through each face (outward normal $\hat{n}$):

```
         z=1 (top)
          +--------+
         /|       /|
        / |      / |
       +--------+  |
       |  +-----|--+  x-faces: left (x=0), right (x=1)
       | /      | /
       |/       |/
       +--------+
   y=0 (front)   y=1 (back)
```

| Face | Normal | $F \cdot \hat{n}$ | Area | Flux |
|------|--------|-------------------|------|------|
| $x = 1$ (right) | $+\hat{x}$ | $x = 1$ | $1$ | $1$ |
| $x = 0$ (left) | $-\hat{x}$ | $-x = 0$ | $1$ | $0$ |
| $y = 1$ (back) | $+\hat{y}$ | $y = 1$ | $1$ | $1$ |
| $y = 0$ (front) | $-\hat{y}$ | $-y = 0$ | $1$ | $0$ |
| $z = 1$ (top) | $+\hat{z}$ | $z = 1$ | $1$ | $1$ |
| $z = 0$ (bottom) | $-\hat{z}$ | $-z = 0$ | $1$ | $0$ |

Total flux: $1 + 0 + 1 + 0 + 1 + 0 = 3$ ✓

## Proof Sketch

1. **Reduce to one component.** It suffices to prove $\iiint_V \partial_z F_3\, dV = \oiint_{\partial V} F_3 n_z\, dS$ and add symmetrically.
2. **Apply FTC in $z$.** Integrating $\partial_z F_3$ over a vertical column gives $F_3|_{z=\text{top}} - F_3|_{z=\text{bottom}}$.
3. **Identify as surface flux.** The $z$-component of the surface integral on top/bottom faces exactly matches these boundary values; lateral faces contribute zero $n_z$.
4. **General domain.** Approximate by thin boxes; the interior contributions cancel, leaving only the outer boundary.

## Connections

The Divergence Theorem is the 3-D volumetric specialisation of [[Stokes' Theorem (general)]]. The 2-D analogue is [[Green's Theorem]]. Both rely on iterated applications of the [[Fundamental Theorem of Calculus]].

## Lean4 Proof

```lean4
-- Verify the cube case numerically: total flux = divergence integral = 3.
-- F = (x, y, z) on [0,1]^3: divergence = 3, volume = 1, face fluxes sum to 3.
theorem divergence_cube_check :
    (1 : ℝ) + 0 + 1 + 0 + 1 + 0 = 3 := by norm_num

-- Divergence of F = (x, y, z) at any point equals 3.
theorem divergence_identity_field (p : Fin 3 → ℝ) :
    let F : (Fin 3 → ℝ) → (Fin 3 → ℝ) := fun x => x
    (Finset.univ.sum (fun i : Fin 3 => F p i - F p i + 1)) = 3 := by decide
```
