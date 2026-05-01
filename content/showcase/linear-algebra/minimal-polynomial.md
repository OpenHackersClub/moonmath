+++
title = "Minimal Polynomial"
description = "The minimal polynomial of a matrix is the monic polynomial of least degree that annihilates it."
weight = 190
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "\\mu_A \\mid p \\iff p(A) = 0"
prerequisites = ["cayley-hamilton"]
lean4_status = "complete"
+++

## Statement

Let $K$ be a field and $A$ a square matrix over $K$. The **minimal polynomial** $\mu_A \in K[x]$ is the unique monic polynomial of smallest degree such that $\mu_A(A) = 0$. It satisfies the **divisibility characterisation**:

$$p(A) = 0 \implies \mu_A \mid p$$

for every polynomial $p \in K[x]$. Equivalently, $\mu_A$ is the generator of the ideal $\{p \in K[x] : p(A) = 0\}$.

Key relationships:

- $\mu_A \mid \chi_A$ (the characteristic polynomial) — from the [[Cayley–Hamilton Theorem]]
- $\chi_A \mid \mu_A^n$ where $n = \deg \chi_A$
- $\mu_A$ and $\chi_A$ have the same irreducible factors (same roots, possibly different multiplicities)

## Visualization

Compare $A = \begin{pmatrix} 2 & 0 \\ 0 & 2 \end{pmatrix}$ and $J = \begin{pmatrix} 2 & 1 \\ 0 & 2 \end{pmatrix}$.

For $A$ (scalar matrix):
$$A - 2I = 0 \implies \mu_A = x - 2.$$

For $J$ (Jordan block):
$$(J - 2I)^1 = \begin{pmatrix} 0 & 1 \\ 0 & 0 \end{pmatrix} \ne 0,\quad (J - 2I)^2 = 0 \implies \mu_J = (x-2)^2.$$

| Matrix | Char poly $\chi$ | Min poly $\mu$ | $\mu \mid \chi$? |
|--------|-----------------|----------------|------------------|
| $A = 2I$ | $(x-2)^2$ | $x - 2$ | Yes: $(x-2)^2 / (x-2)$ |
| $J = J_2(2)$ | $(x-2)^2$ | $(x-2)^2$ | Yes: trivially |
| $\text{diag}(1,2,3)$ | $(x-1)(x-2)(x-3)$ | $(x-1)(x-2)(x-3)$ | Yes: equal |

The minimal polynomial detects the **Jordan block structure**: $\mu_A$ has $(x - \lambda)^k$ iff the largest Jordan block for $\lambda$ has size $k$.

## Proof Sketch

1. **Ideal structure.** The set $I = \{p \in K[x] : p(A) = 0\}$ is an ideal in $K[x]$. Since $K[x]$ is a PID, $I = (\mu_A)$ for some monic generator $\mu_A$.
2. **Cayley–Hamilton.** The characteristic polynomial $\chi_A \in I$, so $\mu_A \mid \chi_A$.
3. **Divisibility criterion.** If $p(A) = 0$ then $p \in I$, so $\mu_A \mid p$.
4. **Same roots.** If $\lambda$ is an eigenvalue, then $(A - \lambda I)v = 0$ for some $v \ne 0$; evaluating $\mu_A(A)v = 0$ shows $\mu_A(\lambda) = 0$.

## Connections

- [[Cayley–Hamilton Theorem]] — guarantees $\mu_A \mid \chi_A$; together, $\mu_A$ and $\chi_A$ share the same irreducible factors
- [[Jordan Canonical Form]] — the degree of $(x - \lambda)$ in $\mu_A$ equals the size of the largest Jordan block for eigenvalue $\lambda$
- [[Diagonalizability Criterion]] — a matrix is diagonalizable if and only if $\mu_A$ is squarefree (no repeated irreducible factor)

## Lean4 Proof

```lean4
import Mathlib.FieldTheory.Minpoly.Field
import Mathlib.LinearAlgebra.Matrix.Charpoly.Minpoly

/-- The minimal polynomial divides any annihilating polynomial.
    Over a field, `minpoly.dvd` is the direct Mathlib alias. -/
theorem minpoly_dvd_of_aeval_zero
    {K : Type*} [Field K]
    {n : Type*} [Fintype n] [DecidableEq n]
    (M : Matrix n n K) {p : K[X]}
    (hp : Polynomial.aeval M p = 0) :
    minpoly K M ∣ p :=
  minpoly.dvd K M hp

/-- The minimal polynomial divides the characteristic polynomial.
    Mathlib's `Matrix.minpoly_dvd_charpoly` is the direct alias. -/
theorem minpoly_dvd_charpoly_matrix
    {K : Type*} [Field K]
    {n : Type*} [Fintype n] [DecidableEq n]
    (M : Matrix n n K) :
    minpoly K M ∣ M.charpoly :=
  Matrix.minpoly_dvd_charpoly M
```
