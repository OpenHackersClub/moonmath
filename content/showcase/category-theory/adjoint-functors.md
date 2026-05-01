+++
title = "Adjoint Functors"
description = "A pair of functors L ⊣ R with a natural bijection Hom(LX, Y) ≅ Hom(X, RY)"
weight = 20
tags = ["lean4-proof", "category-theory", "visualization"]
latex = "\\mathrm{Hom}(LX, Y) \\cong \\mathrm{Hom}(X, RY)"
prerequisites = ["yoneda-lemma"]
lean4_status = "complete"
+++

## Statement

Functors $L : \mathcal{C} \to \mathcal{D}$ and $R : \mathcal{D} \to \mathcal{C}$ are **adjoint** ($L \dashv R$) if there is a natural bijection:

$$\mathrm{Hom}_{\mathcal{D}}(LX, Y) \;\cong\; \mathrm{Hom}_{\mathcal{C}}(X, RY)$$

natural in both $X \in \mathcal{C}$ and $Y \in \mathcal{D}$. Equivalently, there exist natural transformations $\eta : \mathrm{Id}_{\mathcal{C}} \Rightarrow R \circ L$ (unit) and $\varepsilon : L \circ R \Rightarrow \mathrm{Id}_{\mathcal{D}}$ (counit) satisfying the triangle identities:

$$(\varepsilon L) \circ (L \eta) = \mathrm{id}_L \qquad (R \varepsilon) \circ (\eta R) = \mathrm{id}_R$$

## Visualization

**Free monoid $\dashv$ underlying set** in $\mathbf{Mon}$ vs $\mathbf{Set}$:

```
  L = FreeMonoid : Set ─────▶ Mon     (L(S) = S* = finite words over S)
  R = forget     : Mon ─────▶ Set     (R(M) = underlying set of M)

  Natural bijection for S = {a,b}, M = (ℤ, +):
  ┌─────────────────────────────────────────────────────────┐
  │ MonHom(FreeMonoid{a,b},  (ℤ,+)) ≅ SetMap({a,b}, ℤ)   │
  │                                                         │
  │  f ↦ (a↦3, b↦-1)  ←→  a↦3, b↦-1                      │
  │  f(ab) = 3 + (-1) = 2                                   │
  │  f(aab) = 3+3+(-1) = 5                                  │
  │                                                         │
  │  Unit η_S : S ──▶ R(L(S)) = S*:   s ↦ [s]  (singleton) │
  │  Counit ε_M : L(R(M)) = M* ──▶ M:  [m1,m2] ↦ m1·m2   │
  └─────────────────────────────────────────────────────────┘

  Triangle identity check:
    ε_{LS} ∘ L(η_S) : LS ─▶ L(R(L(S))) ─▶ LS
    [w] ──▶ [[w]] ──▶ [w]   ✓  (flatten-of-singleton = identity)
```

## Proof Sketch

1. **From hom-set bijection to unit/counit:** The unit $\eta_X$ is the image of $\mathrm{id}_{LX}$ under the bijection $\mathrm{Hom}(LX, LX) \cong \mathrm{Hom}(X, RLX)$. The counit $\varepsilon_Y$ is the preimage of $\mathrm{id}_{RY}$ under $\mathrm{Hom}(LRY, Y) \cong \mathrm{Hom}(RY, RY)$.

2. **Triangle identities:** The first triangle $(\varepsilon L)(L\eta) = \mathrm{id}_L$ follows because both sides correspond to $\mathrm{id}_{LX}$ under the bijection (by naturality in $Y$). The second is symmetric.

3. **From unit/counit to bijection:** Given $f : LX \to Y$, map to $Rf \circ \eta_X : X \to RY$. Given $g : X \to RY$, map to $\varepsilon_Y \circ Lg : LX \to Y$. The triangle identities ensure these are inverse.

4. **Uniqueness:** An adjoint functor $R$ is unique up to natural isomorphism once $L$ is fixed (and vice versa), by the Yoneda lemma.

## Connections

Adjoint functors generalize [[Galois Theory|Galois connections]] and appear in every corner of mathematics. The hom-set bijection is a manifestation of the [[Yoneda Lemma]]. Limits and colimits are characterized by adjunctions — see [[Limits and Colimits]].

## Lean4 Proof

```lean4
import Mathlib.CategoryTheory.Adjunction.Basic

open CategoryTheory

/-- The hom-set bijection for an adjunction L ⊣ R.
    Mathlib provides this as `Adjunction.homEquiv`.
    Note: `(L.obj X ⟶ Y) ≃ (X ⟶ R.obj Y)` is a Type (Equiv), not a Prop,
    so we use `def` instead of `theorem`. -/
def adjunction_hom_equiv {C D : Type*} [Category C] [Category D]
    {L : C ⥤ D} {R : D ⥤ C} (adj : L ⊣ R) (X : C) (Y : D) :
    (L.obj X ⟶ Y) ≃ (X ⟶ R.obj Y) :=
  adj.homEquiv X Y

/-- Unit of the adjunction: id_C ⟹ R ∘ L. -/
theorem adjunction_unit_naturality {C D : Type*} [Category C] [Category D]
    {L : C ⥤ D} {R : D ⥤ C} (adj : L ⊣ R) (X : C) :
    adj.homEquiv X (L.obj X) (𝟙 (L.obj X)) = adj.unit.app X :=
  adj.homEquiv_id X
```
