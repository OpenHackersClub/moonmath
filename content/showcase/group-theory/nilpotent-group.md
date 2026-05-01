+++
title = "Nilpotent Group"
description = "A group is nilpotent if its lower central series reaches the trivial subgroup in finitely many steps."
weight = 100
tags = ["lean4-proof", "group-theory", "visualization"]
latex = "G = \\gamma_0 \\supset \\gamma_1 \\supset \\cdots \\supset \\gamma_n = \\{e\\}"
prerequisites = ["class-equation", "sylow-theorems"]
lean4_status = "complete"
+++

## Statement

The **lower central series** of $G$ is $\gamma_0(G) = G$, $\gamma_{k+1}(G) = [G, \gamma_k(G)]$. A group $G$ is **nilpotent** if $\gamma_n(G) = \{e\}$ for some $n$, called the **nilpotency class**.

Equivalently, the **upper central series** $1 = Z_0 \leq Z_1 \leq \cdots$ (where $Z_{i+1}/Z_i = Z(G/Z_i)$) reaches $G$ in $n$ steps.

Key fact: every finite $p$-group (group of order $p^k$) is nilpotent.

## Visualization

**Lower central series of $D_4$ vs $S_3$:**

$D_4 = \langle r, s \mid r^4 = s^2 = e,\; srs^{-1} = r^{-1}\rangle$ (dihedral group of order 8):

```
D_4       (order 8, nilpotency class 2)
 |   [D_4, D_4] = {e, r^2}  (center, order 2)
{e,r^2}   (order 2, central)
 |   [{e,r^2}, D_4] = {e}
{e}
```

$S_3 = \langle (12),(123)\rangle$ (symmetric group of order 6):

```
S_3       (order 6, NOT nilpotent)
 |   [S_3, S_3] = A_3 = {e,(123),(132)}
A_3       (order 3)
 |   [A_3, S_3] = A_3  (stabilizes! never reaches {e})
A_3       ...
```

$S_3$ is not nilpotent because $[A_3, S_3] = A_3 \neq \{e\}$. The lower central series stabilizes above $\{e\}$.

**$p$-groups are nilpotent:** For $G = \mathbb{Z}/4 \times \mathbb{Z}/2$ (order $2^3$):

| Step $k$ | $Z_k$ (upper central series) | $|Z_k|$ |
|---|---|---|
| 0 | $\{0\}$ | 1 |
| 1 | $G$ | 8 |

$Z_1 = G$ immediately — $G$ is abelian, so it coincides with its own center, giving nilpotency class 1.

## Proof Sketch

1. **$p$-groups have non-trivial center.** By the class equation, $|G| \equiv |Z(G)| \pmod{p}$. Since $p \mid |G|$, we get $p \mid |Z(G)|$, so $Z(G) \neq \{e\}$.
2. **Induct on $|G|$.** The center $Z(G)$ is non-trivial by step 1. The quotient $G/Z(G)$ is a $p$-group of smaller order, hence nilpotent by induction.
3. **Lift nilpotency.** The preimage of the upper central series of $G/Z(G)$ gives the upper central series of $G$, reaching $G$ in one extra step.
4. **Nilpotent implies solvable.** Every step $G_k/G_{k+1}$ in the upper central series is abelian (it lies in a center), so the abelian quotient chain certifies solvability.

## Connections

Nilpotent groups are the "tamest" non-abelian groups. The class equation driving step 1 is [[Class Equation]]. Nilpotent groups are always solvable — see [[Solvable Group]] for the comparison. Finite $p$-groups appear throughout [[Sylow Theorems]], where Sylow subgroups of nilpotent groups are always normal.

## Lean4 Proof

```lean4
/-- Every finite p-group is nilpotent. Mathlib:
    `IsPGroup.isNilpotent` in GroupTheory.Nilpotent. -/
theorem pGroup_isNilpotent {G : Type*} [Group G] [Finite G] {p : ℕ}
    [Fact p.Prime] (h : IsPGroup p G) : IsNilpotent G :=
  IsPGroup.isNilpotent h

/-- Abelian groups are nilpotent (class 1): their lower central series reaches
    {e} in one step because [G, G] = {e}. Mathlib's instance: -/
instance abelian_nilpotent {G : Type*} [CommGroup G] : IsNilpotent G :=
  CommGroup.isNilpotent

/-- Subgroups of nilpotent groups are nilpotent. -/
example {G : Type*} [Group G] [IsNilpotent G] (H : Subgroup G) :
    IsNilpotent H :=
  Subgroup.isNilpotent H
```
