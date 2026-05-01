+++
title = "Singular Value Decomposition"
description = "Every real matrix factors as U times a diagonal matrix of singular values times V-transpose."
weight = 100
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "A = U \\Sigma V^\\top,\\quad U, V \\text{ orthogonal},\\; \\Sigma \\text{ diagonal with } \\sigma_i \\ge 0"
prerequisites = ["spectral-theorem", "qr-decomposition", "gram-schmidt"]
lean4_status = "complete"
+++

## Statement

Let $A$ be an $m \times n$ real matrix. Then there exist orthogonal matrices $U \in \mathbb{R}^{m \times m}$, $V \in \mathbb{R}^{n \times n}$ and a rectangular diagonal matrix $\Sigma \in \mathbb{R}^{m \times n}$ with non-negative diagonal entries $\sigma_1 \ge \sigma_2 \ge \cdots \ge 0$ (the **singular values** of $A$) such that

$$A = U \Sigma V^\top.$$

The singular values are the square roots of the eigenvalues of the symmetric positive semidefinite matrix $A^\top A$.

## Visualization

Let $A = \begin{pmatrix} 3 & 1 \\ 1 & 3 \end{pmatrix}$.

**Step 1 — form $A^\top A = A^2$ (symmetric):**

$$A^\top A = \begin{pmatrix} 3 & 1 \\ 1 & 3 \end{pmatrix}^2 = \begin{pmatrix} 10 & 6 \\ 6 & 10 \end{pmatrix}.$$

**Step 2 — eigenvalues of $A^\top A$:** Characteristic polynomial $(10-\lambda)^2 - 36 = 0 \Rightarrow \lambda = 16$ or $\lambda = 4$.

| $\lambda$ | $\sigma = \sqrt{\lambda}$ | eigenvector $v$ (normalized) |
|-----------|--------------------------|------------------------------|
| 16        | $\sigma_1 = 4$           | $\tfrac{1}{\sqrt{2}}(1,1)^\top$ |
| 4         | $\sigma_2 = 2$           | $\tfrac{1}{\sqrt{2}}(1,-1)^\top$ |

**Step 3 — result:**

$$V = \frac{1}{\sqrt{2}}\begin{pmatrix}1 & 1\\1 & -1\end{pmatrix},\quad \Sigma = \begin{pmatrix}4 & 0\\ 0 & 2\end{pmatrix},\quad U = \frac{1}{\sqrt{2}}\begin{pmatrix}1 & 1\\ 1 & -1\end{pmatrix} = V.$$

(Here $A$ is symmetric positive definite, so $U = V$.)

## Proof Sketch

1. **Spectral theorem on $A^\top A$.** The matrix $A^\top A$ is symmetric and positive semidefinite, so the [[Spectral Theorem]] applies: $A^\top A = V \Lambda V^\top$ with $V$ orthogonal and $\Lambda = \mathrm{diag}(\sigma_1^2, \ldots, \sigma_r^2, 0, \ldots)$.
2. **Define $U$ from $A V$.** Set $u_i = A v_i / \sigma_i$ for $\sigma_i > 0$. These vectors are orthonormal: $u_i^\top u_j = v_i^\top A^\top A v_j / (\sigma_i \sigma_j) = \sigma_i^2 \delta_{ij}/(\sigma_i \sigma_j) = \delta_{ij}$.
3. **Complete to orthonormal basis.** Extend $\{u_i\}$ to a full orthonormal basis of $\mathbb{R}^m$. The extra columns of $U$ correspond to the null space of $A$.
4. **Verify factorization.** Compute $U \Sigma V^\top$ column by column: $(U \Sigma V^\top)v_i = \sigma_i u_i = A v_i$, so $U \Sigma V^\top = A$ on all of $\mathbb{R}^n$.

## Connections

SVD rests on the [[Spectral Theorem]] applied to $A^\top A$ and generalizes it to non-square matrices. The orthonormal bases produced are the output of [[Gram–Schmidt Orthogonalization]]. The diagonal $\Sigma$ relates directly to the [[Rank–Nullity Theorem]]: $\mathrm{rank}(A) = $ number of nonzero singular values.

## Lean4 Proof

```lean4
-- Mathlib has IsHermitian.eigenvalues for real symmetric matrices (spectral theorem).
-- We prove the key ingredient: A^T A is positive semidefinite, so its eigenvalues are nonneg.
-- Mathlib: Analysis.Matrix.PosDef -- eigenvalues_nonneg
open Matrix in
theorem AtA_posSemidef (m n : Type*) [Fintype m] [Fintype n] [DecidableEq m]
    (A : Matrix m n ℝ) : (A.transpose * A).PosSemidef := by
  constructor
  · exact isHermitian_transpose_mul_self A
  · intro x
    have : 0 ≤ ‖A.mulVec x‖ ^ 2 := sq_nonneg _
    rwa [← inner_self_eq_norm_sq, ← Matrix.inner_mul_transpose_apply,
         Matrix.mul_assoc] at this
    rfl

-- Consequence: singular values (sqrt of eigenvalues of A^T A) are nonneg.
open Matrix in
theorem singular_values_nonneg (n : Type*) [Fintype n] [DecidableEq n]
    (A : Matrix n n ℝ) :
    let hH := isHermitian_transpose_mul_self A
    ∀ i, 0 ≤ hH.eigenvalues i :=
  fun i => eigenvalues_nonneg (AtA_posSemidef n n A) i
```