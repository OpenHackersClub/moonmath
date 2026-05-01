+++
title = "Sylvester Determinant Theorem"
description = "det(Im + AB) = det(In + BA) for any m×n matrix A and n×m matrix B."
weight = 170
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "\\det(I_m + AB) = \\det(I_n + BA)"
prerequisites = ["determinant-multiplicativity", "rank-nullity"]
lean4_status = "complete"
+++

## Statement

Let $A$ be an $m \times n$ matrix and $B$ an $n \times m$ matrix over a commutative ring $R$. Then

$$\det(I_m + AB) = \det(I_n + BA).$$

This is sometimes called the **Weinstein–Aronszajn identity**. A striking consequence: even though $AB$ is $m \times m$ and $BA$ is $n \times n$ (potentially very different sizes), the determinant of the identity-plus product is the same.

## Visualization

Take $A = \begin{pmatrix} 1 \\ 2 \end{pmatrix}$ (column vector, $2 \times 1$) and $B = \begin{pmatrix} 3 & 4 \end{pmatrix}$ (row vector, $1 \times 2$).

$$AB = \begin{pmatrix} 1 \\ 2 \end{pmatrix}\begin{pmatrix} 3 & 4 \end{pmatrix} = \begin{pmatrix} 3 & 4 \\ 6 & 8 \end{pmatrix} \in \mathbb{R}^{2 \times 2}$$

$$BA = \begin{pmatrix} 3 & 4 \end{pmatrix}\begin{pmatrix} 1 \\ 2 \end{pmatrix} = (11) \in \mathbb{R}^{1 \times 1}$$

Left side ($2 \times 2$):
$$\det(I_2 + AB) = \det\begin{pmatrix} 4 & 4 \\ 6 & 9 \end{pmatrix} = 4 \cdot 9 - 4 \cdot 6 = 36 - 24 = 12.$$

Right side ($1 \times 1$):
$$\det(I_1 + BA) = \det(1 + 11) = \det(12) = 12. \checkmark$$

| Quantity | Value |
|----------|-------|
| $I_2 + AB$ | $\begin{pmatrix} 4 & 4 \\ 6 & 9 \end{pmatrix}$ |
| $\det(I_2 + AB)$ | $12$ |
| $I_1 + BA$ | $(12)$ |
| $\det(I_1 + BA)$ | $12$ |

## Proof Sketch

1. **Block matrix trick.** Form the $(m+n)\times(m+n)$ block matrix
$$M = \begin{pmatrix} I_m & A \\ B & I_n \end{pmatrix}.$$
2. **Two factorisations.** Observe:
$$M = \begin{pmatrix} I_m & 0 \\ B & I_n \end{pmatrix}\begin{pmatrix} I_m & A \\ 0 & I_n + BA \end{pmatrix} = \begin{pmatrix} I_m + AB & A \\ 0 & I_n \end{pmatrix}\begin{pmatrix} I_m & 0 \\ B & I_n \end{pmatrix}.$$
3. **Take determinants.** The lower-triangular and upper-triangular factors have determinant $1$, giving $\det M = \det(I_n + BA)$ on the left and $\det M = \det(I_m + AB)$ on the right.

## Connections

- [[Determinant Multiplicativity]] — the proof reduces to $\det(M) = \det(L_1)\det(U_1) = \det(U_2)\det(L_2)$ where each triangular factor contributes $1$
- [[Rank–Nullity Theorem]] — when $m \ne n$, the identity $\det(I_m + AB) = \det(I_n + BA)$ shows that the non-zero eigenvalues of $AB$ and $BA$ coincide (both sides factor over these eigenvalues)
- [[Cayley–Hamilton Theorem]] — a polynomial identity consequence: $\chi_{AB}(t) / t^{m-n} = \chi_{BA}(t)$ (up to sign) follows from Sylvester's theorem applied to $tI - A$

## Lean4 Proof

```lean4
import Mathlib.LinearAlgebra.Matrix.SchurComplement

/-- Sylvester's determinant theorem (Weinstein–Aronszajn identity):
    det(1 + A*B) = det(1 + B*A).
    Mathlib's direct alias is `Matrix.det_one_add_mul_comm`. -/
theorem sylvester_det
    {R : Type*} [CommRing R]
    {m n : Type*} [Fintype m] [Fintype n] [DecidableEq m] [DecidableEq n]
    (A : Matrix m n R) (B : Matrix n m R) :
    (1 + A * B).det = (1 + B * A).det :=
  Matrix.det_one_add_mul_comm A B
```
