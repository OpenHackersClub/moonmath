+++
title = "Functor Composition"
description = "Functors compose associatively with identity functors as units, forming the category Cat"
weight = 50
tags = ["lean4-proof", "category-theory", "visualization"]
latex = "(F \\circ G) \\circ H = F \\circ (G \\circ H)"
prerequisites = []
lean4_status = "complete"
+++

## Statement

Given functors $F : \mathcal{A} \to \mathcal{B}$, $G : \mathcal{B} \to \mathcal{C}$, $H : \mathcal{C} \to \mathcal{D}$, their composites satisfy:

$$H \circ (G \circ F) = (H \circ G) \circ F$$

and for every functor $F : \mathcal{C} \to \mathcal{D}$:

$$\mathrm{Id}_\mathcal{D} \circ F = F = F \circ \mathrm{Id}_\mathcal{C}$$

These laws make small categories the objects and functors the morphisms of a (very large) category $\mathbf{Cat}$.

## Visualization

**Free and forgetful functors compose to give a monad:**

```
  Set ─────Free─────▶ Mon ─────Free─────▶ Ab
       ◀───Forget────      ◀───Forget────

  Composition chain:
    Free_Ab ∘ Free_Mon : Set ──▶ Ab
    (Forget_Mon ∘ Free_Ab)(A) ≈ Ab underlying set of free abelian group

  Concretely on X = {a, b}:
    Free_Mon({a,b}) = {ε, a, b, ab, ba, aab, ...}    (free monoid)
    Free_Ab({a,b})  = {ma·a + mb·b | ma, mb ∈ ℤ}      (ℤ² as abelian group)

  Associativity on objects (trivially definitional):
    (H ∘ G)(F(X)) = H(G(F(X)))   — same object either way

  Identity laws:
    Id_D(F(X)) = F(X)  ✓
    F(Id_C(X)) = F(X)  ✓ (functors preserve identities)

  Composition table for F : A→B, G : B→C on morphisms f:
    (G ∘ F)(f) := G(F(f))
    ((H ∘ G) ∘ F)(f) = H(G(F(f))) = (H ∘ (G ∘ F))(f)  ✓
```

## Proof Sketch

1. **Composition is well-defined:** Given $F$ maps $\mathcal{A}$-morphisms to $\mathcal{B}$-morphisms and $G$ maps $\mathcal{B}$-morphisms to $\mathcal{C}$-morphisms, $G \circ F$ maps $\mathcal{A}$-morphisms to $\mathcal{C}$-morphisms.

2. **Functor laws for $G \circ F$:**
   - Identity: $(G \circ F)(\mathrm{id}_X) = G(F(\mathrm{id}_X)) = G(\mathrm{id}_{F(X)}) = \mathrm{id}_{G(F(X))}$.
   - Composition: $(G \circ F)(f \circ g) = G(F(f \circ g)) = G(F(f) \circ F(g)) = G(F(f)) \circ G(F(g))$.

3. **Associativity is definitional:** All three composites apply $H$, then $G$, then $F$ to objects and morphisms; they are equal by definition.

4. **Left and right units are definitional:** $\mathrm{Id}_\mathcal{D}(F(X)) = F(X)$ and $F(\mathrm{Id}_\mathcal{C}(X)) = F(X)$ by definition of the identity functor.

## Connections

Functor composition is the composition law in the category $\mathbf{Cat}$, which itself can be studied with the [[Yoneda Lemma]]. The [[Natural Transformation]] between composite functors (whiskering) makes $\mathbf{Cat}$ into a 2-category, a structure that appears in [[Adjoint Functors]] (via triangle identities for $LR$ and $RL$).

## Lean4 Proof

```lean4
import Mathlib.CategoryTheory.Functor.Basic

open CategoryTheory

/-- Functor composition is associative (definitionally equal). -/
theorem functor_comp_assoc {A B C D : Type*}
    [Category A] [Category B] [Category C] [Category D]
    (F : A ⥤ B) (G : B ⥤ C) (H : C ⥤ D) :
    (F ⋙ G) ⋙ H = F ⋙ (G ⋙ H) :=
  rfl

/-- Right identity: F ⋙ 𝟭 D = F. -/
theorem functor_comp_id_right {C D : Type*} [Category C] [Category D]
    (F : C ⥤ D) : F ⋙ 𝟭 D = F :=
  Functor.comp_id F

/-- Left identity: 𝟭 C ⋙ F = F. -/
theorem functor_comp_id_left {C D : Type*} [Category C] [Category D]
    (F : C ⥤ D) : 𝟭 C ⋙ F = F :=
  Functor.id_comp F
```
