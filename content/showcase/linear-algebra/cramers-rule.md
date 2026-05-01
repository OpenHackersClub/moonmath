+++
title = "Cramer's Rule"
description = "Explicit determinant formula for the solution of a linear system"
weight = 50
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "x_i = \\frac{\\det(A_i)}{\\det(A)}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $A$ be an invertible $n \times n$ matrix over a commutative ring $R$ (so $\det(A)$ is a unit), and let $b \in R^n$. The unique solution $x$ to $Ax = b$ satisfies:

$$x_i = \frac{\det(A_i(b))}{\det(A)}$$

where $A_i(b)$ is the matrix $A$ with its $i$-th column replaced by $b$.

In the non-invertible / commutative ring setting, the adjugate form of the rule states:

$$A \cdot (\operatorname{cramer}(A, b)) = \det(A) \cdot b$$

where $\operatorname{cramer}(A, b)_i = \det(A_i(b))$, without requiring $\det(A)$ to be a unit.

## Visualization

Solve the $2 \times 2$ system:

$$\begin{cases} 2x + y = 5 \\ x + 3y = 7 \end{cases}$$

which corresponds to $Ax = b$ with:

$$A = \begin{pmatrix} 2 & 1 \\ 1 & 3 \end{pmatrix}, \quad b = \begin{pmatrix} 5 \\ 7 \end{pmatrix}$$

**Compute $\det(A)$:**

$$\det(A) = (2)(3) - (1)(1) = 5$$

Since $\det(A) = 5 \neq 0$, the system has a unique solution.

**Compute $\det(A_1(b))$ — replace column 1 with $b$:**

$$A_1 = \begin{pmatrix} 5 & 1 \\ 7 & 3 \end{pmatrix}, \quad \det(A_1) = (5)(3) - (1)(7) = 8$$

**Compute $\det(A_2(b))$ — replace column 2 with $b$:**

$$A_2 = \begin{pmatrix} 2 & 5 \\ 1 & 7 \end{pmatrix}, \quad \det(A_2) = (2)(7) - (5)(1) = 9$$

**Apply Cramer's rule:**

$$x = \frac{\det(A_1)}{\det(A)} = \frac{8}{5}, \qquad y = \frac{\det(A_2)}{\det(A)} = \frac{9}{5}$$

**Verify:**

$$2\cdot\frac{8}{5} + \frac{9}{5} = \frac{25}{5} = 5 \checkmark, \qquad \frac{8}{5} + 3\cdot\frac{9}{5} = \frac{35}{5} = 7 \checkmark$$

## Proof Sketch

The adjugate satisfies $A \cdot \operatorname{adj}(A) = \det(A) \cdot I$ by definition (cofactor expansion). Setting $\operatorname{cramer}(A, b) = \operatorname{adj}(A) \cdot b$, one computes:

$$A \cdot \operatorname{cramer}(A, b) = A \cdot \operatorname{adj}(A) \cdot b = \det(A) \cdot b$$

The $i$-th component of $\operatorname{adj}(A) \cdot b$ is $\sum_j (-1)^{i+j} \det(A_{ji})\, b_j$, which is precisely the cofactor expansion of $\det(A_i(b))$ along column $i$. When $\det(A)$ is a unit, dividing through gives the classical formula $x_i = \det(A_i(b))/\det(A)$.

## Connections

- [[Rank–Nullity Theorem]] — Cramer's rule applies when $\operatorname{nullity}(A) = 0$; otherwise the system has no unique solution.
- [[Determinant Multiplicativity]] — the adjugate identity $A \cdot \operatorname{adj}(A) = \det(A) I$ is a consequence of cofactor expansion and the multiplicativity of $\det$.
- [[Cayley–Hamilton Theorem]] — the adjugate $\operatorname{adj}(A)$ appears in the Cayley–Hamilton proof; $p_A(A) = 0$ is assembled from the identity $(\lambda I - A)\operatorname{adj}(\lambda I - A) = p_A(\lambda)I$.
- [[Spectral Theorem]] — in an orthogonal basis of eigenvectors, the linear system decouples and each component's solution is $b_i/\lambda_i$, a spectral analogue of Cramer's formula.
- [[Quadratic Formula]] — for $n=1$, Cramer's rule recovers $x = b/a$; the quadratic formula solves the $n=2$ eigenvalue equation.

## Lean4 Proof

```lean4
/-- Cramer's rule in adjugate form: A *ᵥ (cramer A b) = det(A) • b.
    This holds over any commutative ring without requiring det(A) to be
    a unit. Mathlib locates this in LinearAlgebra.Matrix.Adjugate. -/
theorem cramers_rule {n R : Type*}
    [Fintype n] [DecidableEq n] [CommRing R]
    (A : Matrix n n R) (b : n → R) :
    A *ᵥ (Matrix.cramer A b) = A.det • b :=
  Matrix.mulVec_cramer A b
```
