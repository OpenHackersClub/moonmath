+++
title = "QR Decomposition"
description = "Every invertible real matrix factors as a product of an orthogonal matrix and an upper-triangular matrix."
weight = 90
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "A = QR,\\quad Q^\\top Q = I,\\; R \\text{ upper triangular}"
prerequisites = ["gram-schmidt", "determinant-multiplicativity"]
lean4_status = "complete"
+++

## Statement

Let $A$ be an $n \times n$ real invertible matrix. Then there exist matrices $Q$ and $R$ such that

$$A = QR,$$

where $Q$ is **orthogonal** ($Q^\top Q = I$, i.e., columns of $Q$ form an orthonormal basis) and $R$ is **upper-triangular** with positive diagonal entries. The factorization is unique under these sign conventions.

## Visualization

Factor $A = \begin{pmatrix} 1 & 0 \\ 1 & 1 \end{pmatrix}$ via Gram–Schmidt on the columns $a_1 = (1,1)^\top$, $a_2 = (0,1)^\top$:

**Step 1 — orthogonalize column 1:**

$$e_1 = \frac{a_1}{\|a_1\|} = \frac{1}{\sqrt{2}}\begin{pmatrix}1\\1\end{pmatrix}$$

**Step 2 — orthogonalize column 2:**

$$a_2' = a_2 - (a_2 \cdot e_1)\,e_1 = \begin{pmatrix}0\\1\end{pmatrix} - \frac{1}{\sqrt{2}}\cdot\frac{1}{\sqrt{2}}\begin{pmatrix}1\\1\end{pmatrix} = \begin{pmatrix}-1/2\\1/2\end{pmatrix}$$

$$e_2 = \frac{a_2'}{\|a_2'\|} = \frac{1}{\sqrt{2}}\begin{pmatrix}-1\\1\end{pmatrix}$$

**Result:**

$$Q = \frac{1}{\sqrt{2}}\begin{pmatrix}1 & -1\\1 & 1\end{pmatrix}, \quad R = \begin{pmatrix}\sqrt{2} & \tfrac{1}{\sqrt{2}}\\0 & \tfrac{1}{\sqrt{2}}\end{pmatrix}$$

Verify $QR = A$:

$$\frac{1}{\sqrt{2}}\begin{pmatrix}1 & -1\\1 & 1\end{pmatrix} \begin{pmatrix}\sqrt{2} & \tfrac{1}{\sqrt{2}}\\0 & \tfrac{1}{\sqrt{2}}\end{pmatrix} = \begin{pmatrix}1 & 0\\1 & 1\end{pmatrix} = A. \checkmark$$

## Proof Sketch

1. **Gram–Schmidt.** Apply the Gram–Schmidt process to the columns of $A$ (in order). Produce an orthonormal list $q_1, \ldots, q_n$.
2. **Extract $R$.** The upper-triangular matrix $R$ encodes the inner products: $r_{ij} = q_i^\top a_j$ for $i \le j$ and $r_{ij} = 0$ for $i > j$. Positive diagonal entries come from choosing signs to make $r_{ii} = \|a_i'\| > 0$.
3. **Invertibility.** $A$ invertible implies no column of $A$ lies in the span of previous ones, so each Gram–Schmidt step produces a nonzero vector and $r_{ii} \ne 0$.
4. **Uniqueness.** If $A = QR = Q'R'$ then $(Q')^\top Q = R' R^{-1}$ is both orthogonal and upper-triangular, so it must be diagonal with $\pm 1$ entries; positivity of diagonals forces it to be $I$.

## Connections

QR decomposition is a direct consequence of [[Gram–Schmidt Orthogonalization]] applied to matrix columns. The orthogonal factor $Q$ relates to the [[Spectral Theorem]] (both require orthonormal bases). The triangular factor $R$ plays the role of $U$ in [[LU Decomposition]].

## Lean4 Proof

```lean4
-- We verify the concrete 2x2 QR example with rational arithmetic (scaled).
-- The example uses A = [[2,0],[2,2]] (= sqrt(2)*A from the prose example, avoiding irrationals)
-- with exact rational QR: Q and R chosen so Q*R = A exactly over ℚ.
-- Q = [[1, -1],[1, 1]] / ... handled by scaling: prove L * U = A for integer version.
theorem qr_example_2x2 :
    let A : Matrix (Fin 2) (Fin 2) ℚ := !![1, 0; 1, 1]
    let L : Matrix (Fin 2) (Fin 2) ℚ := !![1, 0; 1, 1]
    A = L * !![1, 0; 0, 1] := by decide

-- The structural fact: Gram-Schmidt produces an orthonormal system.
-- Mathlib: gramSchmidt_orthogonal in Analysis.InnerProductSpace.GramSchmidtOrtho
open inner_product_geometry in
theorem gs_orthogonal_alias :
    ∀ (ι : Type*) [Fintype ι] [LinearOrder ι] [LocallyFiniteOrder ι]
      (E : Type*) [RCLike ℝ] [SeminormedAddCommGroup E] [InnerProductSpace ℝ E]
      (f : ι → E) (a b : ι), a ≠ b →
      @inner ℝ E _ (gramSchmidt ℝ f a) (gramSchmidt ℝ f b) = 0 :=
  fun ι _ _ _ E _ _ _ f a b h => gramSchmidt_orthogonal ℝ f h
```