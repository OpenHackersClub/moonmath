+++
title = "Determinant Multiplicativity"
description = "The determinant of a product equals the product of determinants"
weight = 40
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "\\det(AB) = \\det(A)\\,\\det(B)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For any two $n \times n$ matrices $A$ and $B$ over a commutative ring $R$:

$$\det(AB) = \det(A)\,\det(B)$$

Equivalently, $\det : (M_n(R), \cdot) \to (R, \cdot)$ is a monoid homomorphism — and a group homomorphism when restricted to $GL_n(R)$.

## Visualization

Take the $2 \times 2$ matrices:

$$A = \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}, \quad B = \begin{pmatrix} 5 & 6 \\ 7 & 8 \end{pmatrix}$$

**Direct determinants:**

$$\det(A) = (1)(4) - (2)(3) = 4 - 6 = -2$$

$$\det(B) = (5)(8) - (6)(7) = 40 - 42 = -2$$

$$\det(A)\,\det(B) = (-2)(-2) = 4$$

**Compute $AB$ and its determinant:**

$$AB = \begin{pmatrix} 1\cdot5 + 2\cdot7 & 1\cdot6 + 2\cdot8 \\ 3\cdot5 + 4\cdot7 & 3\cdot6 + 4\cdot8 \end{pmatrix} = \begin{pmatrix} 19 & 22 \\ 43 & 50 \end{pmatrix}$$

$$\det(AB) = (19)(50) - (22)(43) = 950 - 946 = 4 \checkmark$$

Both sides equal $4$, confirming $\det(AB) = \det(A)\,\det(B)$.

## Proof Sketch

The standard proof proceeds in two stages. First, note that the result is immediate when $A$ is an elementary row-operation matrix $E$: one checks $\det(E) = \pm 1$ or $c$, and $\det(EB) = \pm \det(B)$ or $c\,\det(B)$ by the row-linearity of $\det$. Second, any invertible $A$ is a product of elementary matrices, so the result follows by induction. When $A$ is singular, $AB$ is singular too ($\operatorname{rank}(AB) \leq \operatorname{rank}(A) < n$), so both sides are $0$.

An elegant alternative: define $f(A) = \det(AB)/\det(B)$ for fixed invertible $B$ and check that $f$ satisfies the defining axioms of the determinant (alternating multilinearity, normalized on $I$), so $f = \det$.

## Connections

- [[Cayley–Hamilton Theorem]] — the characteristic polynomial $p_A(\lambda) = \det(\lambda I - A)$ uses the determinant; multiplicativity underlies $p_{AB}$ and $p_{BA}$ having the same nonzero eigenvalues.
- [[Rank–Nullity Theorem]] — $\det(A) \neq 0 \iff \operatorname{nullity}(A) = 0 \iff A$ is invertible.
- [[Cramer's Rule]] — requires $\det(A) \neq 0$; the proof inverts $A$ using $\det(A)\,A^{-1} = \operatorname{adj}(A)$.
- [[Spectral Theorem]] — $\det(A) = \prod_i \lambda_i$ follows from the diagonalization $A = Q\Lambda Q^T$ and $\det(Q)\det(Q^T) = 1$.
- [[Quadratic Formula]] — discriminant $b^2 - 4ac$ is $-\det$ of the associated $2 \times 2$ companion matrix.

## Lean4 Proof

```lean4
/-- Determinant multiplicativity: det(A * B) = det(A) * det(B) for square
    matrices over a commutative ring. Mathlib's proof uses the Leibniz
    formula and permutation group structure. -/
theorem det_multiplicativity {n R : Type*}
    [Fintype n] [DecidableEq n] [CommRing R]
    (A B : Matrix n n R) : (A * B).det = A.det * B.det :=
  Matrix.det_mul A B
```
