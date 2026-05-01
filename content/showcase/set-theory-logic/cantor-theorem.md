+++
title = "Cantor's Theorem"
description = "Every set is strictly smaller than its power set: |X| < |2^X| for any set X."
weight = 136
tags = ["lean4-proof", "set-theory-logic", "visualization"]
latex = "|X| < |2^X|"
prerequisites = []
lean4_status = "complete"
+++

## Statement

For any set $X$, the cardinality of $X$ is strictly less than the cardinality of its power set $2^X$:

$$|X| < |2^X|$$

In cardinal arithmetic: there is no surjection from $X$ onto the set of all subsets of $X$.

## Visualization

The diagonal argument for $X = \mathbb{N}$. Suppose $f : \mathbb{N} \to 2^{\mathbb{N}}$ is any function. Build a diagonal set $D = \{n \in \mathbb{N} \mid n \notin f(n)\}$.

```
n    f(n)          n ∈ f(n)?   D contains n?
---  -----------   ---------   -------------
0    {0, 2, 4, …}   yes         no
1    {3, 5, 7, …}   no          YES
2    {2, 4, 6, …}   yes         no
3    {0, 1, 3, 5}   yes         no
4    {1, 4, 9, …}   yes         no
5    {0, 2, 6, …}   no          YES
…
```

$D = \{1, 5, \ldots\}$. For every $n$, $D \neq f(n)$ because they differ at position $n$. So $f$ is not surjective. Since $f$ was arbitrary, no surjection exists.

## Proof Sketch

1. Suppose for contradiction that $f : X \to 2^X$ is surjective.
2. Define the diagonal set $D = \{x \in X \mid x \notin f(x)\}$.
3. Since $f$ is surjective, there exists $d \in X$ with $f(d) = D$.
4. Ask: is $d \in D$? If yes, then $d \notin f(d) = D$, contradiction. If no, then $d \in f(d) = D$, contradiction.
5. Therefore no such surjection can exist, so $|X| < |2^X|$.

## Connections

Cantor's Theorem is the key to the [[Halting Problem Undecidable|Halting Problem]] via a related diagonalization, and it implies there is no universal set (Russell's paradox). Compare the diagonal technique with the [[Pigeonhole Principle]], which gives a bound in the finite case.

## Lean4 Proof

```lean4
import Mathlib.SetTheory.Cardinal.Order

/-- Cantor's Theorem: every cardinal is strictly less than 2 to its own power.
    Mathlib: `Cardinal.cantor`. -/
theorem cantor_theorem (a : Cardinal.{u}) : a < 2 ^ a :=
  Cardinal.cantor a
```
