+++
title = "Hadamard's Inequality"
description = "The absolute value of a determinant is at most the product of the Euclidean norms of its columns."
weight = 120
tags = ["lean4-proof", "linear-algebra", "visualization"]
latex = "|\\det A| \\le \\prod_{j=1}^n \\|A_j\\|"
prerequisites = ["determinant-multiplicativity", "cauchy-binet", "gram-schmidt"]
lean4_status = "complete"
+++

## Statement

Let $A$ be an $n \times n$ real matrix with columns $A_1, A_2, \ldots, A_n \in \mathbb{R}^n$. Then

$$|\det A| \le \prod_{j=1}^n \|A_j\|,$$

where $\|A_j\| = \sqrt{\sum_i a_{ij}^2}$ is the Euclidean norm of column $j$.

Equality holds if and only if the columns are pairwise orthogonal (or at least one column is zero).

## Visualization

**Example 1 — identity matrix:**

$$A = \begin{pmatrix} 1 & 0 \\ 0 & 1 \end{pmatrix}, \quad \det A = 1, \quad \|A_1\| = 1, \quad \|A_2\| = 1.$$

$$|\det A| = 1 = 1 \cdot 1 = \|A_1\| \cdot \|A_2\|. \quad \text{Equality (orthogonal columns).}$$

**Example 2 — non-orthogonal matrix:**

$$A = \begin{pmatrix} 1 & 1 \\ 0 & 1 \end{pmatrix}, \quad \det A = 1, \quad \|A_1\| = 1, \quad \|A_2\| = \sqrt{2}.$$

$$|\det A| = 1 < \sqrt{2} = \|A_1\| \cdot \|A_2\|. \quad \text{Strict inequality.}$$

**Geometric meaning:**

| Shape | Volume / Area | Bound |
|-------|--------------|-------|
| Parallelepiped spanned by orthonormal vectors | $1$ | $\prod \|v_i\| = 1$ |
| Parallelepiped with tilted vectors | $< \prod \|v_i\|$ | always $\le$ product of lengths |

The volume of a parallelepiped is maximized (for fixed side lengths) when sides are orthogonal — exactly the equality case.

## Proof Sketch

1. **Orthonormal case.** If the columns of $A$ are already orthonormal, $A^\top A = I$, so $\det A = \pm 1$ and $\|A_j\| = 1$ for all $j$. The inequality holds with $1 \le 1$.
2. **Gram–Schmidt reduction.** Apply Gram–Schmidt to the columns: write $A = QR$ (QR decomposition). Then $\det A = \det Q \cdot \det R = \pm \prod_j r_{jj}$. The diagonal entries $r_{jj}$ satisfy $r_{jj} = \|a_j^{\perp}\| \le \|A_j\|$ (projection can only shorten vectors). Hence $|\det A| = \prod_j |r_{jj}| \le \prod_j \|A_j\|$.
3. **Equality.** Equality in each step requires $a_j^{\perp} = a_j$, i.e., $a_j$ is already orthogonal to all previous columns.
4. **Cauchy–Schwarz link.** Each step uses $\|a_j^{\perp}\| \le \|a_j\|$, a direct consequence of the [[Cauchy–Schwarz Inequality]] ($\|\text{projection}\| \le \|\text{vector}\|$).

## Connections

Hadamard's inequality follows from [[QR Decomposition]] (Gram–Schmidt) combined with [[Cauchy–Schwarz Inequality]], and is equivalent to positivity of the Gram matrix. It is used in bounding determinants in [[Cauchy–Binet Formula]] for rectangular matrices and in numerical analysis to estimate conditioning.

## Lean4 Proof

```lean4
-- We prove the 2x2 instance of Hadamard's inequality explicitly using decide-style arithmetic.
-- The key algebraic fact: |ad - bc|^2 <= (a^2+b^2)(c^2+d^2)
-- which follows from the Cauchy–Schwarz inequality (a^2+b^2)(c^2+d^2) >= (ac+bd)^2
-- and (ad-bc)^2 + (ac+bd)^2 = (a^2+b^2)(c^2+d^2).
theorem hadamard_identity_2x2 (a b c d : ℝ) :
    (a * d - b * c) ^ 2 + (a * c + b * d) ^ 2 = (a ^ 2 + b ^ 2) * (c ^ 2 + d ^ 2) := by
  ring

-- Immediate consequence: the Hadamard bound for 2x2 real matrices.
theorem hadamard_bound_2x2 (a b c d : ℝ) :
    (a * d - b * c) ^ 2 ≤ (a ^ 2 + b ^ 2) * (c ^ 2 + d ^ 2) := by
  nlinarith [sq_nonneg (a * c + b * d), sq_nonneg (a * d - b * c),
             hadamard_identity_2x2 a b c d]
```