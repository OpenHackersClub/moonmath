+++
title = "Natural Transformation"
description = "A family of morphisms α_X : F(X) → G(X) making every naturality square commute"
weight = 40
tags = ["lean4-proof", "category-theory", "visualization"]
latex = "G(f) \\circ \\alpha_X = \\alpha_Y \\circ F(f)"
prerequisites = ["functor-composition"]
lean4_status = "complete"
+++

## Statement

Let $F, G : \mathcal{C} \to \mathcal{D}$ be functors. A **natural transformation** $\alpha : F \Rightarrow G$ is a family of morphisms $\alpha_X : F(X) \to G(X)$ in $\mathcal{D}$, one for each object $X \in \mathcal{C}$, such that for every morphism $f : X \to Y$ in $\mathcal{C}$ the following square commutes:

$$G(f) \circ \alpha_X = \alpha_Y \circ F(f)$$

This is the **naturality condition**: the components $\alpha_X$ interlock coherently with how $F$ and $G$ act on morphisms.

## Visualization

**Identity natural transformation** $\mathrm{id}_F : F \Rightarrow F$ on the power-set functor $F = \mathcal{P} : \mathbf{Set} \to \mathbf{Set}$:

```
  For any f : X → Y and subset S ⊆ X:

        id_{F(X)}
  F(X) ──────────▶ F(X)
    │                │
 F(f)│                │F(f)        F(f)(S) = f[S] = {f(s) | s ∈ S}
    ▼                ▼
  F(Y) ──────────▶ F(Y)
        id_{F(Y)}

  Check: F(f)(id_{F(X)}(S)) = F(f)(S) = id_{F(Y)}(F(f)(S))  ✓
```

**Concrete non-trivial example:** $\alpha : \mathrm{Id} \Rightarrow \mathcal{P}$ on $\mathbf{Set}$, where $\alpha_X(x) = \{x\}$ (singleton embedding):

```
  X = {a, b},  Y = {1, 2, 3},  f : {a,b} → {1,2,3},  f(a)=1, f(b)=3

  α_X : {a,b} ──▶ P({a,b})          α_Y : {1,2,3} ──▶ P({1,2,3})
        a ↦ {a}                             1 ↦ {1}
        b ↦ {b}                             3 ↦ {3}

  Naturality: P(f)(α_X(a)) = P(f)({a}) = {f(a)} = {1} = α_Y(f(a))  ✓
              P(f)(α_X(b)) = {3} = α_Y(3) = α_Y(f(b))              ✓
```

## Proof Sketch

1. **Identity natural transformation:** Define $(\mathrm{id}_F)_X = \mathrm{id}_{F(X)}$. Naturality: $F(f) \circ \mathrm{id}_{F(X)} = F(f) = \mathrm{id}_{F(Y)} \circ F(f)$.

2. **Vertical composition:** Given $\alpha : F \Rightarrow G$ and $\beta : G \Rightarrow H$, define $(\beta \circ \alpha)_X = \beta_X \circ \alpha_X$. Naturality follows by composing the two naturality squares.

3. **Horizontal composition (whiskering):** For $\alpha : F \Rightarrow G$ and a functor $H$, the whiskered $H\alpha : HF \Rightarrow HG$ has components $(H\alpha)_X = H(\alpha_X)$.

4. **Functors and natural transformations form a category:** The functor category $[\mathcal{C}, \mathcal{D}]$ has functors as objects and natural transformations as morphisms.

## Connections

Natural transformations are the morphisms in functor categories and are precisely what the [[Yoneda Lemma]] classifies when one functor is representable. The notion underpins the definition of [[Adjoint Functors]] (the unit and counit are natural transformations) and appears in the [[Fundamental Theorem of Galois Theory]] as the natural isomorphism between Galois groups.

## Lean4 Proof

```lean4
import Mathlib.CategoryTheory.NatTrans

open CategoryTheory

/-- The identity natural transformation exists and its components are identities.
    Mathlib: `NatTrans.id` and `NatTrans.id_app`. -/
theorem nat_trans_id_app {C D : Type*} [Category C] [Category D]
    (F : C ⥤ D) (X : C) :
    (NatTrans.id F).app X = 𝟙 (F.obj X) :=
  rfl

/-- Naturality square: for any α : F ⟹ G and f : X ⟶ Y,
    G.map f ≫ α.app X = α.app Y ≫ F.map f. -/
theorem nat_trans_naturality {C D : Type*} [Category C] [Category D]
    {F G : C ⥤ D} (α : F ⟶ G) {X Y : C} (f : X ⟶ Y) :
    F.map f ≫ α.app Y = α.app X ≫ G.map f :=
  α.naturality f
```
