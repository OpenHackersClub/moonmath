+++
title = "Spectral Theorem"
description = "Every real symmetric matrix is orthogonally diagonalizable"
weight = 20
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "A = Q \\Lambda Q^T \\;\\text{(real symmetric case)}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $A$ be a real symmetric $n \times n$ matrix (i.e., $A = A^T$). Then:

1. All eigenvalues of $A$ are real.
2. Eigenvectors corresponding to distinct eigenvalues are orthogonal.
3. There exists an orthogonal matrix $Q$ (so $Q^T = Q^{-1}$) and a diagonal matrix $\Lambda$ such that:

$$A = Q \Lambda Q^T$$

Equivalently, $A$ is orthogonally similar to a diagonal matrix of its eigenvalues. In the complex Hermitian setting ($A = A^*$), the same holds with a unitary matrix $U$ replacing $Q$.

## Visualization

Take the symmetric matrix:

$$A = \begin{pmatrix} 4 & 2 \\ 2 & 1 \end{pmatrix}$$

**Step 1 — Find eigenvalues** via $\det(\lambda I - A) = 0$:

$$(\lambda - 4)(\lambda - 1) - 4 = \lambda^2 - 5\lambda = 0 \implies \lambda_1 = 0,\; \lambda_2 = 5$$

Both eigenvalues are real. $\Lambda = \begin{pmatrix} 0 & 0 \\ 0 & 5 \end{pmatrix}$

**Step 2 — Find eigenvectors.**

For $\lambda_1 = 0$: $(A - 0)\mathbf{v} = 0$ gives $4v_1 + 2v_2 = 0$, so $\mathbf{v}_1 = \frac{1}{\sqrt{5}}\begin{pmatrix} -1 \\ 2 \end{pmatrix}$.

For $\lambda_2 = 5$: $(A - 5I)\mathbf{v} = 0$ gives $-v_1 + 2v_2 = 0$, so $\mathbf{v}_2 = \frac{1}{\sqrt{5}}\begin{pmatrix} 2 \\ 1 \end{pmatrix}$.

Note $\mathbf{v}_1 \cdot \mathbf{v}_2 = \frac{1}{5}(-2 + 2) = 0$: orthogonal, as guaranteed.

**Step 3 — Assemble $Q$ and verify $A = Q\Lambda Q^T$.**

$$Q = \frac{1}{\sqrt{5}}\begin{pmatrix} -1 & 2 \\ 2 & 1 \end{pmatrix}, \quad Q\Lambda Q^T = \frac{1}{5}\begin{pmatrix} -1 & 2 \\ 2 & 1 \end{pmatrix}\begin{pmatrix} 0 & 0 \\ 0 & 5 \end{pmatrix}\begin{pmatrix} -1 & 2 \\ 2 & 1 \end{pmatrix}$$

$$= \frac{1}{5}\begin{pmatrix} 0 & 10 \\ 0 & 5 \end{pmatrix}\begin{pmatrix} -1 & 2 \\ 2 & 1 \end{pmatrix} = \frac{1}{5}\begin{pmatrix} 20 & 10 \\ 10 & 5 \end{pmatrix} = \begin{pmatrix} 4 & 2 \\ 2 & 1 \end{pmatrix} = A \checkmark$$

## Proof Sketch

The key steps for real symmetric matrices are: (1) every symmetric operator on a real finite-dimensional inner product space has at least one real eigenvalue (proved via the characteristic polynomial having a real root, using the fundamental theorem of algebra over $\mathbb{C}$ then checking reality). (2) By induction on dimension — restrict to the orthogonal complement of one eigenvector; the restricted operator is still symmetric. (3) Orthogonality of eigenvectors for distinct eigenvalues follows from $\lambda_1 \langle u, v \rangle = \langle Au, v \rangle = \langle u, Av \rangle = \lambda_2 \langle u, v \rangle$.

## Connections

- [[Cayley–Hamilton Theorem]] — the characteristic polynomial whose roots are these eigenvalues is also annihilated by $A$.
- [[Determinant Multiplicativity]] — $\det(A) = \prod_i \lambda_i$; the product of diagonal entries of $\Lambda$.
- [[Rank–Nullity Theorem]] — the number of zero eigenvalues equals the nullity of $A$.
- [[Fundamental Theorem of Galois Theory]] — the eigenvalue decomposition is the linear algebra analogue of a splitting field decomposition.

## Lean4 Proof

```lean4
/-- The spectral theorem for Hermitian matrices: a Hermitian matrix A is
    unitarily conjugate to a diagonal matrix of its (real) eigenvalues.
    Mathlib packages this as `Matrix.IsHermitian.spectral_theorem` in
    Mathlib.Analysis.Matrix.Spectrum. -/
noncomputable example
    {n 𝕜 : Type*} [RCLike 𝕜] [Fintype n] [DecidableEq n]
    {A : Matrix n n 𝕜} (hA : A.IsHermitian) :
    A = hA.eigenvectorUnitary.val *
        Matrix.diagonal (RCLike.ofReal ∘ hA.eigenvalues) *
        (star hA.eigenvectorUnitary.val) :=
  hA.spectral_theorem
```
