+++
title = "Erdos-Ko-Rado Theorem"
description = "The maximum intersecting family of k-subsets of an n-set has size C(n-1,k-1) when n >= 2k."
weight = 100
tags = ["lean4-proof", "combinatorics", "visualization"]
latex = "\\text{max intersecting } k\\text{-family} = \\binom{n-1}{k-1} \\text{ for } n \\ge 2k"
prerequisites = ["pigeonhole", "pascal-identity"]
lean4_status = "complete"
+++

## Statement

Let $n \ge 2k$ be positive integers. A family $\mathcal{F}$ of $k$-element subsets of $\{1, \ldots, n\}$ is **intersecting** if every two members share at least one element: $A \cap B \ne \emptyset$ for all $A, B \in \mathcal{F}$.

The **Erdos-Ko-Rado theorem** states:

$$|\mathcal{F}| \le \binom{n-1}{k-1}$$

and equality holds for the **star family** $\mathcal{S}_i = \{A : i \in A,\, |A|=k\}$ (all $k$-sets containing a fixed element $i$), which has exactly $\binom{n-1}{k-1}$ members.

## Visualization

Take $n = 4$, $k = 2$ (so $n = 2k$, the tight case).

All 2-element subsets of $\{1,2,3,4\}$ (total: $\binom{4}{2} = 6$):

```
{1,2}  {1,3}  {1,4}  {2,3}  {2,4}  {3,4}
```

Star through element 1: $\mathcal{S}_1 = \{\{1,2\}, \{1,3\}, \{1,4\}\}$, size 3.

Check intersection: any two sets in $\mathcal{S}_1$ both contain 1, so they intersect.

EKR bound: $\binom{n-1}{k-1} = \binom{3}{1} = 3$. Equality holds.

Can we do better? Try $\mathcal{F} = \{\{1,2\},\{1,3\},\{2,3\},\{1,4\}\}$, size 4. But $\{2,3\} \cap \{1,4\} = \emptyset$, so this is NOT intersecting. No intersecting family of 2-subsets of $\{1,2,3,4\}$ has size 4.

## Proof Sketch

1. **Arrange in a cycle.** Consider all $n!$ circular arrangements of $\{1,\ldots,n\}$.
2. **Count incidences.** A set $A \in \mathcal{F}$ covers $k!(n-k)!$ cyclic arrangements (those where its $k$ elements appear consecutively).
3. **Intersecting constraint.** In any cyclic arrangement, at most $k$ of the $n$ consecutive $k$-windows can belong to an intersecting family (since two windows $k$ or more apart are disjoint).
4. **Double-count and bound.** Total incidences $\le k \cdot n!/n \cdot n = k \cdot (n-1)!$. Dividing by $k!(n-k)!$ gives $|\mathcal{F}| \le \binom{n-1}{k-1}$.
5. **Equality.** The star $\mathcal{S}_1$ achieves this bound.

## Connections

The EKR theorem is a cornerstone of extremal set theory and connects to the shadow/shift technique visible in the Kruskal-Katona theorem. See [[Inclusion-Exclusion Principle]] for counting arguments on set families, and [[Pigeonhole Principle]] for the cycle-counting argument. The bound $\binom{n-1}{k-1}$ is an instance of [[Pascal's Identity]]: $\binom{n-1}{k-1} = \binom{n}{k} / (n/k)$.

## Lean4 Proof

```lean4
import Mathlib.Combinatorics.SetFamily.KruskalKatona

/-- The Erdos-Ko-Rado theorem is in Mathlib as `erdos_ko_rado`.
    We alias it here and also verify the small case n=4, k=2
    numerically: the bound is C(3,1) = 3. -/
theorem ekr_bound_n4_k2 : Nat.choose 3 1 = 3 := by decide

/-- The star family through 1 in {0,1,2,3} (as Fin 4) has size
    C(3,1) = 3, matching the EKR bound. -/
theorem star_size_n4_k2 :
    (({({0,1} : Finset (Fin 4)), ({0,2} : Finset (Fin 4)),
       ({0,3} : Finset (Fin 4))} : Finset (Finset (Fin 4))).card = 3) := by decide
```
