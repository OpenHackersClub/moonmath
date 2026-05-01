+++
title = "Class Equation"
description = "The order of a finite group equals the size of its center plus the sum of sizes of nontrivial conjugacy classes."
weight = 70
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "|G| = |Z(G)| + \\sum_{\\text{non-central}} |\\mathrm{cl}(g)|"
prerequisites = ["lagrange-theorem", "cauchy-theorem-group"]
lean4_status = "complete"
+++

## Statement

Let $G$ be a finite group. Its elements partition into conjugacy classes $\mathrm{cl}(g) = \{xgx^{-1} \mid x \in G\}$. Central elements $z \in Z(G)$ form singleton classes $\{z\}$. Collecting the rest:

$$|G| = |Z(G)| + \sum_{\mathrm{cl}(g) \not\subseteq Z(G)} |\mathrm{cl}(g)|.$$

By the orbit-stabilizer theorem, $|\mathrm{cl}(g)| = [G : C_G(g)]$, so every summand on the right divides $|G|$ — a powerful divisibility constraint.

## Visualization

**The class equation for $S_4$** (cycle type determines conjugacy class):

| Cycle type | Representative | Class size | $[S_4 : C_{S_4}(g)]$ |
|---|---|---|---|
| $1^4$ | $e$ | 1 | $24/24 = 1$ (central) |
| $2^2$ | $(12)(34)$ | 3 | $24/8 = 3$ |
| $2\,1^2$ | $(12)$ | 6 | $24/4 = 6$ |
| $3\,1$ | $(123)$ | 8 | $24/3 = 8$ |
| $4$ | $(1234)$ | 6 | $24/4 = 6$ |

$$|S_4| = 24 = 1 + 3 + 6 + 8 + 6.$$

The center $Z(S_4) = \{e\}$ contributes only the singleton class. Every other class size divides $24$.

## Proof Sketch

1. The group $G$ acts on itself by conjugation: $g \cdot x = gxg^{-1}$. The orbits are exactly the conjugacy classes.
2. By the orbit-stabilizer theorem, $|\mathrm{cl}(g)| = [G : C_G(g)]$, where $C_G(g)$ is the centralizer.
3. An element $g$ has $|\mathrm{cl}(g)| = 1$ if and only if $g \in Z(G)$.
4. Partitioning the orbits into the trivial (central) and nontrivial ones and summing their sizes gives $|G| = |Z(G)| + \sum_{\mathrm{nontrivial}} [G : C_G(g)]$.

## Connections

The class equation is the engine behind [[Cauchy's Theorem (Group)]] (when $p \mid |G|$, the equation mod $p$ forces a non-central element of order $p$). It also underlies the Sylow counting arguments in [[Sylow Theorems]], bounding Sylow subgroup counts via divisibility.

## Lean4 Proof

```lean4
/-- The class equation: |Z(G)| + sum of nontrivial conjugacy class sizes = |G|.
    Mathlib's `Group.nat_card_center_add_sum_card_noncenter_eq_card` gives this directly. -/
theorem class_equation (G : Type*) [Group G] [Finite G] :
    Nat.card (Subgroup.center G) +
    ∑ᶠ x ∈ ConjClasses.noncenter G, Nat.card x.carrier = Nat.card G :=
  Group.nat_card_center_add_sum_card_noncenter_eq_card G
```
