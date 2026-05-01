+++
title = "Hausdorff Implies T1"
description = "Every Hausdorff (T2) space is a T1 space, meaning singletons are closed sets"
weight = 110
tags = ["lean4-proof", "topology", "visualization"]
latex = "T_2 \\Rightarrow T_1 \\colon \\{x\\} \\text{ closed for all } x"
prerequisites = ["subspace-topology"]
lean4_status = "complete"
+++

## Statement

A topological space $X$ is **Hausdorff** (or $T_2$) if for every two distinct points $x \ne y$ there exist disjoint open sets $U \ni x$ and $V \ni y$:
$$\forall x \ne y,\; \exists U, V \text{ open},\; x \in U,\; y \in V,\; U \cap V = \emptyset.$$

A space is **T1** if every singleton $\{x\}$ is a closed set:
$$\forall x \in X,\; \{x\} \text{ is closed.}$$

**Theorem:** Every Hausdorff space is T1:
$$T_2 \Rightarrow T_1.$$

The converse fails: the **cofinite topology** on an infinite set is T1 (singletons are closed since their complements are cofinite, hence open) but not Hausdorff (any two nonempty open sets intersect in an infinite set).

## Visualization

**Hausdorff separation:** two points $x$ and $y$ separated by disjoint open neighbourhoods.

```
  X:
     ┌──────────────────────────────┐
     │   ┌─────┐         ┌─────┐   │
     │   │  U  │         │  V  │   │
     │   │  x  │         │  y  │   │
     │   └─────┘         └─────┘   │
     │       U ∩ V = ∅              │
     └──────────────────────────────┘

  T1 conclusion: {y} is closed.
  Proof: X \ {y} = ⋃_{x ≠ y} U_x
         where U_x is open and misses y.
         This union is open, so {y} is closed.
```

**Table: separation axioms**

| Axiom | Condition | Implication |
|---|---|---|
| $T_0$ (Kolmogorov) | for $x \ne y$, some open distinguishes them | weakest |
| $T_1$ (Fréchet) | singletons are closed | $T_2 \Rightarrow T_1 \Rightarrow T_0$ |
| $T_2$ (Hausdorff) | distinct points have disjoint open nbhds | intermediate |
| $T_3$ (Regular) | point and closed set have disjoint open nbhds | $T_3 \Rightarrow T_2$ |
| $T_4$ (Normal) | disjoint closed sets have disjoint open nbhds | strongest common |

## Proof Sketch

1. Let $X$ be Hausdorff and fix $y \in X$. We show $\{y\}$ is closed by showing $X \setminus \{y\}$ is open.

2. Take any $x \in X \setminus \{y\}$, so $x \ne y$. By the Hausdorff condition, there exist open sets $U_x \ni x$ and $V_x \ni y$ with $U_x \cap V_x = \emptyset$.

3. In particular, $y \notin U_x$ (since $U_x \cap V_x = \emptyset$ and $y \in V_x$), so $U_x \subseteq X \setminus \{y\}$.

4. Then $X \setminus \{y\} = \bigcup_{x \ne y} U_x$ is a union of open sets, hence open.

5. Therefore $\{y\}$ is closed. Since $y$ was arbitrary, $X$ is T1.

## Connections

- **[[Urysohn's Lemma]]** — Urysohn's lemma requires normality (T4), which implies Hausdorff (T2), which implies T1; the lemma produces continuous functions separating closed sets, generalising the separation axiom hierarchy.
- **[[Heine–Borel Theorem]]** — $\mathbb{R}^n$ is Hausdorff (metric spaces are always Hausdorff), so the Heine–Borel characterisation of compact sets as closed and bounded takes place in a T1 space where singletons are automatically closed.
- **[[Compact Subset of Hausdorff is Closed]]** — the T1 property (singletons closed) is necessary but not sufficient for compact sets to be closed; Hausdorff (T2) is the right axiom, as proven in the companion note.
- **[[Bolzano–Weierstrass Theorem]]** — convergent sequences in Hausdorff spaces have unique limits (the Hausdorff condition separates any two distinct limit candidates), a key feature that T1 alone does not guarantee.

## Lean4 Proof

```lean4
import Mathlib.Topology.Separation.Hausdorff

/-- Every Hausdorff space is T1: singletons are closed.
    Mathlib provides this as an instance with priority 100. -/
theorem hausdorff_implies_t1
    (X : Type*) [TopologicalSpace X] [T2Space X] : T1Space X :=
  inferInstance
```
