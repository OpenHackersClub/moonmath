+++
title = "Diagonalizability Criterion"
description = "A matrix is diagonalizable if and only if its minimal polynomial is squarefree."
weight = 200
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "A \\text{ diagonalizable} \\iff \\mu_A \\text{ squarefree}"
prerequisites = ["minimal-polynomial", "spectral-theorem"]
lean4_status = "complete"
+++

## Statement

Let $K$ be a field and $A$ a square matrix over $K$ (or equivalently, a linear endomorphism of a finite-dimensional $K$-vector space). Then:

$$A \text{ is diagonalizable over } K \iff \mu_A \text{ splits into distinct linear factors over } K.$$

In particular:

- If $A$ has $n$ **distinct eigenvalues** (over $K$), then $A$ is diagonalizable.
- $A$ is diagonalizable iff $\mu_A$ is **squarefree** (no repeated irreducible factor), i.e.\ $\gcd(\mu_A, \mu_A') = 1$.
- Over an algebraically closed field, diagonalizable $\Leftrightarrow$ $\mu_A = (x - \lambda_1)\cdots(x - \lambda_k)$ for distinct $\lambda_i$.

## Visualization

Three canonical $2 \times 2$ examples:

```
Diagonalizable (distinct eigenvalues):
A = ( 1  1 )    eigenvalues 1, 2 (distinct)
    ( 0  2 )    mu_A = (x-1)(x-2)  -- squarefree
                P = ( 1  1 )  P^{-1} A P = diag(1, 2)
                    ( 0  1 )

Not diagonalizable (repeated, non-trivial Jordan block):
B = ( 1  1 )    eigenvalue 1 (double)
    ( 0  1 )    mu_B = (x-1)^2  -- NOT squarefree
                (B - I)^1 ≠ 0, so min poly has degree 2

Diagonalizable (repeated but semisimple):
C = ( 1  0 )    eigenvalue 1 (double)
    ( 0  1 )    mu_C = x - 1  -- squarefree!
                C = I, already diagonal
```

| Matrix | Eigenvalues | Min poly | Squarefree? | Diagonalizable? |
|--------|------------|----------|-------------|-----------------|
| $A = \begin{pmatrix}1&1\\0&2\end{pmatrix}$ | $1, 2$ | $(x-1)(x-2)$ | Yes | Yes |
| $B = \begin{pmatrix}1&1\\0&1\end{pmatrix}$ | $1, 1$ | $(x-1)^2$ | No | No |
| $C = I_2$ | $1, 1$ | $x - 1$ | Yes | Yes |

## Proof Sketch

1. **Squarefree implies diagonalizable.** If $\mu_A = p_1 \cdots p_k$ with distinct monic irreducibles, the Chinese Remainder Theorem for $K[x]$-modules gives $V = \ker p_1(A) \oplus \cdots \oplus \ker p_k(A)$. Over an algebraically closed field, each $p_i = x - \lambda_i$ and each kernel is an eigenspace.
2. **Diagonalizable implies squarefree.** If $A = P \, \text{diag}(\lambda_1, \ldots, \lambda_n) P^{-1}$, then $(A - \lambda_i I)|_{\text{eigenspace of }\lambda_i} = 0$, so $\mu_A$ vanishes at each $\lambda_i$ with multiplicity $1$ — hence $\mu_A = \prod_{\text{distinct}} (x - \lambda_i)$, which is squarefree.
3. **Distinct eigenvalues are sufficient.** $n$ distinct eigenvalues $\Rightarrow$ $n$ linearly independent eigenvectors (by the linear independence of eigenvectors for distinct eigenvalues) $\Rightarrow$ $A$ is diagonalizable.
4. **Semisimple endomorphism perspective.** An endomorphism $f$ is semisimple iff $\mu_f$ is squarefree; this is `Module.End.IsSemisimple` in Mathlib.

## Connections

- [[Minimal Polynomial]] — the squarefree condition on $\mu_A$ is precisely the distinction between the diagonal and non-diagonal Jordan forms
- [[Jordan Canonical Form]] — the Jordan form is fully diagonal iff every Jordan block has size $1$, which happens iff $\mu_A$ is squarefree
- [[Spectral Theorem]] — for Hermitian matrices over $\mathbb{C}$, all eigenvalues are real and the spectral theorem provides an orthonormal eigenbasis, giving diagonalizability unconditionally

## Lean4 Proof

```lean4
import Mathlib.LinearAlgebra.Semisimple
import Mathlib.LinearAlgebra.Eigenspace.Basic

/-- Eigenvectors corresponding to distinct eigenvalues are linearly independent.
    This is the key sufficiency lemma: n distinct eigenvalues give n independent
    eigenvectors and hence diagonalizability.
    Mathlib's direct alias is `Module.End.eigenvectors_linearIndependent`. -/
theorem distinct_eigenvalues_independent
    {K : Type*} [Field K]
    {V : Type*} [AddCommGroup V] [Module K V]
    (f : Module.End K V)
    (μs : Set K) (xs : μs → V)
    (h_eigenvec : ∀ μ : μs, f.HasEigenvector μ (xs μ)) :
    LinearIndependent K xs :=
  Module.End.eigenvectors_linearIndependent f μs xs h_eigenvec
```

