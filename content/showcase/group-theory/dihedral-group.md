+++
title = "Dihedral Group"
description = "The dihedral group Dₙ describes the symmetries of a regular n-gon; it has order 2n"
weight = 210
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "D_n = \\langle r, s \\mid r^n = s^2 = (rs)^2 = 1 \\rangle,\\quad |D_n| = 2n"
prerequisites = ["lagrange-theorem", "group-presentation"]
lean4_status = "complete"
+++

## Statement

The **dihedral group** $D_n$ is the group of symmetries of a regular $n$-gon, consisting of $n$ rotations and $n$ reflections. It has presentation

$$D_n = \langle r, s \mid r^n = 1,\; s^2 = 1,\; (rs)^2 = 1 \rangle,$$

equivalently $srs^{-1} = r^{-1}$ (the reflection reverses rotation direction). The order is $|D_n| = 2n$.

The elements are $\{r^k : 0 \leq k < n\} \cup \{sr^k : 0 \leq k < n\}$, with multiplication rules:

$$r^j \cdot r^k = r^{j+k \bmod n}, \quad r^j \cdot sr^k = sr^{k-j \bmod n}, \quad sr^j \cdot r^k = sr^{j+k \bmod n}, \quad sr^j \cdot sr^k = r^{k-j \bmod n}.$$

## Visualization

$D_4$ (symmetries of the square), $|D_4| = 8$:

```
Elements:   1, r, r², r³  (rotations by 0°, 90°, 180°, 270°)
            s, rs, r²s, r³s  (reflections across 4 axes)

Cayley table (partial):

     ·   |   1    r   r²   r³    s   rs  r²s  r³s
─────────┼────────────────────────────────────────
     1   |   1    r   r²   r³    s   rs  r²s  r³s
     r   |   r   r²   r³    1   rs  r²s  r³s    s
    r²   |  r²   r³    1    r  r²s  r³s    s   rs
    r³   |  r³    1    r   r²  r³s    s   rs  r²s
     s   |   s  r³s  r²s   rs    1   r³   r²    r
    rs   |  rs    s  r³s  r²s    r    1   r³   r²
   r²s   | r²s   rs    s  r³s   r²    r    1   r³
   r³s   | r³s  r²s   rs    s   r³   r²    r    1
```

Key identity: $s \cdot r = r^3 s = r^{-1} s$ — reflecting then rotating equals rotating backward then reflecting.

Subgroup lattice of $D_4$:

```
D_4  (order 8)
├── ⟨r⟩ = Z/4  (order 4, index 2, normal)
├── {1, r², s, r²s} ≅ Z/2×Z/2  (order 4, index 2, normal)
├── {1, r², rs, r³s} ≅ Z/2×Z/2  (order 4, index 2, normal)
├── ⟨r²⟩ = {1, r²}  (order 2, index 4, normal — central)
├── ⟨s⟩, ⟨r²s⟩, ⟨rs⟩, ⟨r³s⟩  (order 2, not normal)
└── {1}
```

## Proof Sketch

1. Label vertices of the $n$-gon $1, \ldots, n$. Rotations $r^k$ send vertex $i$ to $i+k \bmod n$. Reflection $s$ sends $i$ to $-i \bmod n$.
2. The composition $sr \cdot r^{n-1} \cdot s^{-1} = r^{-1}$ confirms $srs = r^{-1}$, so $D_n$ is presented as above.
3. Counting: $n$ distinct rotations (since $r$ has order exactly $n$) and $n$ distinct reflections $sr^k$ (checking $sr^j = sr^k \Rightarrow j = k$ by cancellation), giving $2n$ elements total.
4. Uniqueness: any symmetry of the $n$-gon is determined by where vertex 1 goes (n choices) and the orientation (2 choices).

## Connections

$D_n$ is the simplest non-abelian group for $n \geq 3$ and a prototypical example for [[Lagrange's Theorem]] (8 divides into $\{1, 2, 4, 8\}$-order subgroups). The [[Sylow Theorems]] determine $D_n$'s Sylow subgroups precisely. $D_4$'s non-commutativity ($rs \neq sr$) makes it a standard counterexample in group theory, and its [[Conjugation Action]] has kernel $Z(D_4) = \{1, r^2\}$ for $n = 4$.

## Lean4 Proof

```lean4
import Mathlib.GroupTheory.SpecificGroups.Dihedral

/-- |Dₙ| = 2n.
    Mathlib: `DihedralGroup.card` (for Fintype cardinality, needs [NeZero n])
    in `Mathlib.GroupTheory.SpecificGroups.Dihedral`. -/
theorem dihedral_card (n : ℕ) [NeZero n] :
    Fintype.card (DihedralGroup n) = 2 * n :=
  DihedralGroup.card

/-- The Nat.card version holds without NeZero (returns 0 for n = 0). -/
theorem dihedral_nat_card (n : ℕ) :
    Nat.card (DihedralGroup n) = 2 * n :=
  DihedralGroup.nat_card
```
