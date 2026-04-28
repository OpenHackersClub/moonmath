+++
title = "Fundamental Theorem of Galois Theory"
description = "Bijection between intermediate fields and subgroups of the Galois group"
weight = 20
difficulty = "advanced"
tags = ["galois-theory", "visualization"]
latex = "\\{\\text{intermediate fields}\\} \\longleftrightarrow \\{\\text{subgroups of } \\text{Gal}(K/F)\\}"
prerequisites = ["quintic"]
lean4_status = "sorry"
+++

## Statement

Let $K/F$ be a finite Galois extension with Galois group $G = \text{Gal}(K/F)$. There is an inclusion-reversing bijection:

$$\{\text{intermediate fields } F \subseteq E \subseteq K\} \longleftrightarrow \{\text{subgroups } H \leq G\}$$

given by $E \mapsto \text{Gal}(K/E)$ and $H \mapsto K^H$ (the fixed field of $H$).

Moreover:
- $[K:E] = |H|$ and $[E:F] = [G:H]$
- $E/F$ is normal (Galois) if and only if $H$ is a normal subgroup of $G$
- When $E/F$ is Galois, $\text{Gal}(E/F) \cong G/H$

## Example: Splitting Field of $x^4 - 2$

The splitting field of $x^4 - 2$ over $\mathbb{Q}$ is $\mathbb{Q}(\sqrt[4]{2}, i)$. The Galois group is the dihedral group $D_4$ of order 8. The lattice of subgroups of $D_4$ corresponds precisely to the lattice of intermediate fields.

## Connections

This theorem underlies the proof of the [[Impossibility of the Quintic Formula]]. It also explains the result on [[Constructible Numbers]] (regular $n$-gon constructibility reduces to checking whether the Galois group has a composition series with all factors of order 2).

## Lean4 Proof

```lean4
/-- The Galois correspondence: intermediate fields ↔ subgroups of Gal(K/F).
    For a finite Galois extension K/F, the map E ↦ Gal(K/E) is an
    inclusion-reversing bijection. -/
theorem galois_correspondence
    {F K : Type*} [Field F] [Field K] [Algebra F K]
    [FiniteDimensional F K] [IsGalois F K] :
    Function.Bijective
      (fun E : IntermediateField F K => E.fixingSubgroup) := by
  sorry -- Artin's theorem + dimension counting

/-- Normal intermediate extensions correspond to normal subgroups. -/
theorem normal_iff_normal_subgroup
    {F K : Type*} [Field F] [Field K] [Algebra F K]
    [FiniteDimensional F K] [IsGalois F K]
    (E : IntermediateField F K) :
    Normal F E ↔ E.fixingSubgroup.Normal := by
  sorry -- follows from the Galois correspondence
```
