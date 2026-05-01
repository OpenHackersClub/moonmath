+++
title = "Halting Problem Undecidable"
description = "There is no computable function that decides whether an arbitrary program halts on a given input."
weight = 143
tags = ["lean4-proof", "set-theory-logic", "visualization"]
latex = "\\nexists \\text{ computable } H : (\\text{code}, \\text{input}) \\to \\{0,1\\} \\text{ deciding halting}"
prerequisites = ["cantor-theorem"]
lean4_status = "complete"
+++

## Statement

There is no computable predicate $H$ such that for all programs $p$ and inputs $x$:

$$H(p, x) = 1 \iff \text{the program coded by } p \text{ halts on input } x$$

In Mathlib's formulation: for any fixed input $n$, the predicate $\lambda c,\, (\text{eval}(c, n) \text{ is defined})$ is **not** computable.

## Visualization

The classic diagonalization argument. Suppose $H(p, x)$ decides halting. Define:

```
D(p) = if H(p, p) = "halts"  then  loop forever
       else                         halt immediately

Now ask: does D(D) halt?

Case 1: D(D) halts.
   Then H(D, D) = "halts".
   So by definition of D, D(D) loops forever.  ← Contradiction.

Case 2: D(D) loops.
   Then H(D, D) = "loops".
   So by definition of D, D(D) halts.          ← Contradiction.

Both cases are impossible → H cannot exist. □
```

The argument is structurally identical to [[Cantor's Theorem|Cantor's diagonal]], with programs playing the role of subsets.

## Proof Sketch

The Mathlib proof uses Rice's theorem (of which the halting problem is the canonical special case):

1. Encode programs as natural numbers (Gödel numbering). Let $\text{eval}(c, n)$ be the partial recursive function computed by code $c$ on input $n$.
2. Suppose $\text{Halt}(c, n) = [\text{eval}(c, n) \text{ is defined}]$ is computable.
3. Build a program $D$ that: on input $c$, runs $\text{Halt}(c, c)$; if halts, diverges; if diverges, halts.
4. Evaluating $D$ on itself gives a contradiction in both branches.
5. Therefore $\text{Halt}$ is not computable. $\square$

## Connections

The halting problem is the prototype of **undecidability** in computability theory. Its proof mirrors [[Cantor's Theorem]] (diagonalization). Many other undecidable problems (Post's correspondence, Hilbert's 10th problem, provability in PA) reduce from the halting problem. Compare [[Gödel's Completeness Theorem]]: completeness says every truth has a proof, but the halting problem shows not every truth is *effectively findable*.

## Lean4 Proof

```lean4
import Mathlib.Computability.Halting

open Nat.Partrec.Code

/-- The Halting Problem: for any fixed input n, no computable predicate decides
    whether code c halts on n.
    Mathlib: `halting_problem`. -/
theorem halting_problem_undecidable (n : ℕ) :
    ¬ComputablePred fun c : Code => (eval c n).Dom :=
  halting_problem n
```
