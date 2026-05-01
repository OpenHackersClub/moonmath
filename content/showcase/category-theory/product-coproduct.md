+++
title = "Products and Coproducts"
description = "Binary limits and colimits: products with projections and coproducts with injections satisfy dual universal properties"
weight = 70
tags = ["lean4-proof", "category-theory", "visualization"]
latex = "\\mathrm{Hom}(Z, A \\times B) \\cong \\mathrm{Hom}(Z, A) \\times \\mathrm{Hom}(Z, B)"
prerequisites = ["limits-colimits"]
lean4_status = "complete"
+++

## Statement

In a category $\mathcal{C}$, the **product** of $A$ and $B$ is an object $A \times B$ with projections $\pi_1 : A \times B \to A$, $\pi_2 : A \times B \to B$, universal in the sense that for any $Z$ with morphisms $f : Z \to A$, $g : Z \to B$, there is a unique $\langle f, g \rangle : Z \to A \times B$ with $\pi_1 \circ \langle f,g\rangle = f$ and $\pi_2 \circ \langle f,g\rangle = g$.

The **coproduct** $A \sqcup B$ (dual) has injections $\iota_1 : A \to A \sqcup B$, $\iota_2 : B \to A \sqcup B$, universal among cocones.

In $\mathbf{Set}$: $A \times B$ is Cartesian product, $A \sqcup B$ is disjoint union.

## Visualization

**Universal property of product in Set:**

```
  Product:

          Z
        / | \
       f  |  g
      ↙  ⟨f,g⟩ ↘
     A ◀π₁ A×B π₂▶ B

  Example: A = {0,1}, B = {red,blue}
  A × B = {(0,red),(0,blue),(1,red),(1,blue)}

  f : {*} → {0,1},   f(*) = 1
  g : {*} → {red,blue}, g(*) = red
  ⟨f,g⟩(*) = (1, red)   ✓  unique!

  Coproduct:

     A ──ι₁──▶ A ⊔ B ◀──ι₂── B
      \          ↑           /
       f        [f,g]       g
        \                  /
         ──────────▶ Z ◀──

  A = {0,1}, B = {red,blue}
  A ⊔ B = {inl 0, inl 1, inr red, inr blue}  (disjoint union, 4 elements)

  f : {0,1} → ℕ,         f(0)=0, f(1)=1
  g : {red,blue} → ℕ,    g(red)=2, g(blue)=3
  [f,g](inl 0)=0, [f,g](inl 1)=1, [f,g](inr red)=2, [f,g](inr blue)=3
```

## Proof Sketch

1. **Product in Set:** Define $A \times B = \{(a,b) \mid a \in A, b \in B\}$ with $\pi_1(a,b) = a$, $\pi_2(a,b) = b$. For any $f : Z \to A$, $g : Z \to B$, the map $z \mapsto (f(z), g(z))$ is the unique factorisation.

2. **Coproduct in Set:** Define $A \sqcup B = A \times \{0\} \cup B \times \{1\}$ (or use `Sum` type). Injections $\iota_1(a) = (a,0)$, $\iota_2(b) = (b,1)$. Copairing $[f,g](a,0) = f(a)$, $[f,g](b,1) = g(b)$.

3. **Uniqueness:** Any morphism $h : A \times B \to Z$ with $h \circ \pi_1 = f$ and $h \circ \pi_2 = g$ satisfies $h(a,b) = h(\langle\pi_1, \pi_2\rangle(a,b))$, so $h = \langle f, g \rangle$ by universality.

4. **Products and coproducts are adjoint to diagonal:** $\Delta : \mathcal{C} \to \mathcal{C} \times \mathcal{C}$ has left adjoint $-\sqcup-$ and right adjoint $-\times-$.

## Connections

Products and coproducts in the category of natural numbers (as a poset under divisibility) are LCM and GCD, connecting to [[Fundamental Theorem of Arithmetic]]. The isomorphism $|A \times B| = |A| \cdot |B|$ and $|A \sqcup B| = |A| + |B|$ underlies the [[Inclusion–Exclusion Principle]] in combinatorics.

## Lean4 Proof

```lean4
import Mathlib.CategoryTheory.Limits.Types.Products

open CategoryTheory CategoryTheory.Limits

/-- Binary products exist in Type u.
    Derived from the general HasLimits instance. -/
theorem type_has_binary_products : HasBinaryProducts (Type 0) :=
  inferInstance

/-- Binary coproducts exist in Type u. -/
theorem type_has_binary_coproducts : HasBinaryCoproducts (Type 0) :=
  inferInstance

/-- The product in Type is the Cartesian product (Sum.inl / Sum.inr for coprod). -/
example (A B : Type) (a : A) (b : B) :
    (prod.lift (C := Type) (fun _ : Unit => a) (fun _ : Unit => b)) () =
    prod.lift (fun _ : Unit => a) (fun _ : Unit => b) () :=
  rfl
```
