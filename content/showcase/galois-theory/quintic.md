+++
title = "Impossibility of the Quintic Formula"
description = "No general algebraic formula exists for polynomials of degree 5 or higher"
weight = 10
tags = ["lean4-proof", "galois-theory", "visualization"]
latex = "S_5 \\text{ is not solvable} \\implies \\text{no radical formula for degree } \\geq 5"
prerequisites = []
lean4_status = "sorry"
+++

## Statement

There is no general formula using radicals (addition, subtraction, multiplication, division, and $n$-th roots) that solves all polynomial equations of degree $\geq 5$.

## The Chain of Ideas

1. **Field extensions:** Given a polynomial $f(x)$ over $\mathbb{Q}$, adjoin its roots to get the splitting field $K$.
2. **Galois group:** The group $\text{Gal}(K/\mathbb{Q})$ permutes the roots of $f$. The [[Fundamental Theorem of Galois Theory]] turns subgroups of this group into intermediate fields.
3. **Solvable groups:** A polynomial is solvable by radicals if and only if its Galois group is a solvable group.
4. **$S_5$ is not solvable:** The symmetric group $S_5$ contains $A_5$, which is simple and non-abelian.
5. **Conclusion:** The general quintic has Galois group $S_5$, hence is not solvable by radicals.

## Concrete Example

The polynomial $f(x) = x^5 - 4x + 2$ has Galois group $S_5$ over $\mathbb{Q}$ — it is irreducible by Eisenstein's criterion at $p = 2$, and one can verify that the discriminant is not a perfect square.

## Connections

This result is a direct consequence of the [[Fundamental Theorem of Galois Theory]]. The question of [[Constructible Numbers]] is also resolved using similar group-theoretic techniques.

## Lean4 Proof

```lean4
/-- The alternating group A_5 is simple (has no proper normal subgroups). -/
theorem A5_simple : IsSimpleGroup (Equiv.Perm (Fin 5)).alternatingGroup := by
  sorry -- A_5 has order 60, check all possible normal subgroup orders

/-- S_5 is not solvable because it contains the simple non-abelian A_5. -/
theorem S5_not_solvable : ¬ IsSolvable (Equiv.Perm (Fin 5)) := by
  sorry -- A_5 ◁ S_5 is simple and non-abelian, blocking any composition series
         -- with abelian factors

/-- The general quintic is not solvable by radicals. -/
theorem quintic_unsolvable :
    ∃ f : Polynomial ℚ, f.natDegree = 5 ∧ ¬ IsSolvableByRad f := by
  sorry -- x^5 - 4x + 2 has Galois group S_5
```
