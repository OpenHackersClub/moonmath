+++
title = "Schur Decomposition"
description = "Every square complex matrix is unitarily similar to an upper triangular matrix."
weight = 150
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "A = Q T Q^*,\\quad T \\text{ upper triangular},\\; Q \\text{ unitary}"
prerequisites = ["spectral-theorem"]
lean4_status = "complete"
+++

## Statement

For every $A \in \mathbb{C}^{n \times n}$ there exists a unitary matrix $Q$ (i.e.\ $Q Q^* = I$) and an upper triangular matrix $T$ such that

$$A = Q T Q^*.$$

The diagonal entries of $T$ are precisely the eigenvalues of $A$ (with multiplicity). When $A$ is normal ($A A^* = A^* A$) the triangular form $T$ is actually diagonal, recovering the [[Spectral Theorem]].

## Visualization

Take $A = \begin{pmatrix} 0 & -1 \\ 1 & 0 \end{pmatrix}$, the standard rotation by $90°$. Over $\mathbb{R}$ it has no eigenvalues, but over $\mathbb{C}$ it has eigenvalues $\pm i$.

```
Eigenvalues of A: λ₁ = i,  λ₂ = -i
Eigenvectors (columns of Q):
  q₁ = (1/√2)(1, -i)ᵀ,   q₂ = (1/√2)(1, i)ᵀ

Schur form T = QᴴAQ:
  T = ( i    *  )     (* = off-diagonal entry, value 0 here since A is normal)
      ( 0   -i  )
```

Because $A$ is unitary (hence normal), the Schur form is diagonal:

$$T = \begin{pmatrix} i & 0 \\ 0 & -i \end{pmatrix}, \qquad Q = \frac{1}{\sqrt{2}}\begin{pmatrix} 1 & 1 \\ -i & i \end{pmatrix}.$$

| Step | Value |
|------|-------|
| $A q_1$ | $i \, q_1$ (eigenvector) |
| $A q_2$ | $-i \, q_2$ (eigenvector) |
| $Q^* A Q$ | $\text{diag}(i, -i) = T$ |

## Proof Sketch

1. **Base case.** Pick any eigenvalue $\lambda_1$ and unit eigenvector $q_1$. Extend to an orthonormal basis $\{q_1, \ldots, q_n\}$ of $\mathbb{C}^n$.
2. **Block reduction.** Relative to this basis, $A$ has the block form $\begin{pmatrix} \lambda_1 & * \\ 0 & A' \end{pmatrix}$ where $A'$ is $(n-1)\times(n-1)$.
3. **Induction.** Apply the construction to $A'$. By induction, $A'$ is unitarily similar to an upper triangular matrix $T'$, which assembles into $T = \begin{pmatrix} \lambda_1 & * \\ 0 & T' \end{pmatrix}$.
4. **Diagonal for normal matrices.** If $A$ is normal, $\|T e_j\|^2 = \|T^* e_j\|^2$ for every standard basis vector forces all off-diagonal entries to zero, giving a diagonal $T$.

## Connections

- [[Spectral Theorem]] — Schur decomposition restricted to normal matrices is exactly the spectral theorem; the diagonal form records eigenvalues
- [[Cayley–Hamilton Theorem]] — the characteristic polynomial of $A$ equals that of its Schur form $T$ (since $\det(A - \lambda I) = \det(T - \lambda I)$); Cayley–Hamilton then applies equally to $T$
- [[Jordan Canonical Form]] — Schur triangularises without ordering eigenspaces; Jordan goes further, revealing the nilpotent structure inside each eigenspace

## Lean4 Proof

```lean4
import Mathlib.Analysis.Matrix.Spectrum

/-- Schur decomposition for Hermitian matrices (the normal case):
    the star-algebra automorphism by the conjugate of the eigenvector
    unitary diagonalizes A.  This is `conjStarAlgAut_star_eigenvectorUnitary`
    in Mathlib — a direct one-line alias. -/
theorem hermitian_schur
    {𝕜 : Type*} [RCLike 𝕜]
    {n : Type*} [Fintype n] [DecidableEq n]
    {A : Matrix n n 𝕜} (hA : A.IsHermitian) :
    Unitary.conjStarAlgAut 𝕜 _ (star hA.eigenvectorUnitary) A =
      Matrix.diagonal (RCLike.ofReal ∘ hA.eigenvalues) :=
  hA.conjStarAlgAut_star_eigenvectorUnitary
```


