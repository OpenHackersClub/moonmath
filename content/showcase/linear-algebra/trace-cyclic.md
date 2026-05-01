+++
title = "Trace Cyclic Property"
description = "The trace of a product of matrices is invariant under cyclic permutations: tr(AB) = tr(BA)."
weight = 180
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "\\operatorname{tr}(AB) = \\operatorname{tr}(BA)"
prerequisites = ["determinant-multiplicativity"]
lean4_status = "complete"
+++

## Statement

For any $m \times n$ matrix $A$ and $n \times m$ matrix $B$ over a commutative ring $R$:

$$\operatorname{tr}(AB) = \operatorname{tr}(BA).$$

More generally, the trace is invariant under **cyclic permutations** of a product:

$$\operatorname{tr}(A_1 A_2 \cdots A_k) = \operatorname{tr}(A_2 \cdots A_k A_1) = \cdots = \operatorname{tr}(A_k A_1 \cdots A_{k-1}).$$

Note that $\operatorname{tr}(ABC) = \operatorname{tr}(BCA) = \operatorname{tr}(CAB)$ in general, but $\operatorname{tr}(ABC) \ne \operatorname{tr}(BAC)$ — only cyclic permutations preserve the trace, not arbitrary ones.

## Visualization

Take $A = \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}$ and $B = \begin{pmatrix} 5 & 6 \\ 7 & 8 \end{pmatrix}$.

Compute $AB$:
$$AB = \begin{pmatrix} 1 \cdot 5 + 2 \cdot 7 & 1 \cdot 6 + 2 \cdot 8 \\ 3 \cdot 5 + 4 \cdot 7 & 3 \cdot 6 + 4 \cdot 8 \end{pmatrix} = \begin{pmatrix} 19 & 22 \\ 43 & 50 \end{pmatrix}.$$

Compute $BA$:
$$BA = \begin{pmatrix} 5 \cdot 1 + 6 \cdot 3 & 5 \cdot 2 + 6 \cdot 4 \\ 7 \cdot 1 + 8 \cdot 3 & 7 \cdot 2 + 8 \cdot 4 \end{pmatrix} = \begin{pmatrix} 23 & 34 \\ 31 & 46 \end{pmatrix}.$$

| Product | Matrix | Trace |
|---------|--------|-------|
| $AB$    | $\begin{pmatrix} 19 & 22 \\ 43 & 50 \end{pmatrix}$ | $19 + 50 = 69$ |
| $BA$    | $\begin{pmatrix} 23 & 34 \\ 31 & 46 \end{pmatrix}$ | $23 + 46 = 69$ |

Both traces equal $69$, confirming $\operatorname{tr}(AB) = \operatorname{tr}(BA)$.

## Proof Sketch

1. **Expand by definition.** $\operatorname{tr}(AB) = \sum_i (AB)_{ii} = \sum_i \sum_j A_{ij} B_{ji}$.
2. **Swap summation.** $\sum_i \sum_j A_{ij} B_{ji} = \sum_j \sum_i B_{ji} A_{ij} = \sum_j (BA)_{jj} = \operatorname{tr}(BA)$.
3. **Commutativity of $R$.** The interchange $A_{ij} B_{ji} = B_{ji} A_{ij}$ uses that $R$ is commutative.
4. **Cyclic extension.** $\operatorname{tr}(ABC) = \operatorname{tr}((AB)C) = \operatorname{tr}(C(AB)) = \operatorname{tr}(CAB)$; apply inductively.

## Connections

- [[Cayley–Hamilton Theorem]] — the proof of Cayley–Hamilton for matrices over a commutative ring uses the trace (and more generally the symmetric functions of eigenvalues) to identify coefficients of the characteristic polynomial
- [[Determinant Multiplicativity]] — both trace and determinant are similarity invariants; $\operatorname{tr}(PAP^{-1}) = \operatorname{tr}(A)$ follows from the cyclic property applied to $P$, $A$, $P^{-1}$
- [[Sylvester Determinant Theorem]] — a determinant analogue: just as $\det(I+AB) = \det(I+BA)$, the cyclic property gives $\operatorname{tr}(AB) = \operatorname{tr}(BA)$ as a first-order version of the same symmetry

## Lean4 Proof

```lean4
import Mathlib.LinearAlgebra.Matrix.Trace

/-- Trace cyclic property: tr(A * B) = tr(B * A).
    Mathlib's direct alias is `Matrix.trace_mul_comm`. -/
theorem trace_cyclic
    {R : Type*} [AddCommMonoid R] [CommMagma R]
    {m n : Type*} [Fintype m] [Fintype n]
    (A : Matrix m n R) (B : Matrix n m R) :
    Matrix.trace (A * B) = Matrix.trace (B * A) :=
  Matrix.trace_mul_comm A B
```
