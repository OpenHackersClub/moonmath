+++
title = "Rank–Nullity Theorem"
description = "The dimensions of image and kernel of a linear map sum to the domain dimension"
weight = 30
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "\\operatorname{rank}(f) + \\operatorname{nullity}(f) = \\dim V"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $f : V \to W$ be a linear map between finite-dimensional vector spaces over a field $K$. Then:

$$\dim(\operatorname{im} f) + \dim(\ker f) = \dim V$$

Equivalently, $\operatorname{rank}(f) + \operatorname{nullity}(f) = \dim V$.

## Visualization

Consider the linear map $f : \mathbb{R}^3 \to \mathbb{R}^3$ given by the matrix:

$$A = \begin{pmatrix} 1 & 2 & 3 \\ 2 & 4 & 6 \\ 0 & 1 & 1 \end{pmatrix}$$

**Step 1 — Row reduce to find the image (column space).**

$$\begin{pmatrix} 1 & 2 & 3 \\ 2 & 4 & 6 \\ 0 & 1 & 1 \end{pmatrix} \xrightarrow{R_2 \leftarrow R_2 - 2R_1} \begin{pmatrix} 1 & 2 & 3 \\ 0 & 0 & 0 \\ 0 & 1 & 1 \end{pmatrix} \xrightarrow{R_2 \leftrightarrow R_3} \begin{pmatrix} 1 & 2 & 3 \\ 0 & 1 & 1 \\ 0 & 0 & 0 \end{pmatrix}$$

Two pivot columns $\Rightarrow$ $\operatorname{rank}(A) = 2$.

Image basis: columns 1 and 2 of $A$, i.e., $\{(1,2,0)^T,\; (2,4,1)^T\}$.

**Step 2 — Find the null space.**

Free variable: $x_3 = t$. Back-substituting: $x_2 = -t$, $x_1 = -2x_2 - 3x_3 = 2t - 3t = -t$.

$$\ker A = \operatorname{span}\left\{\begin{pmatrix} -1 \\ -1 \\ 1 \end{pmatrix}\right\}, \quad \operatorname{nullity}(A) = 1$$

**Dimension table:**

| | Dimension |
|---|---|
| $\dim(\operatorname{im} A)$ | 2 |
| $\dim(\ker A)$ | 1 |
| $\dim(\mathbb{R}^3)$ | 3 |
| rank + nullity | $2 + 1 = 3$ ✓ |

## Proof Sketch

Choose a basis $\{k_1, \ldots, k_m\}$ for $\ker f$ and extend it to a basis $\{k_1, \ldots, k_m, v_1, \ldots, v_r\}$ of $V$. One shows that $\{f(v_1), \ldots, f(v_r)\}$ is a basis for $\operatorname{im} f$: they span $\operatorname{im} f$ (since $f(k_i) = 0$) and are linearly independent (any linear dependence $\sum c_j f(v_j) = 0$ implies $\sum c_j v_j \in \ker f$, forcing all $c_j = 0$). Hence $\dim(\operatorname{im} f) = r$ and $\dim V = m + r$.

## Connections

- [[Cayley–Hamilton Theorem]] — the minimal polynomial degree and kernel dimensions are intertwined.
- [[Spectral Theorem]] — the nullity of $A - \lambda I$ is the geometric multiplicity of eigenvalue $\lambda$.
- [[Cramer's Rule]] — Cramer's rule applies precisely when $\ker A = \{0\}$, i.e., $\operatorname{nullity}(A) = 0$.
- [[Determinant Multiplicativity]] — $\det(A) \neq 0$ if and only if $\operatorname{nullity}(A) = 0$ if and only if $\operatorname{rank}(A) = n$.

## Lean4 Proof

```lean4
/-- Rank–nullity theorem: for a linear map f : V → W between finite-dimensional
    K-vector spaces, the finrank of the image plus the finrank of the kernel
    equals the finrank of the domain. -/
theorem rank_nullity {K V W : Type*}
    [DivisionRing K] [AddCommGroup V] [Module K V]
    [AddCommGroup W] [Module K W]
    [FiniteDimensional K V] (f : V →ₗ[K] W) :
    Module.finrank K (LinearMap.range f) +
    Module.finrank K (LinearMap.ker f) =
    Module.finrank K V :=
  LinearMap.finrank_range_add_finrank_ker f
```
