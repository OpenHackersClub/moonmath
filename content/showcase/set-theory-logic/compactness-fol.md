+++
title = "Compactness Theorem (FOL)"
description = "A first-order theory has a model if and only if every finite subset has a model."
weight = 140
tags = ["lean4-proof", "set-theory-logic", "visualization"]
latex = "T \\text{ satisfiable} \\iff \\forall T_0 \\subseteq_\\text{fin} T,\\ T_0 \\text{ satisfiable}"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Let $L$ be a first-order language and $T$ an $L$-theory (a set of $L$-sentences). Then:

$$T \text{ is satisfiable} \iff \text{every finite } T_0 \subseteq T \text{ is satisfiable}$$

## Visualization

Define $\Phi_n$ = "there exist at least $n$ distinct elements" (for each $n \ge 1$):

$$\Phi_n \;=\; \exists x_1 \cdots \exists x_n \bigwedge_{i \neq j} \neg(x_i = x_j)$$

Consider $T = \{\Phi_1, \Phi_2, \Phi_3, \ldots\}$.

```
Finite subset       Satisfying model
-----------         ---------------
{Φ₁}               Any nonempty set, e.g. {a}
{Φ₁, Φ₂}           Any set with ≥ 2 elements, e.g. {a, b}
{Φ₁, Φ₂, Φ₃}       Any set with ≥ 3 elements, e.g. {a, b, c}
{Φ₁, …, Φ_n}       Any set with ≥ n elements
```

Every finite subset $\{Φ_1, \ldots, \Phi_n\}$ is satisfied by a set of size $n$. By compactness, the full theory $T$ is satisfiable — and any model must be infinite. This gives a purely logical proof that "infinite" is not first-order definable by a finite set of axioms.

## Proof Sketch

The Mathlib proof uses an ultraproduct construction:

1. For each finite $T_0 \subseteq T$, let $M_{T_0}$ be a model of $T_0$.
2. Form the product $\prod_{T_0} M_{T_0}$ and choose a non-principal ultrafilter $\mathcal{U}$ on the directed set of finite subsets.
3. The **ultraproduct** $\prod_{T_0} M_{T_0} / \mathcal{U}$ satisfies every sentence $\phi \in T$: for $\phi \in T$, all large enough $T_0$ contain $\phi$, so almost-all $M_{T_0}$ satisfy $\phi$, so the ultraproduct does by Łoś's theorem.

## Connections

Compactness is one of the two central meta-theorems of first-order logic (the other is [[Gödel's Completeness Theorem]]). Together they entail that syntactic provability and semantic truth coincide. The [[Löwenheim–Skolem Theorem]] is a standard corollary: a theory with an infinite model has models of every infinite cardinality.

## Lean4 Proof

```lean4
import Mathlib.ModelTheory.Satisfiability

open FirstOrder Language Theory

/-- The Compactness Theorem: a first-order theory is satisfiable iff finitely satisfiable.
    Mathlib: `FirstOrder.Language.Theory.isSatisfiable_iff_isFinitelySatisfiable`. -/
theorem fol_compactness {L : Language} {T : L.Theory} :
    T.IsSatisfiable ↔ T.IsFinitelySatisfiable :=
  isSatisfiable_iff_isFinitelySatisfiable
```
