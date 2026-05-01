+++
title = "Conjugation Action"
description = "Every group acts on itself by conjugation; orbits are conjugacy classes"
weight = 190
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "c_g(h) = g h g^{-1},\\quad \\mathrm{conj} : G \\to \\mathrm{Aut}(G)"
prerequisites = ["cayley-theorem", "first-isomorphism-theorem"]
lean4_status = "complete"
+++

## Statement

For a group $G$, **conjugation by $g$** is the map $c_g : G \to G$ defined by

$$c_g(h) = g h g^{-1}.$$

Each $c_g$ is a group automorphism of $G$. The assignment $g \mapsto c_g$ defines a group homomorphism

$$\mathrm{conj} : G \to \mathrm{Aut}(G), \quad \ker(\mathrm{conj}) = Z(G).$$

The **conjugacy class** of $h$ is the orbit $\{g h g^{-1} : g \in G\}$ under this action. Normal subgroups are exactly those that are unions of conjugacy classes.

## Visualization

In $S_3 = \{e, (12), (13), (23), (123), (132)\}$, conjugate elements have the same cycle type:

| Conjugacy class | Elements | Size |
|-----------------|----------|------|
| $\{e\}$ | identity | 1 |
| $\{(12), (13), (23)\}$ | transpositions | 3 |
| $\{(123), (132)\}$ | 3-cycles | 2 |

Verify: $(23)(12)(23)^{-1} = (23)(12)(23) = (13)$. So $(12)$ and $(13)$ are conjugate via $(23)$.

```
Conjugation table (column g, row h, entry g h g⁻¹) in S₃:

h\g   | (12)   | (13)   | (23)
──────┼────────┼────────┼────────
(12)  | (12)   | (13)   | (23)
(13)  | (12)   | (13)   | (23)   ← wait, each conjugate is a transposition
(123) | (132)  | (132)  | (132)
(132) | (123)  | (123)  | (123)
```

The kernel of $\mathrm{conj}$ is $Z(S_3) = \{e\}$ (since no nontrivial element of $S_3$ commutes with all others), so $\mathrm{conj}$ is injective — this gives the embedding $S_3 \hookrightarrow \mathrm{Aut}(S_3)$.

## Proof Sketch

1. $c_g$ is a bijection with inverse $c_{g^{-1}}$ (since $c_g \circ c_{g^{-1}} = \mathrm{id}$).
2. $c_g$ is a homomorphism: $c_g(h_1 h_2) = g h_1 h_2 g^{-1} = (g h_1 g^{-1})(g h_2 g^{-1}) = c_g(h_1) c_g(h_2)$.
3. The map $g \mapsto c_g$ is a homomorphism: $c_{g_1 g_2}(h) = g_1 g_2 h g_2^{-1} g_1^{-1} = c_{g_1}(c_{g_2}(h))$.
4. $g \in \ker(\mathrm{conj}) \Leftrightarrow c_g = \mathrm{id} \Leftrightarrow g h g^{-1} = h$ for all $h \Leftrightarrow g \in Z(G)$.

## Connections

Conjugation is the key ingredient in the [[Sylow Theorems]]: Sylow subgroups are conjugate to each other, and the number of Sylow $p$-subgroups is $|G : N_G(P)|$ — counted by the orbit-stabilizer theorem applied to the conjugation action. The [[Class Equation]] $|G| = |Z(G)| + \sum [G : C_G(x_i)]$ also comes from decomposing $G$ into conjugacy classes. The [[First Isomorphism Theorem]] applies to $\mathrm{conj}$ to give $G/Z(G) \cong \mathrm{Inn}(G)$.

## Lean4 Proof

```lean4
import Mathlib.Algebra.Group.End

/-- Conjugation `g ↦ (h ↦ g * h * g⁻¹)` is a group homomorphism G →* MulAut G.
    Mathlib: `MulAut.conj` in `Mathlib.Algebra.Group.End`. -/
example {G : Type*} [Group G] (g h : G) :
    MulAut.conj g h = g * h * g⁻¹ :=
  MulAut.conj_apply g h

/-- The conjugation map itself is a group homomorphism. -/
noncomputable def conjugation_hom (G : Type*) [Group G] : G →* MulAut G :=
  MulAut.conj
```
