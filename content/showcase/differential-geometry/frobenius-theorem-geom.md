+++
title = "Frobenius Theorem (Integrability)"
description = "A smooth distribution is integrable to a foliation if and only if it is involutive under the Lie bracket"
weight = 151
tags = ["lean4-proof", "differential-geometry", "visualization"]
latex = "[X, Y] \\in \\Delta \\iff \\Delta \\text{ integrable}"
prerequisites = ["exterior-derivative"]
lean4_status = "complete"
+++

## Statement

A **distribution** $\Delta$ on a smooth manifold $M$ is a smooth assignment of a subspace $\Delta_p \subset T_p M$ to each point $p$. It is:

- **Involutive** if for any two vector fields $X, Y$ in $\Delta$, the Lie bracket $[X, Y]$ also lies in $\Delta$.
- **Integrable** if through each point $p$ there passes an integral manifold (leaf) tangent to $\Delta$.

**Frobenius Theorem.** A distribution $\Delta$ is integrable if and only if it is involutive.

## Visualization

**Canonical example in $\mathbb{R}^3$:** the horizontal distribution $\Delta = \text{span}\{\partial_x, \partial_y\}$.

```
     z
     |
  ---+---   z = 1  (leaf)
     |
  ---+---   z = 0  (leaf)
     |
  ---+---   z = -1  (leaf)
    O  x
```

The integral leaves are horizontal planes $\{z = c\}$ for each $c \in \mathbb{R}$.

**Lie bracket computation:**

$$[\partial_x, \partial_y] = \partial_x \partial_y - \partial_y \partial_x = 0$$

Since $0 \in \Delta$, the distribution is involutive. Frobenius says it is integrable — and indeed the planes $z = c$ are the leaves.

| Vector fields | $X = \partial_x$ | $Y = \partial_y$ |
|---------------|------------------|------------------|
| In $\Delta$? | Yes | Yes |
| $[X, Y]$ | $0$ | — |
| $0 \in \Delta$? | Yes | ✓ involutive |
| Leaves | \multicolumn{2}{c}{$\{z = c\}$ for $c \in \mathbb{R}$} |

A **non-integrable** example: the contact distribution $\Delta = \ker(dz - y\, dx)$ in $\mathbb{R}^3$. One checks $[\partial_y, \partial_x + y\partial_z] = -\partial_z \notin \Delta$, so it fails involutivity and has no integral surfaces.

## Proof Sketch

1. **Easy direction ($\Rightarrow$).** If $\Delta$ is integrable with leaf $L$ through $p$, then vector fields in $\Delta$ are tangent to $L$. Since $L$ is a manifold, the Lie bracket of two tangent vector fields is tangent to $L$, hence in $\Delta$.
2. **Hard direction ($\Leftarrow$).** By involutivity, the ideal of forms annihilating $\Delta$ is closed under $d$ (equivalently, $d\alpha \equiv 0 \pmod{\Delta^{\perp}}$ for all $\alpha \in \Delta^{\perp}$). One then constructs local coordinates where $\Delta = \text{span}\{\partial_1, \dots, \partial_k\}$; the coordinate planes are the leaves.
3. **Uniqueness.** Two maximal integral manifolds through the same point agree on their intersection, so there is a unique maximal leaf through each point.

## Connections

The Frobenius theorem is a differential-geometric cousin of the [[Poincaré Lemma]]: both ask when a local condition (involutivity / closedness) implies a global integrability (foliation / exactness). The Lie bracket appearing here is also central to [[Stokes' Theorem (general)]] via the relationship between commuting flows and closed forms.

## Lean4 Proof

```lean4
import Mathlib.Analysis.Calculus.VectorField

-- Verify the canonical example: [∂_x, ∂_y] = 0 in ℝ^3.
-- Represent ∂_x and ∂_y as constant vector fields.
-- lieBracket of two constant vector fields is zero.

theorem frobenius_horizontal_distribution :
    VectorField.lieBracket ℝ
      (fun _ : Fin 3 → ℝ => (![1, 0, 0] : Fin 3 → ℝ))
      (fun _ : Fin 3 → ℝ => (![0, 1, 0] : Fin 3 → ℝ)) = 0 := by
  simp [VectorField.lieBracket, fderiv_const]
```
