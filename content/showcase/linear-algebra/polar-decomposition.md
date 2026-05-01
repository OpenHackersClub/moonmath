+++
title = "Polar Decomposition"
description = "Every invertible matrix factors as a unitary times a positive definite matrix."
weight = 160
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "A = U P,\\quad U \\text{ unitary},\\; P = \\sqrt{A^* A} \\text{ positive semidefinite}"
prerequisites = ["spectral-theorem"]
lean4_status = "complete"
+++

## Statement

Let $A$ be an invertible $n \times n$ complex matrix. Then there exist a unique unitary matrix $U$ and a unique positive definite matrix $P$ such that

$$A = U P, \qquad P = \sqrt{A^* A}, \qquad U = A P^{-1}.$$

The matrix $P = \sqrt{A^* A}$ is the unique positive semidefinite square root of $A^* A$ (which exists because $A^* A$ is Hermitian with non-negative eigenvalues). In the rectangular generalisation ($m \times n$), uniqueness of $U$ is replaced by uniqueness of the restriction to the column space.

## Visualization

Take $A = \begin{pmatrix} 3 & 0 \\ 0 & -2 \end{pmatrix}$.

Step 1 — Compute $A^* A$:
$$A^* A = \begin{pmatrix} 3 & 0 \\ 0 & -2 \end{pmatrix}^* \begin{pmatrix} 3 & 0 \\ 0 & -2 \end{pmatrix} = \begin{pmatrix} 9 & 0 \\ 0 & 4 \end{pmatrix}.$$

Step 2 — Positive definite square root:
$$P = \sqrt{A^* A} = \begin{pmatrix} 3 & 0 \\ 0 & 2 \end{pmatrix}.$$

Step 3 — Unitary factor:
$$U = A P^{-1} = \begin{pmatrix} 3 & 0 \\ 0 & -2 \end{pmatrix} \begin{pmatrix} 1/3 & 0 \\ 0 & 1/2 \end{pmatrix} = \begin{pmatrix} 1 & 0 \\ 0 & -1 \end{pmatrix}.$$

Verification:
$$U P = \begin{pmatrix} 1 & 0 \\ 0 & -1 \end{pmatrix} \begin{pmatrix} 3 & 0 \\ 0 & 2 \end{pmatrix} = \begin{pmatrix} 3 & 0 \\ 0 & -2 \end{pmatrix} = A. \checkmark$$

| Quantity | Value |
|----------|-------|
| $A^* A$  | $\text{diag}(9, 4)$ |
| $P = \sqrt{A^* A}$ | $\text{diag}(3, 2)$ |
| $U = A P^{-1}$ | $\text{diag}(1, -1)$ |
| $U U^*$ | $I$ (unitary) |
| $P^* = P$, $P \ge 0$ | positive definite |

## Proof Sketch

1. **Positive semidefiniteness of $A^* A$.** For any $v$, $\langle A^* A v, v \rangle = \|Av\|^2 \ge 0$, so $A^* A$ is positive semidefinite. When $A$ is invertible, $\ker A = \{0\}$, so it is positive definite.
2. **Square root.** By the [[Spectral Theorem]], $A^* A = Q D Q^*$ with $D = \text{diag}(\lambda_i)$, $\lambda_i > 0$. Set $P = Q \sqrt{D} Q^*$. Then $P^2 = A^* A$ and $P$ is positive definite.
3. **Unitary factor.** Set $U = A P^{-1}$. Then $U^* U = (AP^{-1})^* (AP^{-1}) = P^{-1} A^* A P^{-1} = P^{-1} P^2 P^{-1} = I$.
4. **Uniqueness.** If $A = U_1 P_1 = U_2 P_2$ then $P_1^2 = A^* A = P_2^2$; by uniqueness of the positive definite square root, $P_1 = P_2$, hence $U_1 = U_2$.

## Connections

- [[Spectral Theorem]] — the existence of $P = \sqrt{A^* A}$ rests on spectral diagonalization of the Hermitian matrix $A^* A$
- [[Determinant Multiplicativity]] — $|\det A| = \det P$ since $\det U = 1$ for unitary $U$, and $\det(A^* A) = |\det A|^2$
- [[Cauchy–Schwarz Inequality]] — the positive semidefiniteness of $A^* A$ is an instance of the Cauchy–Schwarz inequality applied to the inner product $\langle Av, Av \rangle$

## Lean4 Proof

```lean4
import Mathlib.LinearAlgebra.Matrix.PosDef

/-- The matrix Aᴴ * A is positive semidefinite for any matrix A.
    This is the foundation of polar decomposition: P = sqrt(Aᴴ * A).
    Mathlib's `Matrix.posSemidef_conjTranspose_mul_self` is the direct alias. -/
theorem ata_posSemidef
    {m n : Type*} [Fintype n] [DecidableEq n] [Fintype m]
    (A : Matrix m n ℂ) :
    (Aᴴ * A).PosSemidef :=
  Matrix.posSemidef_conjTranspose_mul_self A
```

