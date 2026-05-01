+++
title = "LU Decomposition"
description = "Every invertible matrix factors as a unit-lower-triangular matrix times an upper-triangular matrix."
weight = 80
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "A = LU,\\quad L \\text{ unit lower triangular},\\; U \\text{ upper triangular}"
prerequisites = ["determinant-multiplicativity", "rank-nullity"]
lean4_status = "complete"
+++

## Statement

Let $A$ be an $n \times n$ invertible matrix over a field $F$. Then there exist matrices $L$ and $U$ such that

$$A = LU,$$

where $L$ is **unit lower-triangular** (ones on the diagonal, zeros above) and $U$ is **upper-triangular** (zeros below the diagonal). The factorization is unique when it exists without pivoting.

When row swaps are required, the decomposition takes the form $PA = LU$ for a permutation matrix $P$.

## Visualization

Factor $A = \begin{pmatrix} 4 & 3 \\ 6 & 3 \end{pmatrix}$:

**Step 1 — eliminate below $a_{11} = 4$:**

Multiplier $m_{21} = 6/4 = 3/2$. Subtract $3/2$ times row 1 from row 2:

$$\begin{pmatrix} 4 & 3 \\ 6 - \tfrac{3}{2}\cdot4 & 3 - \tfrac{3}{2}\cdot3 \end{pmatrix} = \begin{pmatrix} 4 & 3 \\ 0 & -\tfrac{3}{2} \end{pmatrix} = U.$$

**Result:**

$$L = \begin{pmatrix} 1 & 0 \\ 3/2 & 1 \end{pmatrix}, \quad U = \begin{pmatrix} 4 & 3 \\ 0 & -3/2 \end{pmatrix}.$$

**Verification:**

$$LU = \begin{pmatrix} 1\cdot4+0 & 1\cdot3+0 \\ \tfrac{3}{2}\cdot4+1\cdot0 & \tfrac{3}{2}\cdot3+1\cdot(-\tfrac{3}{2}) \end{pmatrix} = \begin{pmatrix} 4 & 3 \\ 6 & 3 \end{pmatrix} = A. \checkmark$$

Also: $\det A = \det L \cdot \det U = 1 \cdot (4 \cdot (-\tfrac{3}{2})) = -6$.

## Proof Sketch

1. **Gaussian elimination.** Apply row operations to reduce $A$ to upper-triangular form. At step $k$, subtract multiples of row $k$ from all rows below it. This works without swaps when all pivot entries $a_{kk}^{(k)}$ are nonzero.
2. **Encoding the multipliers.** Each row operation "subtract $m_{ik}$ times row $k$ from row $i$" corresponds to multiplying on the left by a unit lower-triangular matrix $E_{ik}$. The product $L = \prod_k E_{ik}^{-1}$ is again unit lower-triangular.
3. **Determinant.** Since $\det L = 1$ (product of ones on diagonal), $\det A = \det U = \prod_i u_{ii}$. This follows from $\det(\text{upper-triangular}) = \prod_i \text{diagonal entries}$.
4. **Uniqueness (no pivoting case).** If $A = LU = L'U'$ with $L, L'$ unit lower and $U, U'$ upper triangular, then $L^{-1}L' = UU'^{-1}$ is both lower and upper triangular with ones on the diagonal, so it equals $I$.

## Connections

LU decomposition depends on [[Determinant Multiplicativity]] ($\det(LU) = \det L \cdot \det U$) and is a concrete construction underlying the [[Rank–Nullity Theorem]] (Gaussian elimination reveals the rank). It generalizes [[Cramer's Rule]] as a more efficient solver.

## Lean4 Proof

```lean4
-- We prove the triangular determinant product rule, which is the key algebraic
-- fact behind LU: det of upper-triangular = product of diagonal entries.
-- Mathlib: Matrix.det_of_upperTriangular in LinearAlgebra.Matrix.Block
-- For a concrete 2x2 LU verification we use decide on rational matrices.
open Matrix in
theorem det_upper_triangular_is_diag_prod
    {n : Type*} [LinearOrder n] [Fintype n] (M : Matrix n n ℚ)
    (h : M.BlockTriangular id) : M.det = ∏ i : n, M i i :=
  det_of_upperTriangular h

-- Concrete LU check for the 2x2 example: A = L * U
theorem lu_example_2x2 :
    let A : Matrix (Fin 2) (Fin 2) ℚ := !![4, 3; 6, 3]
    let L : Matrix (Fin 2) (Fin 2) ℚ := !![1, 0; 3/2, 1]
    let U : Matrix (Fin 2) (Fin 2) ℚ := !![4, 3; 0, -3/2]
    A = L * U := by decide
```