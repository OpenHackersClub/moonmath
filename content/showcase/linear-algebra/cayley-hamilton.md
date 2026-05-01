+++
title = "Cayley–Hamilton Theorem"
description = "Every square matrix satisfies its own characteristic polynomial"
weight = 10
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "p_A(A) = 0 \\;\\text{where}\\; p_A(\\lambda) = \\det(\\lambda I - A)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $A$ be an $n \times n$ matrix over a commutative ring $R$, and let $p_A(\lambda) = \det(\lambda I - A)$ be its characteristic polynomial. Then:

$$p_A(A) = 0$$

That is, substituting $A$ itself for the scalar variable $\lambda$ in the characteristic polynomial yields the zero matrix.

## Visualization

Take the concrete $2 \times 2$ matrix:

$$A = \begin{pmatrix} 3 & 1 \\ 2 & 4 \end{pmatrix}$$

**Step 1 — Compute the characteristic polynomial.**

$$p_A(\lambda) = \det(\lambda I - A) = \det\begin{pmatrix} \lambda - 3 & -1 \\ -2 & \lambda - 4 \end{pmatrix}$$

$$= (\lambda - 3)(\lambda - 4) - (-1)(-2) = \lambda^2 - 7\lambda + 10$$

**Step 2 — Compute $A^2$.**

$$A^2 = \begin{pmatrix} 3 & 1 \\ 2 & 4 \end{pmatrix}\begin{pmatrix} 3 & 1 \\ 2 & 4 \end{pmatrix} = \begin{pmatrix} 11 & 7 \\ 14 & 18 \end{pmatrix}$$

**Step 3 — Evaluate $p_A(A) = A^2 - 7A + 10I$.**

$$A^2 - 7A + 10I = \begin{pmatrix} 11 & 7 \\ 14 & 18 \end{pmatrix} - 7\begin{pmatrix} 3 & 1 \\ 2 & 4 \end{pmatrix} + 10\begin{pmatrix} 1 & 0 \\ 0 & 1 \end{pmatrix}$$

$$= \begin{pmatrix} 11 - 21 + 10 & 7 - 7 + 0 \\ 14 - 14 + 0 & 18 - 28 + 10 \end{pmatrix} = \begin{pmatrix} 0 & 0 \\ 0 & 0 \end{pmatrix}$$

Verified: $p_A(A) = 0$.

## Proof Sketch

The naive approach — "substitute $A$ for $\lambda$ in $\det(\lambda I - A)$" — is circular: the determinant expands as a scalar polynomial, not a matrix polynomial. The correct proof works over the adjugate matrix $\text{adj}(\lambda I - A)$, whose entries are polynomials in $\lambda$. One shows $(\lambda I - A)\,\text{adj}(\lambda I - A) = p_A(\lambda)\,I$ as a polynomial matrix identity, then compares coefficients of $\lambda^k$ on both sides to assemble the relation $p_A(A) = 0$.

An alternative proof observes that the result holds over any field (by finding a splitting field where $A$ is upper-triangular) and then descends to general commutative rings by a universality argument.

## Connections

- [[Spectral Theorem]] — eigenvalues are roots of $p_A$; Cayley–Hamilton shows $A$ itself is a "root."
- [[Determinant Multiplicativity]] — the characteristic polynomial is built from the determinant.
- [[Rank–Nullity Theorem]] — the minimal polynomial divides $p_A$; its degree relates to the kernel.
- [[Fundamental Theorem of Galois Theory]] — Galois extensions are controlled by characteristic polynomials of Frobenius elements.
- [[Quadratic Formula]] — in dimension 2, $p_A$ is quadratic; Cayley–Hamilton is the matrix analogue of a root equation.

## Lean4 Proof

```lean4
/-- The Cayley–Hamilton theorem: every square matrix satisfies its own
    characteristic polynomial. The Mathlib proof works over any commutative
    ring via the adjugate identity. -/
theorem cayley_hamilton {n : Type*} {R : Type*}
    [Fintype n] [DecidableEq n] [CommRing R]
    (A : Matrix n n R) : Polynomial.aeval A A.charpoly = 0 :=
  Matrix.aeval_self_charpoly A
```
