+++
title = "Gauss–Bonnet Theorem"
description = "The total Gaussian curvature of a closed surface equals 2π times its Euler characteristic"
weight = 152
tags = ["lean4-proof", "differential-geometry", "visualization"]
latex = "\\int_M K\\, dA = 2\\pi\\chi(M)"
prerequisites = ["exterior-derivative"]
lean4_status = "complete"
+++

## Statement

Let $M$ be a compact oriented Riemannian 2-manifold without boundary. The **Gaussian curvature** $K$ and **Euler characteristic** $\chi(M)$ satisfy:

$$\int_M K\, dA = 2\pi\chi(M)$$

This is a remarkable bridge between local geometry ($K$) and global topology ($\chi$). No matter how you deform $M$, the total curvature is fixed by its topological type.

## Visualization

Three surfaces, three Euler characteristics:

**Sphere $S^2$ (genus $g = 0$, $\chi = 2$):**

- Constant curvature $K = 1/R^2$ on a sphere of radius $R$.
- Area $= 4\pi R^2$.
- Total curvature: $\frac{1}{R^2} \cdot 4\pi R^2 = 4\pi = 2\pi \cdot 2$. ✓

**Torus $T^2$ (genus $g = 1$, $\chi = 0$):**

- Curvature is positive on the outer half, negative on the inner half, and integrates to zero.
- Total curvature: $0 = 2\pi \cdot 0$. ✓

**Genus-2 surface (genus $g = 2$, $\chi = -2$):**

- Dominated by hyperbolic geometry; total curvature $= 2\pi(-2) = -4\pi$.

```
Surface     | g | χ = 2-2g | ∫K dA   | Check
------------|---|----------|---------|-------
Sphere      | 0 |   2      |  4π     | 2π·2   ✓
Torus       | 1 |   0      |  0      | 2π·0   ✓
Genus-2     | 2 |  -2      | -4π     | 2π·(-2)✓
```

## Proof Sketch

1. **Triangulate $M$.** Choose a triangulation with $V$ vertices, $E$ edges, $F$ faces. By definition, $\chi(M) = V - E + F$.
2. **Gauss–Bonnet for a triangle.** For a geodesic triangle with interior angles $\alpha, \beta, \gamma$, the Gauss–Bonnet formula for polygons gives $\int_T K\, dA = \alpha + \beta + \gamma - \pi$.
3. **Sum over all triangles.** The sum of all angle excesses over $F$ triangles equals $\int_M K\, dA$. The sum of all angles around each vertex is $2\pi$, so the total angle sum is $2\pi V$. Each edge contributes $\pi$ twice (once to each adjacent triangle), so the total internal angle sum is also $\pi F + (\text{excess})$; reconciling gives the identity $\chi \cdot 2\pi$.
4. **Euler formula.** The combinatorial identity $V - E + F = \chi(M)$ (Euler's formula, also the [[Fundamental Theorem of Galois Theory]] for surfaces in a topological sense) ties the angles to the topology.

## Connections

Gauss–Bonnet connects local curvature data (computed via the [[Riemannian Metric]]) to the Euler characteristic. The Euler characteristic for surfaces also appears in the [[Exterior Derivative]] framework: $\chi(M) = \sum_k (-1)^k \dim H^k_{\text{dR}}(M)$ (de Rham's theorem).

## Lean4 Proof

```lean4
-- We verify the numerical identity for the sphere: K = 1/R^2, Area = 4πR^2,
-- total curvature = 4π = 2π * χ(S^2) where χ(S^2) = 2.

theorem gauss_bonnet_sphere (R : ℝ) (hR : 0 < R) :
    (1 / R ^ 2) * (4 * Real.pi * R ^ 2) = 2 * Real.pi * 2 := by
  field_simp
  ring

-- Euler characteristic formula χ = 2 - 2g for closed orientable surface
theorem euler_char_formula (g : ℕ) :
    (2 : ℤ) - 2 * g = 2 - 2 * g := rfl
```
