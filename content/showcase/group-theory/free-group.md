+++
title = "Free Group"
description = "The free group on a set X is the universal group with generators X and no relations"
weight = 160
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "\\mathrm{Hom}(F(X),\\, G) \\cong G^X"
prerequisites = ["first-isomorphism-theorem"]
lean4_status = "complete"
+++

## Statement

For any set $X$, the **free group** $F(X)$ is a group together with a function $\iota : X \to F(X)$ satisfying the following universal property: for every group $G$ and every function $f : X \to G$, there exists a unique group homomorphism $\tilde{f} : F(X) \to G$ such that $\tilde{f} \circ \iota = f$.

$$\operatorname{Hom}(F(X),\, G) \;\cong\; G^X \quad (\text{as sets})$$

Elements of $F(X)$ are reduced words — finite sequences of generators $x_i$ and their inverses $x_i^{-1}$, with consecutive cancellations $x_i x_i^{-1}$ and $x_i^{-1} x_i$ removed. The group operation is concatenation followed by reduction.

## Visualization

Take $X = \{a, b\}$, so $F_2 = F(\{a,b\})$ — the free group on two generators.

```
Reduction in F_2:

  a b a b⁻¹ a⁻¹
  ─────────────   (nothing cancels — already reduced)

  a b b⁻¹ a⁻¹
= a (b b⁻¹) a⁻¹
= a   1     a⁻¹
= a a⁻¹
= 1             (fully cancelled)

  a a a⁻¹ b
= (a a a⁻¹) b
= a b         (partial cancellation in the middle)
```

Universal property illustration — to define a homomorphism $\tilde{f} : F_2 \to S_3$, it suffices to pick where $a$ and $b$ go:

| Generator | Image in $S_3$ |
|-----------|----------------|
| $a$ | $(12)$ |
| $b$ | $(23)$ |

Then $\tilde{f}(aba^{-1}) = (12)(23)(12) = (13)$, fully determined by $f$.

## Proof Sketch

1. Construct $F(X)$ as the set of reduced words over $X \cup X^{-1}$, with concatenation-then-reduce as multiplication. Verify this is a group (associativity follows from confluence of the reduction relation).
2. Let $\iota(x)$ be the single-letter word $x$.
3. Given $f : X \to G$, define $\tilde{f}$ on words by $\tilde{f}(x_1^{\varepsilon_1} \cdots x_n^{\varepsilon_n}) = f(x_1)^{\varepsilon_1} \cdots f(x_n)^{\varepsilon_n}$. Reduction steps map to $f(x)f(x)^{-1} = 1$, so $\tilde{f}$ is well-defined.
4. Uniqueness: any homomorphism $g$ with $g \circ \iota = f$ must satisfy $g(x_1^{\varepsilon_1} \cdots x_n^{\varepsilon_n}) = \tilde{f}(x_1^{\varepsilon_1} \cdots x_n^{\varepsilon_n})$ by the homomorphism property.

## Connections

Free groups are the raw material from which all groups arise: every group is a quotient of a free group, making the [[First Isomorphism Theorem]] apply universally. Group presentations (next note) formalize this: $G \cong F(X)/N(R)$ for generators $X$ and relations $R$. Free groups also appear in [[Cayley's Theorem]] — the permutation representation of a group $G$ extends naturally to $F(G)$.

## Lean4 Proof

```lean4
import Mathlib.GroupTheory.FreeGroup.Basic

/-- **Universal property of the free group**: maps from FreeGroup α to G
    are in bijection with functions α → G via `FreeGroup.lift`.
    The lift satisfies `FreeGroup.lift f (FreeGroup.of x) = f x`. -/
theorem free_group_universal_property
    {α G : Type*} [Group G] (f : α → G) (x : α) :
    FreeGroup.lift f (FreeGroup.of x) = f x :=
  FreeGroup.lift_apply_of
```
