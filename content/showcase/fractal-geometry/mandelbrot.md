+++
title = "Mandelbrot Set"
description = "Interactive visualization of z_{n+1} = z_n^2 + c and its connectedness"
weight = 10
premier = true
tags = ["lean4-proof", "interactive", "visualization", "fractal-geometry"]
latex = "z_{n+1} = z_n^2 + c"
prerequisites = []
lean4_status = "sorry"
+++

## Definition

The Mandelbrot set $\mathcal{M}$ is the set of complex numbers $c$ for which the orbit of $0$ under iteration of $f_c(z) = z^2 + c$ remains bounded:

$$\mathcal{M} = \{ c \in \mathbb{C} : \sup_{n} |f_c^n(0)| < \infty \}$$

Equivalently, $c \in \mathcal{M}$ if and only if $|z_n| \leq 2$ for all $n$, where $z_0 = 0$ and $z_{n+1} = z_n^2 + c$.

## Key Properties

- **Connectedness:** The Mandelbrot set is connected (Douady and Hubbard, 1982). This deep result means it is a single "piece" despite its intricate boundary.
- **Self-similarity:** Miniature copies of the entire set appear at every scale along the boundary — the same self-referential structure formalised by [[Iterated Function Systems]].
- **Boundary complexity:** The boundary of $\mathcal{M}$ has [[Hausdorff Dimension|Hausdorff dimension]] 2 (Shishikura, 1998).

## Escape-Time Algorithm

For each pixel (corresponding to a value of $c$), iterate $z_{n+1} = z_n^2 + c$ starting from $z_0 = 0$. If $|z_n| > 2$ for some $n$, the point escapes and is colored by $n$. Otherwise, color it black (assumed to be in $\mathcal{M}$).

## Connections

The boundary of the Mandelbrot set has [[Hausdorff Dimension]] equal to 2. The filled Julia sets for each $c$ can be viewed as attractors of [[Iterated Function Systems]].

## Lean4 Proof

```lean4
import Mathlib

/-- Iteration of z ↦ z² + c starting from z₀ = 0. -/
def mandelbrotIter (c : ℂ) : Nat → ℂ
  | 0 => 0
  | n + 1 => (mandelbrotIter c n) ^ 2 + c

/-- A point c is in the Mandelbrot set if its orbit stays bounded.
    Uses `‖z‖` (the norm) in place of `Complex.abs` which is not a constant. -/
def InMandelbrotSet (c : ℂ) : Prop :=
  ∀ n : Nat, ‖mandelbrotIter c n‖ ≤ 2

/-- The Mandelbrot set is connected (Douady-Hubbard, 1982). -/
theorem mandelbrot_connected :
    IsConnected {c : ℂ | InMandelbrotSet c} := by
  sorry -- deep result using potential theory and conformal mapping
```
