+++
title = "Alternating Group"
description = "The alternating group Aₙ consists of even permutations; it has order n!/2 for n ≥ 2"
weight = 200
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "|A_n| = n!/2"
prerequisites = ["lagrange-theorem", "simple-groups"]
lean4_status = "complete"
+++

## Statement

The **alternating group** $A_n$ is the kernel of the sign homomorphism $\mathrm{sgn} : S_n \to \{+1, -1\}$. Its elements are the **even permutations** — those expressible as a product of an even number of transpositions.

For $n \geq 2$:

$$|A_n| = \frac{n!}{2}$$

For $n \geq 5$, $A_n$ is **simple** (no proper nontrivial normal subgroups), making it one of the infinite families in the classification of finite simple groups.

## Visualization

$A_4$ has order $4!/2 = 12$. Its elements by cycle type:

| Cycle type | Permutations | Count | Even? |
|------------|-------------|-------|-------|
| $()$ | $e$ | 1 | yes |
| $(ab)(cd)$ | $(12)(34), (13)(24), (14)(23)$ | 3 | yes |
| $(abc)$ | $(123),(132),(124),(142),(134),(143),(234),(243)$ | 8 | yes |

Total: $1 + 3 + 8 = 12 = 4!/2$. The subgroup $V_4 = \{e, (12)(34), (13)(24), (14)(23)\}$ is normal in $A_4$, so $A_4$ is NOT simple.

```
A_4 subgroup lattice (sketch):
         A_4  (order 12)
          │
         V_4  (order 4, normal)
       /  |  \
   ⟨(12)(34)⟩ ⟨(13)(24)⟩ ⟨(14)(23)⟩  (order 2)
          │
         {e}
```

For comparison, $|A_5| = 60$, and $A_5$ has conjugacy classes of sizes $1, 15, 20, 12, 12$. No nontrivial union of these (plus $\{e\}$) has size dividing 60 and being a normal subgroup — hence $A_5$ is simple.

## Proof Sketch

1. $\mathrm{sgn} : S_n \to \{\pm 1\}$ is a surjective group homomorphism (for $n \geq 2$, the transposition $(12)$ has sign $-1$).
2. By the [[First Isomorphism Theorem]], $S_n / A_n \cong \{\pm 1\}$, so $[S_n : A_n] = 2$ and $|A_n| = n!/2$.
3. $A_n$ is normal in $S_n$ as the kernel of a homomorphism, and in fact has index 2 (the unique subgroup of index 2 is always normal).
4. Simplicity for $n \geq 5$: any normal subgroup of $A_n$ is a union of conjugacy classes. A combinatorial check shows no proper nontrivial union is closed under conjugation.

## Connections

$A_5 \cong \mathrm{PSL}(2,5) \cong$ the symmetry group of the icosahedron — the smallest nonabelian [[Simple Groups|simple group]]. The simplicity of $A_5$ is the key obstruction in the [[Impossibility of the Quintic Formula]]: $S_5$ contains $A_5$ as a composition factor, and $A_5$ being simple (hence not solvable) prevents any radical solution. The [[Conjugation Action]] on $A_n$ computes conjugacy classes, which determine simplicity.

## Lean4 Proof

```lean4
import Mathlib.GroupTheory.SpecificGroups.Alternating

/-- |Aₙ| = n!/2 for any nontrivial α (here α = Fin n).
    Mathlib: `Equiv.Perm.card_alternatingGroup`
    and `two_mul_card_alternatingGroup` in
    `Mathlib.GroupTheory.SpecificGroups.Alternating`. -/
theorem alternating_card (n : ℕ) [NeZero n] [Nontrivial (Fin n)] :
    2 * Fintype.card (alternatingGroup (Fin n)) = Fintype.card (Equiv.Perm (Fin n)) :=
  two_mul_card_alternatingGroup

/-- A₅ is a simple group. -/
example : IsSimpleGroup (alternatingGroup (Fin 5)) :=
  inferInstance
```
