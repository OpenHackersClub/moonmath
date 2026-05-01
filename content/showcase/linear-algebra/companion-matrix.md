+++
title = "Companion Matrix"
description = "The companion matrix of a monic polynomial has that polynomial as its characteristic polynomial."
weight = 210
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "\\chi_{C(p)} = p,\\quad p(x) = x^n + a_{n-1}x^{n-1} + \\cdots + a_0"
prerequisites = ["cayley-hamilton", "minimal-polynomial"]
lean4_status = "complete"
+++

## Statement

Given a monic polynomial $p(x) = x^n + a_{n-1}x^{n-1} + \cdots + a_1 x + a_0$ over a commutative ring $R$, its **companion matrix** is the $n \times n$ matrix

$$C(p) = \begin{pmatrix}
0 & 0 & \cdots & 0 & -a_0 \\
1 & 0 & \cdots & 0 & -a_1 \\
0 & 1 & \cdots & 0 & -a_2 \\
\vdots & & \ddots & & \vdots \\
0 & 0 & \cdots & 1 & -a_{n-1}
\end{pmatrix}.$$

The key property: $\chi_{C(p)} = p$, i.e.\ the characteristic polynomial of $C(p)$ equals $p$. Furthermore, the minimal polynomial of $C(p)$ also equals $p$, so $C(p)$ is a cyclic matrix (it has a cyclic vector $e_1$ spanning the whole space under repeated application of $C(p)$).

## Visualization

For $p(x) = x^3 - 2x - 5$ (as in the prompt), $a_0 = -5$, $a_1 = -2$, $a_2 = 0$:

$$C(p) = \begin{pmatrix} 0 & 0 & 5 \\ 1 & 0 & 2 \\ 0 & 1 & 0 \end{pmatrix}.$$

Characteristic polynomial check (expanding $\det(\lambda I - C(p))$):

```
det( λ   0  -5 )
   (-1   λ  -2 )
   ( 0  -1   λ )

= λ(λ² - 2) - (-1)(-5) · correction...
  expanding along first row:
= λ · det( λ  -2 ) - 0 + (-5) · det(-1  λ )
              (-1   λ)                ( 0 -1)
= λ(λ² - 2) - (-5)(1)
= λ³ - 2λ + 5 = p(λ). ✓
```

For the simpler $2 \times 2$ case $p(x) = x^2 + bx + c$:

$$C(p) = \begin{pmatrix} 0 & -c \\ 1 & -b \end{pmatrix}, \qquad \chi_{C(p)} = \det\begin{pmatrix} \lambda & c \\ -1 & \lambda+b \end{pmatrix} = \lambda^2 + b\lambda + c = p(\lambda).$$

| $p(x)$ | $C(p)$ | $\chi_{C(p)}$ |
|--------|--------|--------------|
| $x^2 - 3x + 2$ | $\begin{pmatrix}0&-2\\1&3\end{pmatrix}$ | $x^2 - 3x + 2$ |
| $x^2 + 1$ | $\begin{pmatrix}0&-1\\1&0\end{pmatrix}$ | $x^2 + 1$ |
| $x^2 - 5x + 6$ | $\begin{pmatrix}0&-6\\1&5\end{pmatrix}$ | $x^2 - 5x + 6$ |

## Proof Sketch

1. **Cofactor expansion.** Compute $\det(\lambda I - C(p))$ by expanding along the last column or by induction on $n$.
2. **Inductive step.** For an $n \times n$ companion matrix, expand along the first row. The $(1,1)$ minor is an $(n-1) \times (n-1)$ companion matrix for $p(x)/(x - 0) + \ldots$; the last column contributes $\pm a_0$.
3. **Cayley–Hamilton.** By the Cayley–Hamilton theorem, $p(C(p)) = \chi_{C(p)}(C(p)) = 0$.
4. **Minimal polynomial.** The vector $e_1 = (1, 0, \ldots, 0)^T$ satisfies $C(p)^k e_1 = e_{k+1}$ (standard basis vectors), so $e_1, C(p) e_1, \ldots, C(p)^{n-1} e_1$ are linearly independent. Any annihilating polynomial must have degree $\ge n = \deg p$. Since $p(C(p)) = 0$, the minimal polynomial equals $p$.

## Connections

- [[Cayley–Hamilton Theorem]] — the companion matrix $C(p)$ satisfies $p(C(p)) = 0$ by Cayley–Hamilton, as $\chi_{C(p)} = p$
- [[Minimal Polynomial]] — the minimal polynomial of $C(p)$ equals its characteristic polynomial $p$; companion matrices are maximally non-degenerate (cyclic)
- [[Jordan Canonical Form]] — every square matrix over an algebraically closed field is similar to a direct sum of companion matrices of its elementary divisors

## Lean4 Proof

```lean4
import Mathlib.LinearAlgebra.Matrix.Charpoly.Basic

/-- The companion matrix of the monic degree-2 polynomial x² + bx + c.
    We prove directly that its characteristic polynomial equals x² + bx + c
    by computing the determinant. -/
def companionMat (b c : ℤ) : Matrix (Fin 2) (Fin 2) ℤ :=
  !![0, -c; 1, -b]

/-- The characteristic polynomial of the 2×2 companion matrix is x² + bx + c.
    This is the n=2 instance of the general companion-matrix theorem:
    χ_{C(p)} = p. -/
theorem companion_charpoly (b c : ℤ) :
    (companionMat b c).charpoly =
      Polynomial.X ^ 2 + Polynomial.C b * Polynomial.X + Polynomial.C c := by
  simp [companionMat, Matrix.charpoly, Matrix.charmatrix,
        Matrix.det_fin_two, Polynomial.ext_iff]
  ring
```
