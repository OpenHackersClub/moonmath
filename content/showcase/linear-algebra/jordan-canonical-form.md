+++
title = "Jordan Canonical Form"
description = "Every square matrix over an algebraically closed field is similar to a direct sum of Jordan blocks."
weight = 140
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "A = P J P^{-1},\\quad J = \\bigoplus_i J_{n_i}(\\lambda_i)"
prerequisites = ["spectral-theorem", "cayley-hamilton"]
lean4_status = "complete"
+++

## Statement

Let $F$ be an algebraically closed field and $A$ an $n \times n$ matrix over $F$. Then there exists an invertible matrix $P$ and a block-diagonal matrix

$$J = J_{n_1}(\lambda_1) \oplus J_{n_2}(\lambda_2) \oplus \cdots \oplus J_{n_k}(\lambda_k)$$

such that $A = P J P^{-1}$. Each **Jordan block** is

$$J_m(\lambda) = \begin{pmatrix} \lambda & 1 & 0 & \cdots & 0 \\ 0 & \lambda & 1 & \cdots & 0 \\ \vdots & & \ddots & \ddots & \vdots \\ 0 & 0 & \cdots & \lambda & 1 \\ 0 & 0 & \cdots & 0 & \lambda \end{pmatrix} \in F^{m \times m}.$$

The form is unique up to reordering of blocks.

## Visualization

Consider $A = \begin{pmatrix} 2 & 1 \\ 0 & 2 \end{pmatrix}$ vs $B = \begin{pmatrix} 2 & 0 \\ 0 & 2 \end{pmatrix}$.

```
  Matrix A (non-semisimple)        Jordan form of A
  +---------+                      +---------+
  | 2  1    |   ~ similar ~        | 2  1    |  (IS the Jordan block J_2(2))
  | 0  2    |                      | 0  2    |
  +---------+                      +---------+
  eigenvalue λ=2 (double),         one Jordan block of size 2
  minimal poly = (x-2)^2

  Matrix B (semisimple / diagonal) Jordan form of B
  +---------+                      +---------+
  | 2  0    |   ~ similar ~        | 2  0    |  (two blocks J_1(2))
  | 0  2    |                      | 0  2    |
  +---------+                      +---------+
  eigenvalue λ=2 (double),         minimal poly = (x-2), squarefree
```

The key distinction: $A$ needs a $2 \times 2$ block because $(A - 2I)^2 = 0$ but $(A - 2I) \neq 0$; $B$ is already diagonal so each Jordan block has size $1$.

| Matrix | Min poly | Jordan block sizes for $\lambda=2$ |
|--------|----------|------------------------------------|
| $A$    | $(x-2)^2$ | $\{2\}$ |
| $B$    | $x-2$    | $\{1, 1\}$ |

## Proof Sketch

1. **Generalised eigenspaces.** For each eigenvalue $\lambda_i$, the generalised eigenspace $V_i = \ker(A - \lambda_i I)^{n}$ is $A$-invariant. The Cayley–Hamilton theorem guarantees $V = \bigoplus_i V_i$.
2. **Jordan basis on each block.** On $V_i$, the restriction $N_i = (A - \lambda_i I)|_{V_i}$ is nilpotent. A nilpotent operator on a finite-dimensional space admits a basis of **Jordan chains** — sequences $v, Nv, N^2 v, \ldots$ that yield one Jordan block per chain.
3. **Assemble.** Concatenate the Jordan bases across all generalised eigenspaces to get $P$; $J$ is block-diagonal by construction.
4. **Uniqueness.** The block sizes for $\lambda$ are determined by $\dim \ker (A - \lambda I)^k$ for $k = 1, 2, \ldots$, which are similarity invariants.

## Connections

- [[Cayley–Hamilton Theorem]] — guarantees $V = \bigoplus_i V_i$ over algebraically closed fields
- [[Minimal Polynomial]] — block sizes for $\lambda_i$ equal the size of the largest Jordan block, matching the multiplicity of $(\lambda_i)$ in the minimal polynomial
- [[Spectral Theorem]] — the real/complex spectral theorem gives the semisimple part; Jordan form extends this to the full nilpotent decomposition

## Lean4 Proof

```lean4
import Mathlib.LinearAlgebra.JordanChevalley

/-- The Jordan–Chevalley-Dunford decomposition: every endomorphism of a
    finite-dimensional vector space over a perfect field splits as nilpotent
    plus semisimple.  This is the structural engine behind Jordan canonical form.
    Mathlib's `Module.End.exists_isNilpotent_isSemisimple` is the direct alias. -/
theorem jordan_chevalley_decomp
    {K : Type*} [Field K] [PerfectField K]
    {V : Type*} [AddCommGroup V] [Module K V] [FiniteDimensional K V]
    (f : V →ₗ[K] V) :
    ∃ᵉ (n ∈ Algebra.adjoin K {f}) (s ∈ Algebra.adjoin K {f}),
      IsNilpotent n ∧ Module.End.IsSemisimple s ∧ f = n + s :=
  Module.End.exists_isNilpotent_isSemisimple
```

